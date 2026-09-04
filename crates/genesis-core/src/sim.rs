//! La boucle de tick, 0.0.1.
//!
//! Ordre fixe des phases (voir BIBLE/03_DATA_MODEL.md). Chaque phase se termine avant la
//! suivante.
//!
//! Parallelisme et determinisme : les phases par entite sans effet de bord partage
//! (regeneration des cases, perception et decision, mouvement, evaluation des morts)
//! tournent en parallele. Rien ne change selon l'ordonnancement des threads car :
//!  - tout tirage RNG est fait avant, en sequence, dans l'ordre des `EntityId`,
//!  - chaque calcul parallele est pur : il lit du partage immuable et n'ecrit que sur
//!    son entite, ou renvoie une valeur,
//!  - toute accumulation de flottants se fait entite par entite, jamais entre entites,
//!  - les phases avec ecriture partagee (repas sur une case, retrait des morts,
//!    reproduction) restent sequentielles, dans l'ordre des `EntityId`.
//! Le test `same_seed_same_config_same_world_frame_by_frame` verrouille tout ca.

use rayon::prelude::*;

use crate::cognition::{BehaviorMode, Memory, Mind, Needs};
use crate::config::SimConfig;
use crate::entity::{Action, Entity, EntityId, Position};
use crate::event::{DeathCause, Event, EventKind, ReplicationFail};
use crate::genome::{Genome, N_TRAITS, SPECIES_TRAITS};
use crate::spatial::SpatialHash;
use crate::world::{Cell, ResourceField, Space, WorldState};

// -- Profilage optionnel : `GENESIS_PROFILE=1` accumule le temps par phase, `profile_dump`
//    l'affiche. Aucun cout quand la variable n'est pas mise (un booleen lu une fois).
mod prof {
    use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
    use std::time::Instant;
    pub static ON: AtomicBool = AtomicBool::new(false);
    static INIT: std::sync::Once = std::sync::Once::new();
    pub static NS: [AtomicU64; 10] = [
        AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
        AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
        AtomicU64::new(0), AtomicU64::new(0),
    ];
    pub const NAMES: [&str; 10] = [
        "p1 regen", "p2/3 decide", "p4 move", "p5 metab", "p5b cells", "p5c cog", "p6 life",
        "p7 repro", "p8b watch", "cap",
    ];
    pub fn enabled() -> bool {
        INIT.call_once(|| {
            if std::env::var("GENESIS_PROFILE").is_ok() {
                ON.store(true, Ordering::Relaxed);
            }
        });
        ON.load(Ordering::Relaxed)
    }
    pub struct Span(pub usize, pub Instant);
    impl Span {
        pub fn start(i: usize) -> Option<Span> {
            if enabled() { Some(Span(i, Instant::now())) } else { None }
        }
    }
    impl Drop for Span {
        fn drop(&mut self) {
            NS[self.0].fetch_add(self.1.elapsed().as_nanos() as u64, Ordering::Relaxed);
        }
    }
    pub fn dump() {
        if !enabled() { return; }
        eprintln!("--- profil (ms cumules) ---");
        for i in 0..10 {
            eprintln!("  {:<14} {:>8.1}", NAMES[i], NS[i].load(Ordering::Relaxed) as f64 / 1e6);
        }
    }
}
pub use prof::dump as profile_dump;

/// Cree un evenement, lui attribue le prochain `seq` (0.0.2, tranche 3b : a la creation,
/// plus a l'ecriture), le pousse, renvoie son `seq`.
fn emit(events: &mut Vec<Event>, ctr: &mut u64, tick: u64, kind: EventKind) -> u64 {
    let seq = *ctr;
    *ctr += 1;
    let mut e = Event::now(tick, kind);
    e.seq = seq;
    events.push(e);
    seq
}

/// Comme `emit`, mais l'evenement cite les `seq` qui l'ont cause.
fn emit_caused(
    events: &mut Vec<Event>,
    ctr: &mut u64,
    tick: u64,
    kind: EventKind,
    causes: Vec<u64>,
) -> u64 {
    let seq = *ctr;
    *ctr += 1;
    let mut e = Event::now(tick, kind).caused_by(causes);
    e.seq = seq;
    events.push(e);
    seq
}

/// Nombre de ticks dans une annee-monde, d'apres la duree d'un tick (`[time]`).
pub fn ticks_per_year(cfg: &SimConfig) -> f64 {
    let secs = cfg.time.tick_duration_seconds.max(1) as f64;
    (365.25 * 86_400.0) / secs
}

fn season_theta(cfg: &SimConfig, t: u64) -> f64 {
    let period = (cfg.season.period_years.max(0.01) as f64 * ticks_per_year(cfg)).max(1.0);
    (t as f64 / period) * std::f64::consts::TAU
}

/// Phase de la saison nourriciere au tick `t`, dans [-1, 1] : `+1` abondance, `-1` disette,
/// `0` aux intersaisons. Sinusoide pure : deterministe, et un rechargement depuis un
/// instantane la retrouve exactement.
pub fn season_phase(cfg: &SimConfig, t: u64) -> f32 {
    if cfg.season.amplitude <= 0.0 {
        return 0.0;
    }
    season_theta(cfg, t).sin() as f32
}

/// Phase de la saison thermique, dans [-1, 1] : `+1` plein ete, `-1` plein hiver. **Decalee
/// d'un quart de cycle** sur la saison nourriciere : le grand froid tombe quand la nourriture
/// est correcte, la disette quand la temperature est moyenne. La population affronte deux
/// stress distincts par annee au lieu d'un seul, ce qui entretient la diversite genetique.
pub fn season_temp_phase(cfg: &SimConfig, t: u64) -> f32 {
    if cfg.season.temp_amplitude_c <= 0.0 {
        return 0.0;
    }
    (season_theta(cfg, t) - std::f64::consts::FRAC_PI_2).sin() as f32
}

/// Multiplicateur applique a la capacite nourriciere des cases par la saison courante. `1.0`
/// quand les saisons sont coupees (`amplitude = 0`), sinon `1 + amplitude * phase`, borne en
/// bas par `regen_floor` pour eviter l'effondrement total deterministe.
pub fn season_factor(cfg: &SimConfig, t: u64) -> f32 {
    if cfg.season.amplitude <= 0.0 {
        return 1.0;
    }
    (1.0 + cfg.season.amplitude * season_phase(cfg, t)).max(cfg.season.regen_floor.max(0.0))
}

/// Fait avancer le monde d'un tick. Renvoie les evenements produits, `seq` deja attribue.
pub fn tick(world: &mut WorldState, cfg: &SimConfig) -> Vec<Event> {
    world.tick += 1;
    let t = world.tick;
    let space = world.space.clone();
    let mut events: Vec<Event> = Vec::new();

    let mut _sp = prof::Span::start(0);
    // -- Phase 1, environnement : regeneration des ressources.
    // Le plafond et la vitesse de chaque case sont proportionnels a sa fertilite (zones
    // riches, zones mortes). La tension de surexploitation decroit et freine la regen :
    // une case trop recoltee met du temps a revenir. Une case au dessus de son plafond
    // (un cadavre vient d'y tomber) n'est pas rabotee, juste plus alimentee.
    // La regeneration ne tourne qu'un tick sur `regen_every`, avec un taux multiplie
    // d'autant : le milieu change lentement, ca evite de balayer 16 000 cases chaque tick.
    let regen_every = cfg.resources.regen_every.max(1) as u64;
    if t % regen_every == 0 {
        // Les saisons : la capacite nourriciere des cases oscille autour de sa base, sinusoide
        // pure du tick. En saison maigre, le plafond ET la vitesse de regeneration baissent :
        // les cases portent moins et reviennent moins vite, une famine synchrone. Une case
        // au-dessus de son plafond descendu n'est pas videe, juste plus alimentee (lag).
        let season = season_factor(cfg, t);
        let max_cell = cfg.resources.max_per_cell * season;
        let regen_rate = cfg.resources.regen_rate * regen_every as f32 * season;
        let strain_decay = cfg.environment.strain_decay.powi(regen_every as i32);
        let ResourceField { cell, strain, fertility } = &mut world.resources;
        cell.par_iter_mut()
            .zip(strain.par_iter_mut())
            .zip(fertility.par_iter())
            .for_each(|((c, s), &f)| {
                let cell_cap = max_cell * f;
                if *s < 1e-4 && *c >= cell_cap {
                    *s = 0.0;
                    return;
                }
                *s *= strain_decay;
                let sc = s.min(1.0);
                let regen = regen_rate * cell_cap * (1.0 - sc);
                let ceil = cell_cap.max(*c);
                *c = (*c + regen).min(ceil);
            });
    }

    // Index spatial des positions en debut de tick (indices dans `world.entities`).
    let index = SpatialHash::build(&world.entities, &space, cfg.cohesion.radius.max(1.0));
    let coh = cfg.cohesion.clone();
    let sim_scale = coh.similarity_scale.max(0.01);
    let repro_threshold = cfg.reproduction.energy_threshold.max(0.01);

    // Voix : relais nerveux (0.0.2, `[voice] nerve_relay`). Un tissu qui compte assez de
    // membres agents etend leur portee de perception de signal. Mesure une fois ici (pas de
    // nom de tissu) : par entite, le carre du multiplicateur de portee applicable, 1.0 sinon.
    let nerve_r2_mult: Vec<f32> = if cfg.voice.nerve_relay {
        let cslot: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let entity_tissue: Vec<Option<u32>> = world
            .entities
            .iter()
            .map(|e| e.cell_id.and_then(|id| cslot.get(&id)).and_then(|&k| world.cells[k].tissue))
            .collect();
        let mut tissue_agents: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for (e, tid) in world.entities.iter().zip(entity_tissue.iter()) {
            if e.mind.is_some() {
                if let Some(tid) = tid {
                    *tissue_agents.entry(*tid).or_insert(0) += 1;
                }
            }
        }
        let min_agents = cfg.voice.nerve_min_agents;
        let mult2 = cfg.voice.nerve_radius_mult.max(1.0).powi(2);
        entity_tissue
            .iter()
            .map(|tid| match tid {
                Some(tid) if tissue_agents.get(tid).copied().unwrap_or(0) >= min_agents => mult2,
                _ => 1.0,
            })
            .collect()
    } else {
        Vec::new()
    };

    drop(_sp.take()); _sp = prof::Span::start(1);
    // -- Phase 2 et 3, perception et decision : chaque entite choisit une cible.
    // La cible de nourriture (balayage large et couteux) n'est recalculee que tous les
    // `REPLAN_TICKS`, decalee par l'id : une entite garde son cap plusieurs ticks. Le
    // calcul, pur, est reparti sur plusieurs threads (le `Vec` se decoupe parfaitement).
    // Les seeds d'errance sont tirees avant, en sequence : le flux RNG ne devie jamais.
    const REPLAN_TICKS: u64 = 8;
    let n = world.entities.len();
    let wander_seeds: Vec<u64> = (0..n).map(|_| world.rng.next_u64()).collect();

    let entities_ref = &world.entities;
    let resources_ref = &world.resources;
    let index_ref = &index;
    let space_ref = &space;
    let coh_ref = &coh;
    // Cognition (0.0.3) : un agent laisse sa memoire episodique tirer sa cible hors des
    // lieux de peril, vers les lieux d'aubaine. Lecture seule sur `e.mind` : sur en parallele.
    let cog_weight = cfg.cognition.mem_weight;
    let cog_radius = cfg.cognition.mem_radius.max(0.5);
    let cog_needs_weight = cfg.cognition.needs_weight;
    let cog_fear_gain = cfg.cognition.fear_gain;
    let cog_social_pull = cfg.cognition.social_pull;
    let cog_heritable = cfg.cognition.heritable_personality;
    let cog_friend_pull = cfg.cognition.friend_pull;
    // Voix tranche 2 : un appel entendu inflechit la cible. On lit les signaux vivants de
    // ce tick (emis aux ticks precedents, la phase Voix de ce tick vient plus bas).
    let signals_ref = &world.signals;
    let voice_r2 = cfg.voice.signal_radius.max(0.1) * cfg.voice.signal_radius.max(0.1);
    let bounty_pull = cfg.voice.bounty_pull.max(0.0);
    let hear_calls = bounty_pull > 0.0
        && signals_ref.iter().any(|s| s.kind == crate::voice::SignalKind::Bounty);
    // (target, colony_support, mode) ; mode = 255 pour une entite sans esprit.
    let plans: Vec<Option<(Position, f32, u8)>> = entities_ref
        .par_iter()
        .enumerate()
        .zip(wander_seeds.par_iter())
        .map(|((my_idx, e), &wander_seed)| {
            let replan = e.target.is_none() || t.wrapping_add(e.id) % REPLAN_TICKS == 0;
            if !replan {
                return None;
            }
            let pos = e.position;
            let traits_a = e.genome.traits.as_array();

            // Support de colonie et centre de masse des parents proches (bloc de cases),
            // sans allocation.
            let (mut cx, mut cy, mut wsum, mut support) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            let r2 = coh_ref.radius * coh_ref.radius;
            index_ref.for_each_neighbor(pos, coh_ref.radius, |nidx| {
                if nidx as usize == my_idx {
                    return;
                }
                let nb = &entities_ref[nidx as usize];
                if pos.dist2(&nb.position) > r2 {
                    return;
                }
                let simi = (1.0
                    - trait_l1(&traits_a, &nb.genome.traits.as_array()) / sim_scale)
                    .clamp(0.0, 1.0);
                if simi <= 0.0 {
                    return;
                }
                cx += nb.position.x * simi;
                cy += nb.position.y * simi;
                wsum += simi;
                support += simi;
            });
            support = support.min(coh_ref.support_cap);

            let radius = 2.0 + e.genome.traits.perception * 6.0;
            let food = forage_target(resources_ref, space_ref, pos, radius);
            let has_food = food.is_some();
            let forage = match food {
                Some(cp) => cp,
                None => {
                    let mut r = crate::rng::Rng::from_seed(wander_seed);
                    Position {
                        x: (pos.x + (r.next_f32() - 0.5) * 12.0)
                            .clamp(0.0, space_ref.width as f32 - 0.001),
                        y: (pos.y + (r.next_f32() - 0.5) * 12.0)
                            .clamp(0.0, space_ref.height as f32 - 0.001),
                    }
                }
            };
            // Comportement de l'agent. `caution` / `curiosity` : traits herites (tranche 5)
            // ou, si `heritable_personality` est faux, formules derivees des tranches 1-4.
            let (target, mode) = match e.mind.as_deref() {
                Some(mind) => {
                    let (caution, curiosity) = if cog_heritable {
                        (
                            0.25 + 0.7 * e.genome.traits.caution,
                            0.3 + 0.7 * e.genome.traits.curiosity,
                        )
                    } else {
                        (
                            0.3 + 0.5 * e.genome.traits.lifespan,
                            0.4 + 0.6 * e.genome.traits.perception,
                        )
                    };
                    // Ami : l'agent le plus familier de valence positive, s'il est trouve.
                    let friend = mind.top_friend().and_then(|fid| {
                        entities_ref
                            .binary_search_by_key(&fid, |x| x.id)
                            .ok()
                            .map(|fi| entities_ref[fi].position)
                    });
                    // Un appel a portee ? Le plus proche (on ignore le sien : meme case et
                    // `born == t` serait un cas limite, mais un appel de ce tick n'existe pas
                    // encore ici, il est emis plus bas).
                    let call = if hear_calls {
                        let r2 = match nerve_r2_mult.get(my_idx) {
                            Some(&m) => voice_r2 * m,
                            None => voice_r2,
                        };
                        let mut best: Option<(f32, Position)> = None;
                        for s in signals_ref.iter() {
                            if s.kind != crate::voice::SignalKind::Bounty {
                                continue;
                            }
                            let d = pos.dist2(&s.pos);
                            if d <= r2 && best.map_or(true, |(bd, _)| d < bd) {
                                best = Some((d, s.pos));
                            }
                        }
                        best.map(|(_, p)| p)
                    } else {
                        None
                    };
                    let ctx = Decide {
                        pos,
                        forage,
                        has_food,
                        call,
                        call_pull: bounty_pull,
                        kin: if wsum > 0.0 {
                            Some(Position { x: cx / wsum, y: cy / wsum })
                        } else {
                            None
                        },
                        friend,
                        friend_pull: cog_friend_pull,
                        energy_frac: e.energy / repro_threshold,
                        caution,
                        curiosity,
                        needs: mind.needs,
                        nw: cog_needs_weight,
                        fear_gain: cog_fear_gain,
                        social_pull: cog_social_pull,
                        mem_weight: cog_weight,
                        mem_radius: cog_radius,
                        sw: space_ref.width as f32,
                        sh: space_ref.height as f32,
                        episodic: &mind.episodic,
                    };
                    let (tgt, m) = blend_target(&ctx);
                    (tgt, m as u8)
                }
                None => (forage, 255),
            };
            Some((target, support, mode))
        })
        .collect();

    // Application : les plans sont alignes sur l'ordre de `entities`, aucune recherche.
    for (e, plan) in world.entities.iter_mut().zip(plans) {
        if let Some((target, support, mode)) = plan {
            e.target = Some(target);
            e.colony_support = support;
            if mode != 255 {
                if let Some(mind) = e.mind.as_deref_mut() {
                    mind.mode = MODE_FROM_CODE[mode as usize];
                }
            }
        }
    }

    drop(_sp.take()); _sp = prof::Span::start(2);
    // -- Phase 4, mouvement. Chaque entite ne touche qu'elle-meme : parallele direct sur
    // le `Vec`, qui se decoupe parfaitement.
    // Le climat : la gravite rencherit le deplacement (un monde lourd favorise les corps
    // lents et economes). `gravity = 1` (Terre) ne change rien.
    let move_cost = cfg.metabolism.move_cost * cfg.planet.gravity.max(0.0);
    let sw = space.width as f32;
    let sh = space.height as f32;
    // Biologie de fond (tranche 8) : un corps use se traine.
    let frail_slow = cfg.biology.frail_slow.clamp(0.0, 1.0);
    world.entities.par_iter_mut().for_each(|e| {
        let Some(tg) = e.target else { return };
        let pos = e.position;
        let max_step =
            (0.3 + e.genome.traits.speed * 1.2) * (frail_slow + (1.0 - frail_slow) * e.health);
        let dx = tg.x - pos.x;
        let dy = tg.y - pos.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let (nx, ny, moved) = if dist <= max_step || dist < 1e-6 {
            (tg.x, tg.y, dist)
        } else {
            (pos.x + dx / dist * max_step, pos.y + dy / dist * max_step, max_step)
        };
        e.position = Position {
            x: nx.clamp(0.0, sw - 0.001),
            y: ny.clamp(0.0, sh - 0.001),
        };
        e.energy -= move_cost * moved;
    });

    drop(_sp.take()); _sp = prof::Span::start(3);
    // -- Phase 5, metabolisme : depense d'energie, repas sur la case.
    // Retenue sur les communs : une entite `cohesion` haute, entouree de parents
    // (`colony_support`), mange un peu moins et fatigue beaucoup moins la case partagee.
    // Membrane (0.0.2, tranche 2b) : un membre de cellule EN DANGER de famine brule moins
    // d'energie de base (`cell_burn_relief` * gravite, metabolisme mutualise). Le repli ne
    // touche que la zone de peril : l'equilibre des temps gras (ou vit la diversite genetique)
    // ne bouge pas, seul le tampon anti-disette compte. L'avantage de survie du pluricellulaire.
    let energy_ceiling = cfg.reproduction.energy_threshold * 2.0;
    let cell_burn_relief = cfg.cells.cell_burn_relief.clamp(0.0, 0.9);
    let support_cap = coh.support_cap.max(0.001);
    // Le climat : un corps loin de sa temperature optimale coute plus cher a habiter. La
    // temperature effective du monde oscille avec la saison thermique ; l'optimum propre a
    // chaque entite depend de son trait `heat_tol` (schema v18). Le surcout est donc calcule
    // par entite, dans la boucle. `temp_metab_slope == 0` : aucun effet.
    let base_burn = cfg.metabolism.base_burn;
    let temp_slope = cfg.planet.temp_metab_slope.max(0.0);
    let eff_temp = cfg.planet.temperature_c
        + cfg.season.temp_amplitude_c * season_temp_phase(cfg, t);
    let heat_span = cfg.planet.heat_tol_span_c.max(0.0);
    let temp_optimal = cfg.planet.temp_optimal_c;
    let eat_rate = cfg.metabolism.eat_rate;
    let strain_per_harvest = cfg.environment.strain_per_harvest;
    // Cognition (0.0.3) : seuils de choc, ecrits pour toutes les entites (graine d'un souvenir).
    let peril_energy = cfg.cognition.peril_frac * repro_threshold;
    let bounty_abs = cfg.cognition.bounty_abs;
    let shock_interval = cfg.cognition.shock_interval;
    // La Voix (0.0.4) : les agents qui prennent un choc de famine ce tick crient. On
    // collecte pendant la boucle (emprunt exclusif de `entities[i]`), on pousse apres.
    let mut new_signals: Vec<(Position, crate::voice::SignalKind)> = Vec::new();
    let bounty_call = cfg.voice.bounty_call;
    // Un agent lance un appel quand il mange bien sur une case franchement riche : « bon coin
    // ici ». Seuil relatif au plafond de la case, pas absolu (un monde mur ne franchit jamais
    // le seuil absolu du choc d'aubaine).
    let call_cell_min = cfg.resources.max_per_cell * cfg.voice.bounty_cell_frac.clamp(0.0, 1.0);
    let call_gap = cfg.voice.signal_ttl.max(1);
    for i in 0..world.entities.len() {
        let (pos, burn, want0, restraint) = {
            let e = &world.entities[i];
            let restraint = (e.genome.traits.cohesion
                * (e.colony_support / support_cap).clamp(0.0, 1.0))
                .clamp(0.0, 1.0);
            // Optimum thermique propre a l'entite : `heat_tol` 0..1 le place entre
            // `optimum - span/2` (froid) et `+ span/2` (chaud). `span = 0` : tout le monde
            // partage l'optimum du monde, `heat_tol` est inerte.
            let organism_optimal = temp_optimal + (e.genome.traits.heat_tol - 0.5) * heat_span;
            let temp_factor = 1.0 + temp_slope * (eff_temp - organism_optimal).abs();
            let mut burn = base_burn * temp_factor * (0.5 + e.genome.traits.metabolism);
            if e.cell_id.is_some() && cell_burn_relief > 0.0 && e.energy < peril_energy {
                // Proportionnel a l'enfoncement dans la zone de peril : nul au seuil, plein a
                // zero. Un membre bien nourri paie plein tarif, un membre qui frole la mort
                // profite du metabolisme mutualise de la membrane (tampon anti-disette).
                let severity = ((peril_energy - e.energy) / peril_energy).clamp(0.0, 1.0);
                burn *= 1.0 - cell_burn_relief * severity;
            }
            (
                e.position,
                burn,
                eat_rate * (0.5 + e.genome.traits.efficiency),
                restraint,
            )
        };
        let idx = world.resources.index(&space, pos);
        let want = want0 * (1.0 - restraint * coh.eat_restraint);
        let cell_before = world.resources.cell[idx];
        let gain = want.min(cell_before).max(0.0);
        world.resources.cell[idx] -= gain;
        world.resources.strain[idx] +=
            gain * strain_per_harvest * (1.0 - restraint * coh.strain_restraint);

        let e = &mut world.entities[i];
        e.energy = (e.energy - burn + gain).min(energy_ceiling);
        e.last_action = if gain > 0.05 { Action::Eat } else { Action::Forage };

        // Choc marquant : famine (peril) ou repas exceptionnel (aubaine). Espace dans le
        // temps pour ne pas enregistrer le meme episode a chaque tick.
        let recent = e
            .last_shock
            .is_some_and(|s| t.saturating_sub(s.tick) < shock_interval);
        if !recent {
            if e.energy < peril_energy {
                e.last_shock = Some(crate::cognition::Shock { tick: t, place: e.position, peril: true });
                // Un agent qui frole la mort crie : il emet une alarme a sa position.
                if e.mind.is_some() {
                    new_signals.push((e.position, crate::voice::SignalKind::Alarm));
                }
            } else if gain > bounty_abs {
                e.last_shock = Some(crate::cognition::Shock { tick: t, place: e.position, peril: false });
            }
        }
        // Voix tranche 2 : un agent qui mange bien sur une case franchement riche lance un
        // appel « bon coin ici ». Rate-limite par `call_born` (au moins un ttl entre deux
        // appels d'un meme agent), independant du choc d'aubaine.
        if bounty_call
            && e.mind.is_some()
            && e.energy >= peril_energy
            && cell_before >= call_cell_min
            && gain > eat_rate * 0.3
            && t.saturating_sub(e.call_born) >= call_gap
        {
            e.call_born = t;
            new_signals.push((e.position, crate::voice::SignalKind::Bounty));
        }
    }

    // -- Voix : on efface les vieux signaux, on ajoute ceux du tick, on borne.
    let voice_ttl = cfg.voice.signal_ttl.max(1);
    world.signals.retain(|s| t.saturating_sub(s.born) < voice_ttl);
    for (pos, kind) in new_signals {
        world.signals.push(crate::voice::Signal { pos, born: t, kind });
    }
    let max_sig = cfg.voice.max_signals.max(1);
    if world.signals.len() > max_sig {
        let drop = world.signals.len() - max_sig;
        world.signals.drain(0..drop);
    }

    // -- Phase 5a, predation (0.0.2, `experiments/012_predation.md`, config seulement,
    //    `[predation] enabled`). Une entite qui a faim et qui a a portee une entite nettement
    //    plus faible la mange : la proie est marquee morte (traitee en phase 6 avec
    //    `DeathCause::Predation`), une part de son energie passe au predateur. Sequentiel,
    //    sans RNG, ordre des id, une prise par predateur et par tick ; decisions et gains
    //    accumules puis appliques.
    let mut predated: Vec<EntityId> = Vec::new();
    let pcfg = &cfg.predation;
    if pcfg.enabled && world.entities.len() >= 2 {
        let reach = pcfg.reach.max(0.1);
        let phash = SpatialHash::build(&world.entities, &space, reach);
        let snap: Vec<(Position, f32)> =
            world.entities.iter().map(|e| (e.position, e.energy)).collect();
        let n = snap.len();
        // Hors d'atteinte d'un predateur, etat du tick precedent (tissue_pass tourne en phase 5b) :
        //  - `tissue_shelter` : une entite dont la cellule est a l'INTERIEUR d'une nappe
        //    (`tissue_bonds >= shelter_bonds`) est muree au centre, elle ne chasse pas non plus ;
        //  - `epithelium_shield` : une entite dont la cellule appartient a une nappe SCELLEE
        //    (`Cell.sealed`, tissu ordonne et grand) est protegee, meme au bord : l'epithelium
        //    fait rempart pour toute la feuille.
        let sheltered: Vec<bool> = if cfg.cells.tissue
            && (cfg.cells.tissue_shelter || cfg.cells.epithelium_shield)
        {
            let sb = cfg.cells.shelter_bonds.max(1) as u8;
            let use_shelter = cfg.cells.tissue_shelter;
            let use_shield = cfg.cells.epithelium_shield;
            let safe: std::collections::HashSet<u32> = world
                .cells
                .iter()
                .filter(|c| {
                    c.tissue.is_some()
                        && ((use_shelter && c.tissue_bonds >= sb) || (use_shield && c.sealed))
                })
                .map(|c| c.id)
                .collect();
            world
                .entities
                .iter()
                .map(|e| e.cell_id.map_or(false, |id| safe.contains(&id)))
                .collect()
        } else {
            vec![false; n]
        };
        let mut taken = vec![false; n];
        let mut gain = vec![0.0f32; n];
        let reach2 = reach * reach;
        for i in 0..n {
            if taken[i] || sheltered[i] {
                continue;
            }
            let ei = snap[i].1 + gain[i];
            if ei >= pcfg.hunt_below {
                continue;
            }
            let pos_i = snap[i].0;
            let need = ei * pcfg.prey_frac;
            let mut best: Option<(usize, f32)> = None;
            phash.for_each_neighbor(pos_i, reach, |ju| {
                let j = ju as usize;
                if j == i || taken[j] || sheltered[j] {
                    return;
                }
                let (pj, ej) = snap[j];
                if ej >= need {
                    return;
                }
                let d2 = pos_i.dist2(&pj);
                if d2 > reach2 {
                    return;
                }
                if best.map_or(true, |(_, bd)| d2 < bd) {
                    best = Some((j, d2));
                }
            });
            if let Some((j, _)) = best {
                gain[i] += snap[j].1 * pcfg.transfer;
                taken[j] = true;
                predated.push(world.entities[j].id);
            }
        }
        for i in 0..n {
            if gain[i] > 0.0 {
                world.entities[i].energy = (world.entities[i].energy + gain[i]).min(energy_ceiling);
            }
        }
        world.deaths_predation += predated.len() as u64;
    }

    drop(_sp.take()); _sp = prof::Span::start(4);
    // -- Phase 5b, cellules (0.0.2, tranche 2, etape 1). Entretien des cellules existantes
    //    (bilan, partage d'energie, departs, dissolution) chaque tick ; detection de
    //    nouvelles cellules tous les `check_every` ticks. Sequentiel, avant la mort pour que
    //    le partage d'energie puisse sauver un membre affame.
    cell_phase(world, cfg, &space, t, &mut events);

    drop(_sp.take()); _sp = prof::Span::start(5);
    // -- Phase 5c, cognition (0.0.3, tranche 1). Eveil des entites qui percoivent assez et
    //    ont vecu assez ; entretien de la memoire des agents (decroissance, nouveaux
    //    souvenirs) ; retombee des agents sans souvenir depuis longtemps. Sequentiel, sans
    //    RNG, avant la mort pour qu'un frole-la-mort non fatal soit memorise.
    cognition_phase(world, cfg, t, &index, &nerve_r2_mult, &mut events);

    drop(_sp.take()); _sp = prof::Span::start(6);
    // -- Phase 6, cycle de vie : vieillissement, mort par famine ou par age. Sequentiel :
    // travail par entite minuscule. Le retrait des morts et le depot des cadavres ecrivent
    // sur les cases, donc de toute facon sequentiels.
    let starve_at = cfg.lifecycle.starve_at;
    let lifespan_mean = cfg.lifecycle.lifespan_ticks_mean as f32;
    let age_curve = cfg.lifecycle.age_death_curve;
    let corpse_nut = cfg.environment.corpse_nutrients;
    let corpse_ret = cfg.environment.corpse_energy_return;
    let body_matter = cfg.bricks.body_matter;
    // Biologie de fond (tranche 8) : un corps use meurt plus tot de vieillesse.
    let wear_death_boost = cfg.biology.wear_death_boost.max(0.0);
    let bio_wear_start = cfg.biology.wear_start.clamp(0.0, 1.49);
    let bio_wear_floor = cfg.biology.wear_floor.clamp(0.0, 1.0);
    let bio_heal = cfg.biology.heal_rate.max(0.0);
    let bio_damage = cfg.biology.damage_rate.max(0.0);
    let bio_peril_energy = cfg.cognition.peril_frac * repro_threshold;

    // Decision : `dead` reste dans l'ordre des id. Une entite mangee en phase 5a consomme
    // quand meme son tirage RNG (le flux reste identique), mais meurt par predation.
    let predated_set: std::collections::HashSet<EntityId> = predated.iter().copied().collect();
    let mut dead: Vec<(EntityId, DeathCause)> = Vec::new();
    for i in 0..world.entities.len() {
        let roll = world.rng.next_f32();
        let e = &world.entities[i];
        if predated_set.contains(&e.id) {
            dead.push((e.id, DeathCause::Predation));
            continue;
        }
        if e.energy <= starve_at {
            dead.push((e.id, DeathCause::Starvation));
            continue;
        }
        let expected = lifespan_mean * (0.5 + e.genome.traits.lifespan);
        let frac = (e.age_ticks + 1) as f32 / expected.max(1.0);
        let p_death = if frac < 1.0 {
            0.0
        } else {
            (0.002 * (frac - 1.0).powf(age_curve)).min(0.5)
                * (1.0 + wear_death_boost * (1.0 - e.health))
        };
        if roll < p_death {
            dead.push((e.id, DeathCause::Age));
        }
    }

    for e in world.entities.iter_mut() {
        e.age_ticks += 1;
        e.cooldown = e.cooldown.saturating_sub(1);

        // Sante : integration lente de la condition biologique. Cible = usure de l'age,
        // sauf en famine ou la cible tombe a 0 et la degradation est plus rapide.
        let expected = (lifespan_mean * (0.5 + e.genome.traits.lifespan)).max(1.0);
        let age_frac = e.age_ticks as f32 / expected;
        let wear_target = if age_frac <= bio_wear_start {
            1.0
        } else {
            (1.0 - (age_frac - bio_wear_start) / (1.5 - bio_wear_start) * (1.0 - bio_wear_floor))
                .max(bio_wear_floor)
        };
        let (goal, rate) = if e.energy < bio_peril_energy {
            (0.0, bio_damage)
        } else {
            (wear_target, bio_heal)
        };
        e.health = (e.health + rate * (goal - e.health)).clamp(0.0, 1.0);
    }

    // Positions des morts de ce tick, avec le `seq` de leur `EntityDied` : les agents
    // temoins en garderont un souvenir ancre (0.0.3, tranche 3).
    let mut deaths_here: Vec<(Position, u64, u16)> = Vec::new();

    // Depot des cadavres puis retrait, une seule passe de `retain`.
    for &(id, cause) in &dead {
        let (dead_cell, dead_lineage, dead_pos) = world
            .get(id)
            .map(|e| (e.cell_id, e.genome.lineage, e.position))
            .unwrap_or((None, 0, Position { x: 0.0, y: 0.0 }));
        if let Some(e) = world.get(id) {
            let (pos, energy) = (e.position, e.energy.max(0.0));
            let idx = world.resources.index(&space, pos);
            world.resources.cell[idx] += corpse_nut + energy * corpse_ret;
        }
        // Un membre de cellule qui meurt : la cellule perd un effectif (bilan a jour pour
        // le lecteur ; la phase 5b le reconcilie de toute facon au tick suivant).
        if let Some(cid) = dead_cell {
            if let Some(c) = world.cell_mut(cid) {
                c.member_count = c.member_count.saturating_sub(1);
            }
        }
        // Le corps se decompose : sa matiere structurelle retourne au stock libre du monde.
        world.free_matter += body_matter;
        world.deaths_total += 1;
        match cause {
            DeathCause::Starvation => world.deaths_starvation += 1,
            DeathCause::Age => world.deaths_age += 1,
            // deja compte en phase 5a (world.deaths_predation), on n'incremente pas deux fois.
            DeathCause::Predation => {}
        }
        let seq = emit(
            &mut events,
            &mut world.next_event_seq,
            t,
            EventKind::EntityDied { entity: id, cause },
        );
        // Tracabilite causale : le crash de population cite cette vague de morts, l'extinction
        // de lignee cite la derniere mort d'un de ses membres.
        world.watch.deaths_since_check.push(seq);
        if world.watch.deaths_since_check.len() > 128 {
            world.watch.deaths_since_check.remove(0);
        }
        world.watch.last_death_seq_by_lineage.insert(dead_lineage, seq);
        deaths_here.push((dead_pos, seq, dead_lineage));
    }
    if !dead.is_empty() {
        let gone: std::collections::HashSet<EntityId> = dead.iter().map(|&(id, _)| id).collect();
        world.entities.retain(|e| !gone.contains(&e.id));
    }

    // Temoins : un agent proche d'une mort en garde un souvenir ancre sur l'`EntityDied`.
    // Apres le retrait des morts (pas d'auto-temoignage), sequentiel, sans RNG.
    let wc = &cfg.cognition;
    let wr2 = wc.witness_radius * wc.witness_radius;
    if wr2 > 0.0 && !deaths_here.is_empty() {
        let max_mem = wc.max_memories as usize;
        for &(dpos, dseq, dlin) in &deaths_here {
            for i in 0..world.entities.len() {
                let e = &world.entities[i];
                if e.mind.is_none()
                    || (wc.witness_kin_only && e.genome.lineage != dlin)
                    || e.position.dist2(&dpos) > wr2
                {
                    continue;
                }
                world.entities[i].mind.as_deref_mut().unwrap().record(
                    Memory {
                        formed_tick: t,
                        place: dpos,
                        kind: crate::cognition::MemoryKind::Witnessed,
                        event_seq: Some(dseq),
                        strength: 1.0,
                    },
                    max_mem,
                    wc.memory_merge_dist,
                );
            }
        }
    }

    drop(_sp.take()); _sp = prof::Span::start(7);
    // -- Phase 7, replication : scission asexuee (stade molecule).
    //
    // Chaque entite, dans l'ordre des EntityId, tente de se diviser si elle a l'energie
    // et si sa gestation est terminee. Pas de partenaire. Elle doit d'abord trouver la
    // matiere (briques) dans sa case ; sinon elle patiente. Puis la tentative coute
    // toujours : surcout metabolique, partage de l'energie en deux, gestation. Trois facons
    // de ne pas avoir d'enfant : manque de materiaux (briques), mutation letale (genome),
    // echec environnemental (sans infrastructure).
    let threshold = cfg.reproduction.energy_threshold;
    let cost = cfg.reproduction.energy_cost;
    let matter_retry_frac = cfg.bricks.retry_frac.max(0.0);
    let matter_total = cfg.bricks.matter_per_cell * (space.width as f32) * (space.height as f32);
    let matter_comfort = (cfg.bricks.comfort_frac.max(0.0) * matter_total).max(body_matter);
    let gest_base = cfg.reproduction.gestation_ticks_base as f32;
    let birth_loss_base = cfg.reproduction.birth_loss_base.clamp(0.0, 1.0);
    let crowd_half = cfg.reproduction.crowding_half.max(0.5);
    let maturity_frac = cfg.reproduction.maturity_frac.max(0.0);
    let lifespan_mean = cfg.lifecycle.lifespan_ticks_mean as f32;
    let mut newborns: Vec<Entity> = Vec::new();

    let coh_relief = cfg.cohesion.support_birth_relief;

    // Densite locale a jour (apres deplacement et morts), en cases de 1x1. Une case
    // surpeuplee, sans infrastructure, fait echouer la plupart des divisions.
    let repro_index = SpatialHash::build(&world.entities, &space, 1.0);

    // Eligible : assez d'energie, gestation terminee, et assez age (maturite). Un juvenile
    // ne se reproduit pas, ce qui casse la croissance exponentielle sans plafond artificiel.
    let candidates: Vec<EntityId> = world
        .entities
        .iter()
        .filter(|e| {
            e.energy >= threshold
                && e.cooldown == 0
                && (e.age_ticks as f32)
                    >= lifespan_mean * (0.5 + e.genome.traits.lifespan) * maturity_frac
        })
        .map(|e| e.id)
        .collect();

    let cell_birth_relief = cfg.cells.cell_birth_relief.clamp(0.0, 0.95);
    for a in candidates {
        let (pos, fert, support, in_cell) = {
            let e = world.get(a).unwrap();
            (e.position, e.genome.traits.fertility, e.colony_support, e.cell_id.is_some())
        };
        let local_n = repro_index.count_in_cell(pos).max(1);
        let gest = (gest_base * (1.5 - fert)).max(1.0) as u32;

        // Materiaux : batir un enfant prend `body_matter` du stock libre du monde. Quand la
        // population approche la capacite de charge, ce stock se tend. V2, plateau adouci :
        //  - stock au dela de `body_matter + matter_comfort` : la division passe toujours ;
        //  - en dessous : chance = (stock - body_matter) / matter_comfort, decroit jusqu'a 0
        //    quand il ne reste la matiere que d'un corps ;
        //  - stock sous `body_matter` : echec certain.
        // La division cale avant de couter l'energie ; le parent patiente (moitie moins sur
        // un echec probabiliste, la matiere se libere vite au plateau). La ponction se fait
        // plus bas, une fois l'enfant sur.
        let slack = world.free_matter - body_matter;
        let matter_ok = if slack < 0.0 {
            false
        } else if slack >= matter_comfort {
            true
        } else {
            world.rng.next_f32() < slack / matter_comfort
        };
        if !matter_ok {
            world.repro_blocked_materials += 1;
            let soft = slack >= 0.0;
            let frac = if soft { matter_retry_frac * 0.5 } else { matter_retry_frac };
            let wait = ((gest as f32) * frac).max(1.0) as u32;
            world.get_mut(a).unwrap().cooldown = wait;
            emit(
                &mut events,
                &mut world.next_event_seq,
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Materials },
            );
            continue;
        }

        // La division a lieu : surcout, puis energie partagee en deux moities egales.
        // Le rythme de replication depend de la fertilite (genome).
        let half = {
            let e = world.get_mut(a).unwrap();
            e.energy = ((e.energy - cost) * 0.5).max(0.0);
            e.last_action = Action::Divide;
            e.cooldown = gest;
            e.energy
        };

        // 1) echec d'origine environnementale : plancher sans infrastructure, abaisse par
        //    le support de colonie (l'agregat sert d'infrastructure, logique des coacervats),
        //    aggrave par la surpopulation de la case.
        let crowd = ((local_n.saturating_sub(1)) as f32 / crowd_half).min(1.0);
        let relief = (support * coh_relief).min(birth_loss_base * 0.8);
        let floor = (birth_loss_base - relief).max(0.0);
        let mut birth_loss = (floor + (1.0 - floor) * crowd).clamp(0.0, 0.99);
        // Reproduction protegee : un membre de cellule echoue moins souvent (0.0.2, tranche 2).
        if in_cell {
            birth_loss *= 1.0 - cell_birth_relief;
        }
        if world.rng.next_f32() < birth_loss {
            emit(
                &mut events,
                &mut world.next_event_seq,
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Environment },
            );
            continue;
        }

        // 2) copie du genome, avec chance de mutation letale (tirages RNG dans divide).
        let parent_genome = world.get(a).unwrap().genome.clone();
        let child_genome =
            match Genome::divide(
                &parent_genome,
                a,
                &cfg.reproduction,
                &mut world.rng,
                cfg.cells.adhesion_gene,
            ) {
                Some(g) => g,
                None => {
                    emit(
                        &mut events,
                        &mut world.next_event_seq,
                        t,
                        EventKind::ReplicationFailed {
                            parent: a,
                            reason: ReplicationFail::LethalMutation,
                        },
                    );
                    continue;
                }
            };

        // L'enfant se detache d'environ une case : les lignees se dispersent au lieu de
        // s'etouffer sur la case du parent.
        let child_pos = Position {
            x: (pos.x + (world.rng.next_f32() - 0.5) * 3.0).clamp(0.0, space.width as f32 - 0.001),
            y: (pos.y + (world.rng.next_f32() - 0.5) * 3.0).clamp(0.0, space.height as f32 - 0.001),
        };

        // L'enfant est viable : on ponctionne enfin la matiere. Une division plus tot ce
        // tick a pu vider le stock entre-temps ; dans ce cas rare, echec materiaux.
        if world.free_matter < body_matter {
            world.repro_blocked_materials += 1;
            let wait = ((gest as f32) * matter_retry_frac).max(1.0) as u32;
            world.get_mut(a).unwrap().cooldown = wait;
            emit(
                &mut events,
                &mut world.next_event_seq,
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Materials },
            );
            continue;
        }
        world.free_matter -= body_matter;

        let cid = world.next_entity_id;
        world.next_entity_id += 1;
        newborns.push(Entity {
            id: cid,
            genome: child_genome,
            position: child_pos,
            energy: half,
            age_ticks: 0,
            cooldown: 0,
            last_action: Action::Forage,
            target: None,
            colony_support: 0.0,
            // Un nouveau-ne est libre ; il rejoindra une cellule voisine a la prochaine
            // detection s'il est proche et parent.
            cell_id: None,
            // Un nouveau-ne n'a pas d'histoire : ni esprit, ni choc.
            mind: None,
            last_shock: None,
            // Un corps neuf demarre intact.
            health: 1.0,
            call_born: 0,
        });
        emit(
            &mut events,
            &mut world.next_event_seq,
            t,
            EventKind::EntityDivided { parent: a, child: cid },
        );
    }
    // Les nouveaux nes ont des id croissants, tous superieurs aux existants : un `push`
    // en fin de `Vec` garde le tri par id.
    for nb in newborns {
        let id = nb.id;
        world.entities.push(nb);
        world.births_total += 1;
        emit(&mut events, &mut world.next_event_seq, t, EventKind::EntitySpawned { entity: id });
    }

    drop(_sp.take()); _sp = prof::Span::start(8);
    // -- Phase 8b, veilleurs : detecteurs mecanises. Ils ne mutent que `world.watch` et
    //    produisent des evenements saillants (le materiau des chapitres). Jamais un `if`
    //    qui nomme le resultat (tranchee 7).
    if cfg.watch.interval_ticks > 0 && t % cfg.watch.interval_ticks == 0 {
        run_watchers(world, cfg, t, &mut events);
    }

    drop(_sp.take()); _sp = prof::Span::start(9);
    // -- Phase 8 (journal) et 9 (instantane) sont pilotees par l'appelant (CLI).

    // Garde-fou anti-cascade : on plafonne les evenements du tick, en gardant les plus
    // saillants (tri stable, donc deterministe). On rend ensuite l'ordre des `seq` : le
    // journal reste croissant (les seq des evenements tronques sont des trous, admis).
    let event_cap = cfg.events.max_events_per_tick as usize;
    if events.len() > event_cap {
        events.sort_by_key(|e| std::cmp::Reverse(e.salience));
        events.truncate(event_cap);
        events.sort_by_key(|e| e.seq);
    }
    events
}

/// Quantifie un genome en une cle : chaque trait de corps sur 2 bits (0..3), les
/// `SPECIES_TRAITS` premiers (7 x 2 = 14 bits, tient dans un u16). La personnalite
/// (`caution`, `curiosity`) ne compte pas dans la signature d'espece.
fn genome_key(t: &crate::genome::Traits) -> u16 {
    let mut k = 0u16;
    for (i, &q) in t.quantized().iter().take(SPECIES_TRAITS).enumerate() {
        k |= (q as u16) << (2 * i);
    }
    k
}

/// Meme cle, a partir d'un tableau de traits moyens (cellule).
fn genome_key_arr(a: &[f32; N_TRAITS]) -> u16 {
    let mut k = 0u16;
    for (i, &v) in a.iter().take(SPECIES_TRAITS).enumerate() {
        let q = (v * 4.0).clamp(0.0, 3.0) as u16;
        k |= q << (2 * i);
    }
    k
}

/// Union-find sur des indices 0..n, deterministe (union par rang, compression de chemin).
struct UnionFind {
    parent: Vec<u32>,
    rank: Vec<u8>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: (0..n as u32).collect(), rank: vec![0; n] }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.parent[r] as usize != r {
            r = self.parent[r] as usize;
        }
        let mut c = x;
        while c != r {
            let next = self.parent[c] as usize;
            self.parent[c] = r as u32;
            c = next;
        }
        r
    }
    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb as u32,
            std::cmp::Ordering::Greater => self.parent[rb] = ra as u32,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra as u32;
                self.rank[ra] += 1;
            }
        }
    }
}

/// Phase 5b : entretien des cellules chaque tick, detection tous les `check_every` ticks.
/// Etape 1 (membrane) : les membres restent dans `world.entities`, taggues `cell_id`.
fn cell_phase(
    world: &mut WorldState,
    cfg: &SimConfig,
    space: &Space,
    t: u64,
    events: &mut Vec<Event>,
) {
    let cc = cfg.cells.clone();

    // -- 1. Nettoyer les cell_id orphelins (cellule dissoute, ou instantane recharge).
    {
        let live: std::collections::HashSet<u32> = world.cells.iter().map(|c| c.id).collect();
        for e in world.entities.iter_mut() {
            if let Some(id) = e.cell_id {
                if !live.contains(&id) {
                    e.cell_id = None;
                }
            }
        }
    }

    // -- 2. Table cellule (indice dans world.cells) -> indices des membres, en ordre d'id.
    let ncells = world.cells.len();
    let cell_slot: std::collections::HashMap<u32, usize> =
        world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); ncells];
    for (i, e) in world.entities.iter().enumerate() {
        if let Some(id) = e.cell_id {
            if let Some(&k) = cell_slot.get(&id) {
                members[k].push(i);
            }
        }
    }

    // -- 3. Rafraichir le bilan, partager l'energie, gerer les departs et la dissolution.
    let mut dissolved: Vec<u32> = Vec::new();
    let share = cc.energy_share.clamp(0.0, 1.0);
    let members = members; // fige
    for (k, idxs) in members.iter().enumerate() {
        if idxs.is_empty() {
            dissolved.push(world.cells[k].id);
            continue;
        }
        let n = idxs.len() as f32;
        let (mut cx, mut cy, mut e_sum) = (0.0f32, 0.0f32, 0.0f32);
        let mut mean = [0.0f32; N_TRAITS];
        let mut adh_sum = 0.0f32;
        for &i in idxs {
            let e = &world.entities[i];
            cx += e.position.x;
            cy += e.position.y;
            e_sum += e.energy;
            let a = e.genome.traits.as_array();
            for j in 0..N_TRAITS {
                mean[j] += a[j];
            }
            adh_sum += e.genome.structural.adhesion;
        }
        cx /= n;
        cy /= n;
        let e_mean = e_sum / n;
        for m in mean.iter_mut() {
            *m /= n;
        }
        let mut rad = 0.0f32;
        for &i in idxs {
            let p = world.entities[i].position;
            rad += ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt();
        }
        rad /= n;

        // partage d'energie : chaque membre tend vers la moyenne de la cellule
        if share > 0.0 {
            for &i in idxs {
                let e = &mut world.entities[i];
                e.energy += (e_mean - e.energy) * share;
            }
        }

        // departs : un membre trop loin du centre quitte la cellule
        let leave_r = (rad * cc.leave_factor).max(cc.link_dist);
        let leave_r2 = leave_r * leave_r;
        for &i in idxs {
            let p = world.entities[i].position;
            if (p.x - cx).powi(2) + (p.y - cy).powi(2) > leave_r2 {
                world.entities[i].cell_id = None;
            }
        }
        let count = idxs
            .iter()
            .filter(|&&i| world.entities[i].cell_id.is_some())
            .count() as u32;

        // Dissolution (V2, avec hysteresis et delai de grace) : une cellule fraiche est
        // protegee sauf si elle tombe a zero ; sinon elle ne lache qu'en dessous des seuils
        // de dissolution, plus laches que les seuils de formation.
        let age = t.saturating_sub(world.cells[k].formed_tick);
        let dissolve = count == 0
            || (age >= cc.grace_ticks
                && (count < cc.dissolve_members.max(1)
                    || rad > cc.dissolve_spread
                    || mean[6] < cc.min_cohesion * 0.7));
        if dissolve {
            for &i in idxs {
                world.entities[i].cell_id = None;
            }
            dissolved.push(world.cells[k].id);
            continue;
        }

        // Forme de la membrane : allongement, pour la division (3c) et le rendu.
        let offsets: Vec<(f32, f32)> = idxs
            .iter()
            .filter(|&&i| world.entities[i].cell_id.is_some())
            .map(|&i| {
                let p = world.entities[i].position;
                (p.x - cx, p.y - cy)
            })
            .collect();
        let (elong, _) = cloud_shape(&offsets);

        let c = &mut world.cells[k];
        c.position = Position { x: cx, y: cy };
        c.radius = rad;
        c.member_count = count;
        c.mean_traits = mean;
        c.mean_adhesion = adh_sum / n;
        c.genome_key = genome_key_arr(&mean);
        c.elongation = elong;
    }
    if !dissolved.is_empty() {
        let gone: std::collections::HashSet<u32> = dissolved.iter().copied().collect();
        world.cells.retain(|c| !gone.contains(&c.id));
        for id in dissolved {
            world.cells_dissolved_total += 1;
            emit(events, &mut world.next_event_seq, t, EventKind::CellDissolved { cell: id });
        }
    }

    // -- 3a. Repulsion : deux membranes qui se chevauchent mais dont les genomes sont trop
    //    distants pour fusionner (`> fuse_kin`) se repoussent doucement. Les membres de chaque
    //    cellule glissent a l'oppose de l'autre centre : la membrane devient une frontiere, des
    //    cellules non parentes se cotoient au lieu de se traverser. Sequentiel, sans RNG, ordre
    //    d'id ; poussee accumulee par entite puis appliquee, bornee, gardee dans la grille.
    if cc.repel && world.cells.len() >= 2 {
        let slot: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let mut mem: Vec<Vec<usize>> = vec![Vec::new(); world.cells.len()];
        for (i, e) in world.entities.iter().enumerate() {
            if let Some(id) = e.cell_id {
                if let Some(&k) = slot.get(&id) {
                    mem[k].push(i);
                }
            }
        }
        let snap: Vec<(Position, f32, [f32; N_TRAITS])> =
            world.cells.iter().map(|c| (c.position, c.radius, c.mean_traits)).collect();
        let mut order: Vec<usize> = (0..world.cells.len()).collect();
        order.sort_by_key(|&k| world.cells[k].id);

        let mut push = vec![(0.0f32, 0.0f32); world.entities.len()];
        for ii in 0..order.len() {
            for jj in (ii + 1)..order.len() {
                let (a, b) = (order[ii], order[jj]);
                let (pa, ra, ta) = &snap[a];
                let (pb, rb, tb) = &snap[b];
                let dx = pb.x - pa.x;
                let dy = pb.y - pa.y;
                let d = (dx * dx + dy * dy).sqrt().max(1e-3);
                // On agit des que les membranes se frolent (facteur 1,6), pas seulement quand
                // les nuages de membres se recouvrent : ca cree une vraie distance de garde.
                let reach = (ra + rb) * 1.6;
                if d >= reach || trait_l1(ta, tb) <= cc.fuse_kin {
                    continue;
                }
                let mag = (((reach - d) / reach) * cc.repel_strength.max(0.0)).min(0.4);
                let (nx, ny) = (dx / d, dy / d);
                for &i in &mem[a] {
                    push[i].0 -= nx * mag;
                    push[i].1 -= ny * mag;
                }
                for &i in &mem[b] {
                    push[i].0 += nx * mag;
                    push[i].1 += ny * mag;
                }
            }
        }
        let (gw, gh) = (space.width as f32, space.height as f32);
        for (i, (px, py)) in push.into_iter().enumerate() {
            if px == 0.0 && py == 0.0 {
                continue;
            }
            let e = &mut world.entities[i];
            e.position.x = (e.position.x + px.clamp(-1.2, 1.2)).clamp(0.0, gw - 1e-3);
            e.position.y = (e.position.y + py.clamp(-1.2, 1.2)).clamp(0.0, gh - 1e-3);
        }
    }

    // -- 3b. Fusion : deux membranes stables qui se chevauchent et dont les genomes moyens se
    //    ressemblent n'en font plus qu'une. La plus grosse garde son identite et son histoire ;
    //    la petite y disparait. Rien ne declenche ca a la main : c'est une condition
    //    geometrique et genetique que le monde franchit tout seul quand deux cellules parentes
    //    derivent l'une dans l'autre (T-7). Ordre stable (cells triees par id), une fusion par
    //    cellule et par tick, decisions d'abord puis application.
    if cc.fuse && world.cells.len() >= 2 {
        let n = world.cells.len();
        let mut taken: Vec<bool> = vec![false; n];
        let mut pairs: Vec<(u32, u32)> = Vec::new();
        for a in 0..n {
            if taken[a] {
                continue;
            }
            for b in (a + 1)..n {
                if taken[b] {
                    continue;
                }
                let (ca, cb) = (&world.cells[a], &world.cells[b]);
                if t.saturating_sub(ca.formed_tick) < cc.grace_ticks
                    || t.saturating_sub(cb.formed_tick) < cc.grace_ticks
                {
                    continue;
                }
                let d2 = (ca.position.x - cb.position.x).powi(2)
                    + (ca.position.y - cb.position.y).powi(2);
                let reach = (ca.radius + cb.radius) * cc.fuse_overlap;
                if d2 > reach * reach {
                    continue;
                }
                if trait_l1(&ca.mean_traits, &cb.mean_traits) > cc.fuse_kin {
                    continue;
                }
                // le plus gros garde son id (a effectif egal, le plus ancien = le plus petit id)
                let (keep, gone) = if ca.member_count >= cb.member_count {
                    (ca.id, cb.id)
                } else {
                    (cb.id, ca.id)
                };
                taken[a] = true;
                taken[b] = true;
                pairs.push((keep, gone));
                break;
            }
        }
        for (keep, gone) in pairs {
            let gone_count = world
                .cells
                .iter()
                .find(|c| c.id == gone)
                .map(|c| c.member_count)
                .unwrap_or(0);
            for e in world.entities.iter_mut() {
                if e.cell_id == Some(gone) {
                    e.cell_id = Some(keep);
                }
            }
            world.cells.retain(|c| c.id != gone);
            let (size, at) = world.cell_mut(keep).map_or((0, [0.0, 0.0]), |c| {
                c.member_count += gone_count;
                (c.member_count, [c.position.x, c.position.y])
            });
            world.cells_merged_total += 1;
            emit(
                events,
                &mut world.next_event_seq,
                t,
                EventKind::CellsMerged { cell: keep, absorbed: gone, size, at },
            );
        }
    }

    // -- 3c. Division : une cellule grande, mure et etiree se pince en deux. C'est la
    //    reproduction cellulaire : la cellule devient une unite qui se reproduit. Ordre stable
    //    (cellules triees par id), une division par cellule et par tick, decisions puis
    //    application. Rien ne la declenche a la main : c'est une condition geometrique
    //    (taille, age, forme etiree par la chimiotaxie vers deux zones) que le monde franchit.
    if cc.divide && !world.cells.is_empty() {
        let slot: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let mut mem: Vec<Vec<usize>> = vec![Vec::new(); world.cells.len()];
        for (i, e) in world.entities.iter().enumerate() {
            if let Some(id) = e.cell_id {
                if let Some(&k) = slot.get(&id) {
                    mem[k].push(i);
                }
            }
        }
        let mut order: Vec<usize> = (0..world.cells.len()).collect();
        order.sort_by_key(|&k| world.cells[k].id);

        let mut splits: Vec<(u32, Vec<usize>)> = Vec::new();
        for &k in &order {
            let c = &world.cells[k];
            let age = t.saturating_sub(c.formed_tick);
            // Une cellule ancree dans un tissu par plusieurs liens resiste a la division : le
            // seuil d'allongement monte avec le nombre de liens (`tissue_bonds`, tick precedent).
            // Une cellule tissee est somatique (elle tient la nappe) ; une cellule libre ou de
            // bord se pince normalement. Sans `tissue_bond`, aucun effet.
            let elong_gate = cc.divide_elongation
                + if cc.tissue_bond { c.tissue_bonds as f32 * cc.divide_bond_resist.max(0.0) } else { 0.0 };
            if c.member_count < cc.divide_members
                || age < cc.divide_age_ticks
                || c.elongation < elong_gate
            {
                continue;
            }
            let idxs = &mem[k];
            if idxs.len() < cc.divide_members as usize {
                continue;
            }
            let (cx, cy) = (c.position.x, c.position.y);
            let offs: Vec<(f32, f32)> = idxs
                .iter()
                .map(|&i| {
                    let p = world.entities[i].position;
                    (p.x - cx, p.y - cy)
                })
                .collect();
            let (_, (ax, ay)) = cloud_shape(&offs);
            // projeter sur l'axe long, trier (tiebreak par id d'entite, deja l'ordre de idxs)
            let mut proj: Vec<(f32, usize)> = idxs
                .iter()
                .map(|&i| {
                    let p = world.entities[i].position;
                    ((p.x - cx) * ax + (p.y - cy) * ay, i)
                })
                .collect();
            proj.sort_by(|a, b| {
                a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1))
            });
            let half = proj.len() / 2;
            let leaving: Vec<usize> = proj[half..].iter().map(|(_, i)| *i).collect();
            let staying = proj.len() - leaving.len();
            if leaving.len() < cc.min_members as usize || staying < cc.min_members as usize {
                continue; // les deux moities doivent etre viables seules
            }
            splits.push((c.id, leaving));
        }

        for (parent_id, leaving) in splits {
            let child_id = world.next_cell_id;
            world.next_cell_id += 1;
            for &i in &leaving {
                world.entities[i].cell_id = Some(child_id);
            }
            let n = leaving.len() as f32;
            let (mut cx, mut cy) = (0.0f32, 0.0f32);
            let mut mean = [0.0f32; N_TRAITS];
            let mut adh_sum = 0.0f32;
            for &i in &leaving {
                let e = &world.entities[i];
                cx += e.position.x;
                cy += e.position.y;
                let a = e.genome.traits.as_array();
                for j in 0..N_TRAITS {
                    mean[j] += a[j];
                }
                adh_sum += e.genome.structural.adhesion;
            }
            cx /= n;
            cy /= n;
            for m in mean.iter_mut() {
                *m /= n;
            }
            let adh_mean = adh_sum / n;
            let mut rad = 0.0f32;
            for &i in &leaving {
                let p = world.entities[i].position;
                rad += ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt();
            }
            rad /= n;
            world.cells.push(Cell {
                id: child_id,
                formed_tick: t,
                position: Position { x: cx, y: cy },
                radius: rad,
                member_count: leaving.len() as u32,
                genome_key: genome_key_arr(&mean),
                mean_traits: mean,
                elongation: 1.0,
                parent_cell: Some(parent_id),
                tissue: None,
                tissue_bonds: 0,
                sealed: false,
                organism: None,
                mean_adhesion: adh_mean,
            });
            let at = if let Some(pc) = world.cell_mut(parent_id) {
                pc.member_count = pc.member_count.saturating_sub(leaving.len() as u32);
                pc.elongation = 1.0;
                pc.formed_tick = t; // protege les deux moities d'une refusion immediate
                [pc.position.x, pc.position.y]
            } else {
                [cx, cy]
            };
            world.cells_divided_total += 1;
            emit(
                events,
                &mut world.next_event_seq,
                t,
                EventKind::CellDivided {
                    parent: parent_id,
                    child: child_id,
                    size: leaving.len() as u32,
                    at,
                },
            );
        }
    }

    // -- 3d. Tissus (0.0.2, `[cells] tissue`, config seulement). Des cellules de genome proche
    //    (`trait_l1 <= tissue_kin`) dont les membranes se touchent adherent : elles forment un
    //    tissu = composante connexe d'au moins `tissue_min` cellules, id = plus petit id du
    //    groupe. Sans `tissue_bond` : derive tick a tick d'un test de distance. Avec
    //    `tissue_bond` : la connexite vient de LIENS de paire gardes dans le temps
    //    (`world.cell_bonds`), qui ne cassent qu'au-dela d'un etirement franc -> le tissu tient
    //    une perturbation au lieu de se defaire au premier ecart.
    tissue_pass(
        world,
        &cc,
        space,
        t,
        cfg.lifecycle.starve_at,
        cfg.reproduction.energy_threshold * 2.0,
    );

    // -- 3e. Organismes (0.0.2, `[organism] enabled`, config seulement). Une composante connexe
    //    de cellules qui adherent (sans parente exigee), reconnue apres quelques controles et
    //    gardee avec un id stable. Aux controles seulement.
    organism_pass(
        world,
        &cfg.organism,
        t,
        events,
        cfg.lifecycle.starve_at,
        cfg.reproduction.energy_threshold * 2.0,
    );

    // -- 3f. Contraction musculaire (0.0.2, `[cells] muscle_contract`). Une cellule d'un tissu
    //    dont le nuage de membres est nettement etire exerce une force axiale oscillante : elle
    //    se resserre et se relache, dephasee le long d'une onde qui traverse le tissu.
    if cc.muscle_contract && !world.cells.is_empty() {
        muscle_pass(world, &cc, space, t);
    }

    // -- 4. Detection de nouvelles cellules, tous les `check_every` ticks.
    if cc.check_every == 0 || t % cc.check_every != 0 {
        return;
    }
    cell_detect(world, &cc, space, t, events);
}

/// Reconnait et suit les organismes : composantes connexes de cellules qui se touchent
/// (`organism.reach`, pas de parente), avec une identite stable dans le temps. Tourne aux
/// controles (`organism.check_every`). Sequentiel, sans RNG, ordre des id de cellule.
fn organism_pass(
    world: &mut WorldState,
    oc: &crate::config::OrganismCfg,
    t: u64,
    events: &mut Vec<Event>,
    starve_at: f32,
    energy_ceiling: f32,
) {
    use std::collections::{HashMap, HashSet};
    if !oc.enabled {
        if !world.organisms.is_empty() {
            world.organisms.clear();
            world.watch.org_pending.clear();
        }
        for c in world.cells.iter_mut() {
            c.organism = None;
        }
        return;
    }
    if t % oc.check_every.max(1) != 0 {
        return;
    }

    let n = world.cells.len();
    // 1. composantes connexes par contact des membranes (aucune parente exigee).
    let reach = oc.reach.max(0.1);
    let mut uf: Vec<usize> = (0..n).collect();
    fn find(uf: &mut [usize], mut x: usize) -> usize {
        while uf[x] != x {
            uf[x] = uf[uf[x]];
            x = uf[x];
        }
        x
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&k| world.cells[k].id);
    for ii in 0..n {
        for jj in (ii + 1)..n {
            let (a, b) = (order[ii], order[jj]);
            let (ca, cb) = (&world.cells[a], &world.cells[b]);
            let rr = (ca.radius + cb.radius) * reach;
            if ca.position.dist2(&cb.position) <= rr * rr {
                let (ra, rb) = (find(&mut uf, a), find(&mut uf, b));
                if ra != rb {
                    let (lo, hi) = if world.cells[ra].id <= world.cells[rb].id { (ra, rb) } else { (rb, ra) };
                    uf[hi] = lo;
                }
            }
        }
    }
    let mut comps: HashMap<usize, Vec<usize>> = HashMap::new();
    for k in 0..n {
        let r = find(&mut uf, k);
        comps.entry(r).or_default().push(k);
    }
    let mut cands: Vec<Vec<usize>> =
        comps.into_values().filter(|g| g.len() as u32 >= oc.min_cells.max(2)).collect();
    cands.sort_by_key(|g| g.iter().map(|&k| world.cells[k].id).min().unwrap_or(0));

    // 2. faire continuer les organismes existants : chacun prend la composante qui contient le
    //    plus de ses cellules actuelles. Ordre des id (le plus ancien choisit en premier).
    let mut orgs = std::mem::take(&mut world.organisms);
    orgs.sort_by_key(|o| o.id);
    let mut claimed = vec![false; cands.len()];
    let mut claimed_cells: HashMap<u32, HashSet<usize>> = HashMap::new();
    let mut survivors: Vec<crate::world::Organism> = Vec::new();
    for mut o in orgs {
        let mut best: Option<(usize, usize)> = None;
        for (ci, g) in cands.iter().enumerate() {
            if claimed[ci] {
                continue;
            }
            let ov = g.iter().filter(|&&k| world.cells[k].organism == Some(o.id)).count();
            if ov > 0 && best.map_or(true, |(_, b)| ov > b) {
                best = Some((ci, ov));
            }
        }
        if let Some((ci, _)) = best {
            claimed[ci] = true;
            o.miss = 0;
            o.cells = cands[ci].len().min(u16::MAX as usize) as u16;
            let set: HashSet<usize> = cands[ci].iter().copied().collect();
            for &k in &cands[ci] {
                world.cells[k].organism = Some(o.id);
            }
            claimed_cells.insert(o.id, set);
            survivors.push(o);
        } else {
            o.miss = o.miss.saturating_add(1);
            if o.miss as u32 > oc.persist_checks.max(1) {
                emit(
                    events,
                    &mut world.next_event_seq,
                    t,
                    EventKind::OrganismDissolved { organism: o.id },
                );
            } else {
                survivors.push(o); // en sursis : garde son id, pas de cellules ce controle
            }
        }
    }

    // 3. nettoyage : une cellule qui pointe un organisme disparu, ou qui n'est plus dans la
    //    composante revendiquee par le sien, redevient libre.
    let alive: HashSet<u32> = survivors.iter().map(|o| o.id).collect();
    for k in 0..n {
        if let Some(x) = world.cells[k].organism {
            let ok = alive.contains(&x) && claimed_cells.get(&x).is_some_and(|s| s.contains(&k));
            if !ok {
                world.cells[k].organism = None;
            }
        }
    }

    // 4. composantes non revendiquees : candidates, reconnues apres `persist_checks` controles
    //    tenus (appariement par centroide, comme la formation de cellule).
    let old_pending = std::mem::take(&mut world.watch.org_pending);
    let mut new_pending: Vec<(Position, u16)> = Vec::new();
    let match_r2 = 7.0f32 * 7.0;
    for (ci, g) in cands.iter().enumerate() {
        if claimed[ci] {
            continue;
        }
        let (mut cx, mut cy) = (0.0f32, 0.0f32);
        for &k in g {
            cx += world.cells[k].position.x;
            cy += world.cells[k].position.y;
        }
        cx /= g.len() as f32;
        cy /= g.len() as f32;
        let mut streak = 1u16;
        let mut best = f32::MAX;
        for &(op, os) in &old_pending {
            let d2 = (cx - op.x).powi(2) + (cy - op.y).powi(2);
            if d2 <= match_r2 && d2 < best {
                best = d2;
                streak = os.saturating_add(1);
            }
        }
        if streak >= oc.persist_checks.max(1) as u16 {
            let id = world.next_organism_id;
            world.next_organism_id += 1;
            let name = crate::names::organism_name(id);
            for &k in g {
                world.cells[k].organism = Some(id);
            }
            survivors.push(crate::world::Organism {
                id,
                born_tick: t,
                name,
                cells: g.len().min(u16::MAX as usize) as u16,
                miss: 0,
            });
            world.organisms_formed_total += 1;
            emit(
                events,
                &mut world.next_event_seq,
                t,
                EventKind::OrganismFormed { organism: id, cells: g.len() as u32 },
            );
        } else {
            new_pending.push((Position { x: cx, y: cy }, streak));
        }
    }
    survivors.sort_by_key(|o| o.id);
    world.organisms = survivors;
    world.watch.org_pending = new_pending;

    // Mise en commun de l'energie : chaque membre d'un organisme est ramene d'une fraction vers
    // l'energie moyenne des membres de l'organisme. L'organisme a faim ou est repu en entier.
    // Conserve (deplacement vers la moyenne), sans RNG, ordre des cellules. Seuls les
    // organismes qui ont au moins deux cellules avec des membres comptent.
    let share = oc.pool_share.clamp(0.0, 1.0);
    if share > 0.0 && !world.organisms.is_empty() {
        let cslot: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        // organisme -> (somme d'energie, nombre de membres, indices d'entites)
        let mut pool: std::collections::HashMap<u32, (f32, u32, Vec<usize>)> =
            std::collections::HashMap::new();
        for (i, e) in world.entities.iter().enumerate() {
            let oid = match e.cell_id.and_then(|id| cslot.get(&id)).and_then(|&k| world.cells[k].organism) {
                Some(o) => o,
                None => continue,
            };
            let ent = pool.entry(oid).or_insert((0.0, 0, Vec::new()));
            ent.0 += e.energy;
            ent.1 += 1;
            ent.2.push(i);
        }
        let mut oids: Vec<u32> = pool.keys().copied().collect();
        oids.sort_unstable();
        for oid in oids {
            let (sum, count, idxs) = &pool[&oid];
            if *count < 2 {
                continue;
            }
            let mean = sum / *count as f32;
            for &i in idxs {
                let e = &mut world.entities[i];
                e.energy += (mean - e.energy) * share;
            }
        }
    }

    // Reserve adipeuse (0.0.2, `[organism] adipeux_share`) : en plus du lissage uniforme
    // ci-dessus, les membres d'une cellule RONDE (`elongation < 1.6`) et GORGEE (energie >=
    // `adipeux_rich_frac` du plafond) versent une part de LEUR surplus aux membres de
    // l'organisme vraiment en danger (energie sous 2x le seuil de famine). Une graisse de
    // reserve qui ne se vide que dans le besoin, pas un lissage constant : distinct de
    // `pool_share`. Conserve, sans RNG, ordre des id d'organisme puis des entites. Aucune
    // activite ajoutee (aucun mouvement, aucune entite hors de l'organisme ponctionnee).
    let adi_share = oc.adipeux_share.clamp(0.0, 1.0);
    if adi_share > 0.0 && !world.organisms.is_empty() {
        let cslot: HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let rich_at = energy_ceiling * oc.adipeux_rich_frac.clamp(0.0, 1.0);
        // Marge au-dessus du point de mort par famine (`starve_at`, souvent 0), a l'echelle du
        // monde plutot qu'un doublement degenere quand `starve_at = 0`.
        let danger_at = starve_at + energy_ceiling * 0.08;
        // organisme -> (indices donneurs gorges, indices en danger)
        let mut groups: HashMap<u32, (Vec<usize>, Vec<usize>)> = HashMap::new();
        for (i, e) in world.entities.iter().enumerate() {
            let Some(&k) = e.cell_id.and_then(|id| cslot.get(&id)) else { continue };
            let Some(oid) = world.cells[k].organism else { continue };
            let round = world.cells[k].elongation < 1.6;
            let grp = groups.entry(oid).or_default();
            if round && e.energy >= rich_at {
                grp.0.push(i);
            } else if e.energy < danger_at {
                grp.1.push(i);
            }
        }
        let mut oids: Vec<u32> = groups.keys().copied().collect();
        oids.sort_unstable();
        let mut delta = vec![0.0f32; world.entities.len()];
        for oid in oids {
            let (donors, needy) = &groups[&oid];
            if donors.is_empty() || needy.is_empty() {
                continue;
            }
            let mut pot = 0.0f32;
            for &i in donors {
                pot += (world.entities[i].energy - rich_at).max(0.0) * adi_share;
            }
            if pot <= 0.0 {
                continue;
            }
            let per = pot / needy.len() as f32;
            for &i in donors {
                let surplus = (world.entities[i].energy - rich_at).max(0.0);
                delta[i] -= surplus * adi_share;
            }
            for &i in needy {
                delta[i] += per;
            }
        }
        for (i, d) in delta.into_iter().enumerate() {
            if d != 0.0 {
                world.entities[i].energy = (world.entities[i].energy + d).clamp(0.0, energy_ceiling);
            }
        }
    }
}

/// Contraction musculaire (0.0.2, `[cells] muscle_contract`). Pour chaque cellule d'un tissu
/// dont les membres forment un nuage assez fusiforme (`elongation >= muscle_elong`) : une force
/// axiale oscillante (resserrement le long du grand axe, gonflement le long du petit),
/// dephasee par une onde qui traverse le tissu. Un peu de courant sur les entites libres
/// proches, pendant la phase active. Sequentiel, sans RNG, ordre des id ; poussee accumulee
/// par entite puis appliquee, bornee, gardee dans la grille.
fn muscle_pass(world: &mut WorldState, cc: &crate::config::CellsCfg, space: &Space, t: u64) {
    let elong_min = cc.muscle_elong.max(1.0);
    let strength = cc.muscle_strength.max(0.0);
    let period = cc.muscle_period_ticks.max(1) as f32;
    if strength <= 0.0 {
        return;
    }
    let n = world.cells.len();
    // Rien a faire s'il n'y a aucune cellule assez fusiforme en tissu : on evite meme de
    // construire les tables (le cas le plus frequent).
    if !world
        .cells
        .iter()
        .any(|c| c.tissue.is_some() && c.elongation >= elong_min)
    {
        return;
    }
    let slot: std::collections::HashMap<u32, usize> =
        world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
    let mut mem: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, e) in world.entities.iter().enumerate() {
        if let Some(id) = e.cell_id {
            if let Some(&k) = slot.get(&id) {
                mem[k].push(i);
            }
        }
    }
    // Cellules contractiles, dans l'ordre des id (deterministe).
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&k| world.cells[k].id);

    let mut push = vec![(0.0f32, 0.0f32); world.entities.len()];
    let two_pi = std::f32::consts::TAU;
    // hash spatial des entites libres, pour le courant (portee courte).
    let free_hash = SpatialHash::build(&world.entities, space, 3.0);

    for &k in &order {
        let c = &world.cells[k];
        if c.tissue.is_none() || c.elongation < elong_min || mem[k].len() < 4 {
            continue;
        }
        let (cx, cy) = (c.position.x, c.position.y);
        let tid = c.tissue.unwrap_or(0) as f32;
        // axe principal du nuage de membres
        let offs: Vec<(f32, f32)> =
            mem[k].iter().map(|&i| (world.entities[i].position.x - cx, world.entities[i].position.y - cy)).collect();
        let (_, (ax, ay)) = cloud_shape(&offs);
        // onde peristaltique : phase qui glisse le long d'une direction. Sans locomotion
        // dirigee, cette direction est arbitraire (fonction de l'id du tissu). Avec
        // `muscle_seek_food`, c'est la meme chimiotaxie que `forage_target` (deja utilisee par
        // chaque entite pour chercher a manger) appliquee au tissu : s'il y a mieux a portee,
        // l'onde s'oriente vers la nourriture ; sinon elle retombe sur l'axe arbitraire. `seek`
        // ne porte que le cas ou une cible reelle a ete sentie : c'est ce qui, plus bas,
        // autorise un pas net vers elle (une cellule qui ne fait que reformer sa silhouette
        // sur elle-meme ne va nulle part, il faut un deplacement du nuage entier).
        let arbitrary = ((tid * 0.7).cos(), (tid * 0.7).sin());
        let seek: Option<(f32, f32)> = if cc.muscle_seek_food {
            let sense = (c.radius * cc.muscle_sense_radius.max(0.5)).max(2.0);
            forage_target(&world.resources, space, c.position, sense).and_then(|tgt| {
                let dx = tgt.x - cx;
                let dy = tgt.y - cy;
                let d = (dx * dx + dy * dy).sqrt();
                if d > 1e-3 { Some((dx / d, dy / d)) } else { None }
            })
        } else {
            None
        };
        let (wx, wy) = seek.unwrap_or(arbitrary);
        let phase = two_pi * (t as f32) / period - 0.08 * (cx * wx + cy * wy);
        let s = phase.sin();
        // plus de resserrement que de gonflement (un muscle tire, il ne pousse pas) :
        let contract = if s >= 0.0 { s } else { s * 0.4 };
        let mag = strength * contract;
        for (&i, &(dx, dy)) in mem[k].iter().zip(offs.iter()) {
            let along = dx * ax + dy * ay;
            let perp = -dx * ay + dy * ax;
            // resserre le long du grand axe, gonfle (moitie) le long du petit
            let d_along = -mag * along * 0.5;
            let d_perp = mag * perp * 0.25;
            push[i].0 += d_along * ax - d_perp * ay;
            push[i].1 += d_along * ay + d_perp * ax;
        }
        // pas de reptation : reformer sa silhouette sur son propre centre ne deplace nulle part
        // (une contraction pure est symetrique). S'il y a une cible reellement sentie
        // (`seek`), la cellule tire TOUT son nuage d'un cran vers elle pendant la phase active
        // de contraction -- une extension de pseudopode, pas une teleportation : bornee, au
        // rythme du muscle, seulement quand il y a vraiment quelque chose a atteindre.
        if let Some((sx, sy)) = seek {
            if contract > 0.05 {
                let drift = strength * contract * 0.4;
                for &i in &mem[k] {
                    push[i].0 += drift * sx;
                    push[i].1 += drift * sy;
                }
            }
        }
        // courant : pendant la phase active, la cellule pousse doucement les entites libres
        // proches, radialement. C'est le germe d'un courant / d'une reptation.
        if contract > 0.05 {
            let reach = (c.radius * 1.6).max(2.0);
            let cur = strength * contract * 0.35;
            free_hash.for_each_neighbor(c.position, reach, |ju| {
                let j = ju as usize;
                if world.entities[j].cell_id.is_some() {
                    return;
                }
                let rx = world.entities[j].position.x - cx;
                let ry = world.entities[j].position.y - cy;
                let d = (rx * rx + ry * ry).sqrt().max(1e-3);
                if d > reach {
                    return;
                }
                let f = cur * (1.0 - d / reach);
                push[j].0 += rx / d * f;
                push[j].1 += ry / d * f;
            });
        }
    }
    let (gw, gh) = (space.width as f32, space.height as f32);
    for (i, (px, py)) in push.into_iter().enumerate() {
        if px == 0.0 && py == 0.0 {
            continue;
        }
        let e = &mut world.entities[i];
        e.position.x = (e.position.x + px.clamp(-0.35, 0.35)).clamp(0.0, gw - 1e-3);
        e.position.y = (e.position.y + py.clamp(-0.35, 0.35)).clamp(0.0, gh - 1e-3);
    }
}

fn tissue_pass(
    world: &mut WorldState,
    cc: &crate::config::CellsCfg,
    space: &Space,
    _t: u64,
    starve_at: f32,
    energy_ceiling: f32,
) {
    if !cc.tissue || world.cells.len() < 2 {
        for c in world.cells.iter_mut() {
            c.tissue = None;
            c.sealed = false;
        }
        if !world.cell_bonds.is_empty() {
            world.cell_bonds.clear();
        }
        world.tissues_alive = 0;
        world.tissue_order = 0.0;
        return;
    }
    let n = world.cells.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&k| world.cells[k].id);

    let mut uf: Vec<usize> = (0..n).collect();
    let mut neigh: Vec<Vec<usize>> = vec![Vec::new(); n];
    fn find(uf: &mut [usize], mut x: usize) -> usize {
        while uf[x] != x {
            uf[x] = uf[uf[x]];
            x = uf[x];
        }
        x
    }
    // Gene d'adhesion (0.0.2, piste D etape 1, `[cells] adhesion_gene`) : le seuil de parente
    // pour adherer n'est plus forcement le meme pour tout le monde. Multiplicateur PERSONNEL a
    // la paire, tire de la tolerance heritee des deux cellules (`mean_adhesion`, moyenne du
    // gene sur leurs membres). `1.0` (neutre, comportement inchange) si le levier est coupe.
    let adh_lo = cc.adhesion_mult_min.max(0.05);
    let adh_span = (cc.adhesion_mult_max - adh_lo).max(0.0);
    let adhesion_gene = cc.adhesion_gene;
    let adhesion_mult = |ca: &Cell, cb: &Cell| -> f32 {
        if !adhesion_gene {
            return 1.0;
        }
        adh_lo + ((ca.mean_adhesion + cb.mean_adhesion) * 0.5).clamp(0.0, 1.0) * adh_span
    };
    if cc.tissue_bond {
        // --- adhesion persistante : ce sont les LIENS gardes dans le temps qui portent la
        //     connexite, pas un test de distance refait de zero. Un tissu tient une perturbation.
        let idx: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let form = cc.bond_form.max(0.1);
        let brk = cc.bond_break.max(form + 0.05);
        // 1. elaguer les liens morts : cellule disparue, paire trop etiree, genomes trop derives.
        let mut bonds = std::mem::take(&mut world.cell_bonds);
        bonds.retain(|&(a, b)| {
            let (ka, kb) = match (idx.get(&a), idx.get(&b)) {
                (Some(&ka), Some(&kb)) => (ka, kb),
                _ => return false,
            };
            let (ca, cb) = (&world.cells[ka], &world.cells[kb]);
            let lim = (ca.radius + cb.radius) * brk;
            let kin_keep = cc.tissue_kin * 1.8 * adhesion_mult(ca, cb);
            ca.position.dist2(&cb.position) <= lim * lim
                && trait_l1(&ca.mean_traits, &cb.mean_traits) <= kin_keep
        });
        // 2. nouer les liens neufs : paires parentes qui se touchent, pas deja liees. Meme cout
        //    que l'ancien balayage O(n^2) (~120 cellules).
        let mut set: std::collections::HashSet<(u32, u32)> = bonds.iter().copied().collect();
        for ii in 0..n {
            for jj in (ii + 1)..n {
                let (a, b) = (order[ii], order[jj]);
                let (ca, cb) = (&world.cells[a], &world.cells[b]);
                let key = if ca.id <= cb.id { (ca.id, cb.id) } else { (cb.id, ca.id) };
                if set.contains(&key) {
                    continue;
                }
                let reach = (ca.radius + cb.radius) * form;
                if ca.position.dist2(&cb.position) > reach * reach {
                    continue;
                }
                if trait_l1(&ca.mean_traits, &cb.mean_traits) > cc.tissue_kin * adhesion_mult(ca, cb) {
                    continue;
                }
                set.insert(key);
                bonds.push(key);
            }
        }
        bonds.sort_unstable();
        bonds.dedup();
        // 3. graphe de liens -> voisinage + union-find (id de tissu = plus petit id du groupe).
        for &(a, b) in &bonds {
            if let (Some(&ka), Some(&kb)) = (idx.get(&a), idx.get(&b)) {
                neigh[ka].push(kb);
                neigh[kb].push(ka);
                let (ra, rb) = (find(&mut uf, ka), find(&mut uf, kb));
                if ra != rb {
                    let (lo, hi) = if world.cells[ra].id <= world.cells[rb].id { (ra, rb) } else { (rb, ra) };
                    uf[hi] = lo;
                }
            }
        }
        world.cell_bonds = bonds;
    } else {
        if !world.cell_bonds.is_empty() {
            world.cell_bonds.clear();
        }
        for ii in 0..n {
            for jj in (ii + 1)..n {
                let (a, b) = (order[ii], order[jj]);
                let (ca, cb) = (&world.cells[a], &world.cells[b]);
                let reach = (ca.radius + cb.radius) * cc.tissue_reach.max(0.1);
                if ca.position.dist2(&cb.position) > reach * reach {
                    continue;
                }
                if trait_l1(&ca.mean_traits, &cb.mean_traits) > cc.tissue_kin * adhesion_mult(ca, cb) {
                    continue;
                }
                neigh[a].push(b);
                neigh[b].push(a);
                let (ra, rb) = (find(&mut uf, a), find(&mut uf, b));
                if ra != rb {
                    // rattacher a la racine dont la cellule a le plus petit id : id de tissu stable
                    let (lo, hi) = if world.cells[ra].id <= world.cells[rb].id { (ra, rb) } else { (rb, ra) };
                    uf[hi] = lo;
                }
            }
        }
    }
    let mut sizes: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
    for k in 0..n {
        let r = find(&mut uf, k);
        *sizes.entry(r).or_insert(0) += 1;
    }
    for k in 0..n {
        let r = find(&mut uf, k);
        world.cells[k].tissue = if sizes[&r] >= cc.tissue_min.max(2) {
            Some(world.cells[r].id)
        } else {
            None
        };
    }
    world.tissues_alive = {
        let mut ids: std::collections::HashSet<u32> = std::collections::HashSet::new();
        for c in &world.cells {
            if let Some(id) = c.tissue {
                ids.insert(id);
            }
        }
        ids.len() as u32
    };

    // Place dans le tissu : combien de voisines du meme tissu adherent a cette cellule. C'est
    // le seul indicateur de position (centre / bord) ; les roles en decoulent sans regle qui
    // les nomme. Une cellule isolee ou hors tissu retombe a 0.
    for k in 0..n {
        world.cells[k].tissue_bonds = if world.cells[k].tissue.is_some() {
            let ti = world.cells[k].tissue;
            neigh[k].iter().filter(|&&j| world.cells[j].tissue == ti).count().min(255) as u8
        } else {
            0
        };
    }

    // Abri du tissu (`tissue_shelter`) : les cellules de bord (exposees, elles captent au
    // contact du dehors) reversent une part `shelter_feed` de leur surplus vers les cellules
    // interieures du meme tissu. Flux le long du gradient d'entassement, conserve tick a tick,
    // sans RNG, ordre des id. Le coeur, a l'abri de la predation et nourri par le bord, accumule
    // ses membres et finit par se diviser (la lignee qui se reproduit) ; le bord encaisse et
    // tient la frontiere. La division du travail germinal / somatique emerge de la seule
    // geometrie : aucune regle ne nomme un role.
    if cc.tissue_shelter && cc.shelter_feed > 0.0 && world.tissues_alive > 0 {
        let feed = cc.shelter_feed.clamp(0.0, 0.9);
        let sb = cc.shelter_bonds.max(1) as u8;
        // ne jamais ponctionner un membre en dessous d'une marge au-dessus de la famine
        let starve_floor = starve_at * 2.0;
        let cslot: std::collections::HashMap<u32, usize> =
            world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
        let mut cmem: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, e) in world.entities.iter().enumerate() {
            if let Some(id) = e.cell_id {
                if let Some(&k) = cslot.get(&id) {
                    cmem[k].push(i);
                }
            }
        }
        // par tissu : cellules de bord (donneuses) et cellules interieures (receveuses)
        let mut tissues: std::collections::HashMap<u32, (Vec<usize>, Vec<usize>)> =
            std::collections::HashMap::new();
        for k in 0..n {
            if let Some(tid) = world.cells[k].tissue {
                let e = tissues.entry(tid).or_default();
                if world.cells[k].tissue_bonds >= sb {
                    e.1.push(k); // interieure : receveuse
                } else {
                    e.0.push(k); // bord : donneuse
                }
            }
        }
        let mut tids: Vec<u32> = tissues.keys().copied().collect();
        tids.sort_unstable();
        let mut delta = vec![0.0f32; world.entities.len()];
        for tid in tids {
            let (rim, core) = &tissues[&tid];
            if core.is_empty() || rim.is_empty() {
                continue;
            }
            let mut pot = 0.0f32;
            for &k in rim {
                for &i in &cmem[k] {
                    let surplus = (world.entities[i].energy - starve_floor).max(0.0);
                    pot += surplus * feed;
                }
            }
            if pot <= 0.0 {
                continue;
            }
            let mut receivers = 0usize;
            for &k in core {
                receivers += cmem[k].len();
            }
            if receivers == 0 {
                continue;
            }
            let per = pot / receivers as f32;
            for &k in rim {
                for &i in &cmem[k] {
                    let surplus = (world.entities[i].energy - starve_floor).max(0.0);
                    delta[i] -= surplus * feed;
                }
            }
            for &k in core {
                for &i in &cmem[k] {
                    delta[i] += per;
                }
            }
        }
        for (i, d) in delta.into_iter().enumerate() {
            if d != 0.0 {
                world.entities[i].energy =
                    (world.entities[i].energy + d).clamp(0.0, energy_ceiling);
            }
        }
    }

    // Ordre du tissu : ordre orientationnel a 6 plis (psi6) des centroides de cellules, moyenne
    // sur toutes les cellules en tissu qui ont au moins 3 voisines. `1` = pavage hexagonal
    // parfait, `0` = desordre (scenario KTHNY / phase hexatique). L'activite cellulaire fait
    // monter la "temperature effective" et fondre l'ordre.
    let mut psi_sum = 0.0f64;
    let mut psi_n = 0usize;
    // par tissu : (somme psi6, cellules comptees) -> pour marquer les nappes SCELLEES.
    let mut psi_by_tissue: std::collections::HashMap<u32, (f64, u32)> =
        std::collections::HashMap::new();
    for k in 0..n {
        if world.cells[k].tissue.is_none() || neigh[k].len() < 3 {
            continue;
        }
        let pk = world.cells[k].position;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for &j in &neigh[k] {
            let pj = world.cells[j].position;
            let ang = ((pj.y - pk.y) as f64).atan2((pj.x - pk.x) as f64);
            re += (6.0 * ang).cos();
            im += (6.0 * ang).sin();
        }
        let psi_k = (re * re + im * im).sqrt() / neigh[k].len() as f64;
        psi_sum += psi_k;
        psi_n += 1;
        let e = psi_by_tissue.entry(world.cells[k].tissue.unwrap()).or_insert((0.0, 0));
        e.0 += psi_k;
        e.1 += 1;
    }
    world.tissue_order = if psi_n > 0 { (psi_sum / psi_n as f64) as f32 } else { 0.0 };

    // Nappe scellee (0.0.2, `[cells] epithelium_shield`) : un tissu ordonne (psi6 moyen >=
    // `shield_order`) et assez grand (>= `shield_cells` cellules) fait rempart. On marque chaque
    // cellule ; la phase predation lit `Cell.sealed` (etat du tick precedent) et epargne la proie.
    // Le nombre de cellules qui comptent au psi6 (>= 3 voisines) sert de taille : une chaine ou
    // un bord effiloche ne scelle pas.
    let sealed_tids: std::collections::HashSet<u32> = if cc.epithelium_shield {
        psi_by_tissue
            .iter()
            .filter(|(_, &(s, c))| {
                c >= cc.shield_cells.max(2) && (s / c as f64) as f32 >= cc.shield_order
            })
            .map(|(&tid, _)| tid)
            .collect()
    } else {
        std::collections::HashSet::new()
    };
    for c in world.cells.iter_mut() {
        c.sealed = c.tissue.is_some_and(|t| sealed_tids.contains(&t));
    }

    // Adhesion : les cellules d'un meme tissu se rapprochent jusqu'au contact. C'est cette
    // traction qui fait emerger le pavage. Poussee accumulee par entite puis appliquee, bornee,
    // gardee dans la grille. Sequentiel, sans RNG, ordre des id. Avec `tissue_bond`, `neigh`
    // est le graphe de liens et la raideur est celle du ressort de lien (plus ferme) : un lien
    // etire tire vraiment ses deux cellules vers le contact.
    let pull = if cc.tissue_bond { cc.bond_stiffness.max(0.0) } else { cc.tissue_pull.max(0.0) };
    if pull <= 0.0 {
        return;
    }
    let slot: std::collections::HashMap<u32, usize> =
        world.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
    let mut mem: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, e) in world.entities.iter().enumerate() {
        if let Some(id) = e.cell_id {
            if let Some(&k) = slot.get(&id) {
                mem[k].push(i);
            }
        }
    }
    let snap: Vec<(Position, f32)> = world.cells.iter().map(|c| (c.position, c.radius)).collect();
    let mut push = vec![(0.0f32, 0.0f32); world.entities.len()];
    for a in 0..n {
        if world.cells[a].tissue.is_none() {
            continue;
        }
        for &b in &neigh[a] {
            if b <= a || world.cells[b].tissue != world.cells[a].tissue {
                continue;
            }
            let (pa, ra) = snap[a];
            let (pb, rb) = snap[b];
            let dx = pb.x - pa.x;
            let dy = pb.y - pa.y;
            let d = (dx * dx + dy * dy).sqrt().max(1e-3);
            let touch = (ra + rb) * 0.98; // cible : membranes qui s'effleurent
            let gap = d - touch;
            if gap <= 0.0 {
                continue;
            }
            let mag = (gap * pull).min(0.35);
            let (nx, ny) = (dx / d, dy / d);
            for &i in &mem[a] {
                push[i].0 += nx * mag;
                push[i].1 += ny * mag;
            }
            for &i in &mem[b] {
                push[i].0 -= nx * mag;
                push[i].1 -= ny * mag;
            }
        }
    }
    let (gw, gh) = (space.width as f32, space.height as f32);
    for (i, (px, py)) in push.into_iter().enumerate() {
        if px == 0.0 && py == 0.0 {
            continue;
        }
        let e = &mut world.entities[i];
        e.position.x = (e.position.x + px.clamp(-1.0, 1.0)).clamp(0.0, gw - 1e-3);
        e.position.y = (e.position.y + py.clamp(-1.0, 1.0)).clamp(0.0, gh - 1e-3);
    }
}

/// Detection d'amas coherents de parents, avec persistance. Emet `CellFormed`.
fn cell_detect(
    world: &mut WorldState,
    cc: &crate::config::CellsCfg,
    space: &Space,
    t: u64,
    events: &mut Vec<Event>,
) {
    // 4a. Absorption : une entite libre proche et parente d'une cellule existante la rejoint.
    if !world.cells.is_empty() {
        let snap: Vec<(u32, Position, [f32; N_TRAITS], f32)> = world
            .cells
            .iter()
            .map(|c| (c.id, c.position, c.mean_traits, c.radius))
            .collect();
        for i in 0..world.entities.len() {
            if world.entities[i].cell_id.is_some() {
                continue;
            }
            let (p, a) = {
                let e = &world.entities[i];
                (e.position, e.genome.traits.as_array())
            };
            for &(cid, cpos, ctraits, crad) in &snap {
                let reach = (crad + cc.link_dist).max(cc.link_dist);
                if (p.x - cpos.x).powi(2) + (p.y - cpos.y).powi(2) <= reach * reach
                    && trait_l1(&a, &ctraits) <= cc.kin_dist
                {
                    world.entities[i].cell_id = Some(cid);
                    if let Some(c) = world.cell_mut(cid) {
                        c.member_count += 1;
                    }
                    break;
                }
            }
        }
    }

    // 4b. Composantes connexes sur les entites encore libres.
    let free: Vec<usize> = (0..world.entities.len())
        .filter(|&i| world.entities[i].cell_id.is_none())
        .collect();
    if (free.len() as u32) < cc.min_members {
        world.watch.cell_pending.clear();
        return;
    }
    let mut slot: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (fi, &i) in free.iter().enumerate() {
        slot.insert(i, fi);
    }
    let index = SpatialHash::build(&world.entities, space, cc.link_dist.max(0.5));
    let mut uf = UnionFind::new(free.len());
    let link2 = cc.link_dist * cc.link_dist;
    for (fi, &i) in free.iter().enumerate() {
        let pi = world.entities[i].position;
        let ai = world.entities[i].genome.traits.as_array();
        index.for_each_neighbor(pi, cc.link_dist, |nidx| {
            let j = nidx as usize;
            if let Some(&fj) = slot.get(&j) {
                if fj > fi {
                    let pj = world.entities[j].position;
                    if pi.dist2(&pj) <= link2
                        && trait_l1(&ai, &world.entities[j].genome.traits.as_array()) <= cc.kin_dist
                    {
                        uf.union(fi, fj);
                    }
                }
            }
        });
    }
    let mut groups: std::collections::BTreeMap<usize, Vec<usize>> = std::collections::BTreeMap::new();
    for (fi, &orig) in free.iter().enumerate() {
        let r = uf.find(fi);
        groups.entry(r).or_default().push(orig);
    }

    // Candidats : taille, cohesion moyenne, dispersion.
    let match_r2 = (cc.link_dist * 3.0) * (cc.link_dist * 3.0);
    let old = std::mem::take(&mut world.watch.cell_pending);
    let mut new_pending: Vec<(Position, u16)> = Vec::new();
    for (_, g) in groups {
        if (g.len() as u32) < cc.min_members {
            continue;
        }
        let n = g.len() as f32;
        let (mut cx, mut cy, mut coh) = (0.0f32, 0.0f32, 0.0f32);
        for &i in &g {
            let e = &world.entities[i];
            cx += e.position.x;
            cy += e.position.y;
            coh += e.genome.traits.cohesion;
        }
        cx /= n;
        cy /= n;
        coh /= n;
        if coh < cc.min_cohesion {
            continue;
        }
        let mut sp = 0.0f32;
        for &i in &g {
            let p = world.entities[i].position;
            sp += ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt();
        }
        sp /= n;
        if sp > cc.max_spread {
            continue;
        }

        // Persistance : ce candidat continue-t-il un candidat des controles precedents ?
        let cpos = Position { x: cx, y: cy };
        let mut streak = 1u16;
        let mut best = f32::MAX;
        for &(opos, ostreak) in &old {
            let d2 = (cx - opos.x).powi(2) + (cy - opos.y).powi(2);
            if d2 <= match_r2 && d2 < best {
                best = d2;
                streak = ostreak.saturating_add(1);
            }
        }
        if streak >= cc.persist_checks.max(1) {
            let cid = world.next_cell_id;
            world.next_cell_id += 1;
            let mut mean = [0.0f32; N_TRAITS];
            let mut adh_sum = 0.0f32;
            for &i in &g {
                world.entities[i].cell_id = Some(cid);
                let a = world.entities[i].genome.traits.as_array();
                for j in 0..N_TRAITS {
                    mean[j] += a[j];
                }
                adh_sum += world.entities[i].genome.structural.adhesion;
            }
            for m in mean.iter_mut() {
                *m /= n;
            }
            let adh_mean = adh_sum / n;
            let mut rad = 0.0f32;
            for &i in &g {
                let p = world.entities[i].position;
                rad += ((p.x - cx).powi(2) + (p.y - cy).powi(2)).sqrt();
            }
            rad /= n;
            world.cells.push(Cell {
                id: cid,
                formed_tick: t,
                position: cpos,
                radius: rad,
                member_count: g.len() as u32,
                genome_key: genome_key_arr(&mean),
                mean_traits: mean,
                elongation: 1.0,
                parent_cell: None,
                tissue: None,
                tissue_bonds: 0,
                sealed: false,
                organism: None,
                mean_adhesion: adh_mean,
            });
            world.cells_formed_total += 1;
            emit(
                events,
                &mut world.next_event_seq,
                t,
                EventKind::CellFormed { cell: cid, size: g.len() as u32 },
            );
        } else {
            new_pending.push((cpos, streak));
        }
    }
    world.watch.cell_pending = new_pending;
}

/// Phase 5c : cognition (0.0.3, tranche 1). Sequentiel, sans RNG.
///
/// - Un agent existant : sa memoire decroit ; si son dernier choc est plus recent que son
///   dernier souvenir, il l'enregistre ; s'il n'a plus aucun souvenir depuis longtemps, il
///   retombe entite de fond (`AgentLapsed`).
/// - Une entite non agent qui percoit assez, a vecu assez et vient de subir un choc
///   s'eveille (`AgentAwoke`) : elle gagne un `Mind` avec un premier souvenir.
fn cognition_phase(
    world: &mut WorldState,
    cfg: &SimConfig,
    t: u64,
    index: &SpatialHash,
    nerve_r2_mult: &[f32],
    events: &mut Vec<Event>,
) {
    let cg = &cfg.cognition;
    let lifespan_mean = cfg.lifecycle.lifespan_ticks_mean as f32;
    let max_mem = cg.max_memories as usize;
    let repro_threshold = cfg.reproduction.energy_threshold.max(0.01);
    let support_cap = cfg.cohesion.support_cap.max(0.001);
    let fear_inv2s2 = 1.0 / (2.0 * cg.fear_radius.max(0.5) * cg.fear_radius.max(0.5));
    let social_r = cg.social_radius.max(0.5);
    let social_r2 = social_r * social_r;
    // Le controle social est couteux (une requete spatiale par agent) : tous les N ticks.
    let social_tick = cg.social_check_every.max(1);
    let do_social = t % social_tick == 0;
    // Voix (0.0.4) : perception des alarmes. Balayage lineaire, `signals` est borne.
    let alarm_r2 = cfg.voice.signal_radius.max(0.1) * cfg.voice.signal_radius.max(0.1);
    let alarm_fear = cfg.voice.alarm_fear;
    let hear_alarms = alarm_fear > 0.0 && !world.signals.is_empty();

    for i in 0..world.entities.len() {
        // Age attendu de l'entite, pour le seuil de maturite cognitive.
        let expected = lifespan_mean * (0.5 + world.entities[i].genome.traits.lifespan);
        let age = world.entities[i].age_ticks;
        let perception = world.entities[i].genome.traits.perception;
        let shock = world.entities[i].last_shock;
        let (epos, eenergy, esupport) = {
            let e = &world.entities[i];
            (e.position, e.energy, e.colony_support)
        };

        // Une alarme a portee ? (avant le `&mut mind`). On ignore la sienne : `born == t` et
        // meme position exacte serait un cas limite, mais l'alarme d'un autre a la meme case
        // compte, c'est voulu (la panique se partage).
        let is_agent = world.entities[i].mind.is_some();
        let plain_near = hear_alarms
            && is_agent
            && world
                .signals
                .iter()
                .any(|s| epos.dist2(&s.pos) <= alarm_r2 && !(s.born == t && s.pos == epos));
        let my_r2 = match nerve_r2_mult.get(i) {
            Some(&m) => alarm_r2 * m,
            None => alarm_r2,
        };
        let near_alarm = if plain_near {
            true
        } else if my_r2 > alarm_r2 && hear_alarms && is_agent {
            let relayed = world
                .signals
                .iter()
                .any(|s| epos.dist2(&s.pos) <= my_r2 && !(s.born == t && s.pos == epos));
            if relayed {
                world.nerve_signals_relayed += 1;
            }
            relayed
        } else {
            false
        };

        // Agents proches : collectes avant le `&mut mind` (conflit de borrow sinon).
        let mut near_agents: Vec<EntityId> = Vec::new();
        if do_social && world.entities[i].mind.is_some() {
            index.for_each_neighbor(epos, social_r, |nidx| {
                let j = nidx as usize;
                if j == i {
                    return;
                }
                let nb = &world.entities[j];
                if nb.mind.is_some() && epos.dist2(&nb.position) <= social_r2 {
                    near_agents.push(nb.id);
                }
            });
        }

        if let Some(mind) = world.entities[i].mind.as_deref_mut() {
            mind.decay_and_prune(cg.memory_decay, cg.memory_eps);

            // Nouveau souvenir : le dernier choc est-il posterieur au dernier souvenir enregistre ?
            if let Some(s) = shock {
                let last_formed = mind.episodic.iter().map(|m| m.formed_tick).max().unwrap_or(0);
                if s.tick > last_formed || mind.episodic.is_empty() {
                    mind.record(
                        Memory {
                            formed_tick: s.tick,
                            place: s.place,
                            kind: s.kind(),
                            event_seq: None,
                            strength: 1.0,
                        },
                        max_mem,
                        cg.memory_merge_dist,
                    );
                }
            }

            // Besoins : faim, peur, solitude. Ponderent le comportement en phase 2/3.
            let near_aversive = mind
                .episodic
                .iter()
                .filter(|m| m.kind.is_aversive())
                .map(|m| m.strength * (-epos.dist2(&m.place) * fear_inv2s2).exp())
                .fold(0.0f32, f32::max);
            let shock_peril_recent = shock
                .is_some_and(|s| s.peril && t.saturating_sub(s.tick) < cg.fear_shock_window);
            let solitude = 1.0 - (esupport / support_cap).clamp(0.0, 1.0);
            mind.needs.update(
                cg.hunger_relief,
                cg.fear_relief,
                eenergy / repro_threshold,
                near_aversive,
                shock_peril_recent,
                solitude,
            );
            // Voix : entendre une alarme fait sursauter la peur, sans former de souvenir.
            // Transitoire : elle redescend ensuite via `fear_relief`.
            if near_alarm {
                mind.needs.fear = mind.needs.fear.max(alarm_fear);
            }

            // Souvenirs sociaux : qui etait la, et l'agent se sentait-il bien (tous les N ticks).
            if do_social {
                mind.decay_social(cg.social_decay, cg.social_eps);
                if !near_agents.is_empty() {
                    let mood = if mind.needs.fear > 0.5 {
                        -1.0
                    } else if mind.needs.fear < 0.2 && mind.needs.hunger < 0.4 {
                        1.0
                    } else {
                        0.0
                    };
                    for &oid in &near_agents {
                        mind.touch_tie(oid, cg.social_fam_gain, mood);
                    }
                }
            }

            // Retombee : plus aucun souvenir, et l'agent a passe le delai de grace et de latence.
            let since_awoke = t.saturating_sub(mind.awoke_tick);
            if mind.episodic.is_empty()
                && since_awoke > cg.grace_ticks
                && since_awoke > cg.lapse_ticks
            {
                let id = world.entities[i].id;
                world.entities[i].mind = None;
                emit(events, &mut world.next_event_seq, t, EventKind::AgentLapsed { entity: id });
            }
            continue;
        }

        // Eveil.
        if let Some(s) = shock {
            if perception < cg.perception_min || (age as f32) < cg.age_min_frac * expected {
                continue;
            }
            let id = world.entities[i].id;
            world.entities[i].mind = Some(Box::new(Mind::new(
                t,
                Memory {
                    formed_tick: s.tick,
                    place: s.place,
                    kind: s.kind(),
                    event_seq: None,
                    strength: 1.0,
                },
            )));
            emit(events, &mut world.next_event_seq, t, EventKind::AgentAwoke { entity: id });
        }
    }
}

/// Ordre des modes, pour recoder un `u8` en `BehaviorMode` a l'application.
const MODE_FROM_CODE: [BehaviorMode; 5] = [
    BehaviorMode::Forage,
    BehaviorMode::Flee,
    BehaviorMode::Join,
    BehaviorMode::SeekBounty,
    BehaviorMode::Wander,
];

/// Contexte de decision d'un agent, monte dans la closure de la phase 2/3, lu par
/// `blend_target`. Tout est deja resolu (personnalite, besoins).
struct Decide<'a> {
    pos: Position,
    /// Cible de chimiotaxie (ou pas au hasard si rien de mieux qu'ici).
    forage: Position,
    /// `true` si la chimiotaxie a trouve mieux qu'ici (sinon `forage` est un pas au hasard).
    has_food: bool,
    /// Position de l'appel de nourriture le plus proche a portee (Voix tranche 2), si entendu.
    call: Option<Position>,
    /// A quel point l'appel inflechit la cible.
    call_pull: f32,
    /// Centre de masse des siens proches, si l'agent en a autour.
    kin: Option<Position>,
    /// Position de l'ami le plus familier (0.0.3, tranche 7), si trouve.
    friend: Option<Position>,
    /// Part du glissement de solitude qui vise l'ami plutot que le centre des siens.
    friend_pull: f32,
    /// Energie / seuil de reproduction.
    energy_frac: f32,
    caution: f32,
    curiosity: f32,
    needs: Needs,
    /// `needs_weight` : met a l'echelle l'effet de la peur et de la solitude.
    nw: f32,
    fear_gain: f32,
    social_pull: f32,
    mem_weight: f32,
    mem_radius: f32,
    sw: f32,
    sh: f32,
    episodic: &'a [Memory],
}

impl Decide<'_> {
    fn clamp(&self, p: Position) -> Position {
        Position {
            x: p.x.clamp(0.0, self.sw - 0.001),
            y: p.y.clamp(0.0, self.sh - 0.001),
        }
    }
}

/// Comportement d'un agent : un melange de forces (chimiotaxie, memoire aversive qui
/// repousse, aubaine qui attire, glissement vers les siens si isole). Renvoie la cible ET
/// le mode qui a **domine** la decision (0.0.3, tranche 6 : lecture, pas un changement de
/// comportement). Le mode rend la biographie lisible : « a fui plutot que de manger ».
fn blend_target(c: &Decide) -> (Position, BehaviorMode) {
    let base = c.forage;
    let default_mode = if c.has_food { BehaviorMode::Forage } else { BehaviorMode::Wander };
    // Voix tranche 2 : un appel de nourriture entendu attire la cible vers lui, d'autant plus
    // que l'agent a faim. Ce n'est pas un souvenir : ca marche meme sans memoire.
    let call_shift = |tgt: Position| -> Position {
        let Some(call) = c.call else { return tgt };
        let hungry = if c.nw > 0.0 {
            c.needs.hunger.clamp(0.0, 1.0)
        } else {
            (1.0 - c.energy_frac).clamp(0.0, 1.0)
        };
        let w = (c.call_pull * (0.3 + 0.7 * hungry)).clamp(0.0, 0.7);
        c.clamp(Position {
            x: tgt.x + (call.x - tgt.x) * w,
            y: tgt.y + (call.y - tgt.y) * w,
        })
    };
    if c.episodic.is_empty() {
        let tgt = call_shift(base);
        let mode = if c.call.is_some() { BehaviorMode::SeekBounty } else { default_mode };
        return (tgt, mode);
    }
    let nw = c.nw;
    let drive = if nw > 0.0 {
        (1.0 - c.needs.hunger).clamp(0.0, 1.0)
    } else {
        ((c.energy_frac - 0.5) / 0.5).clamp(0.0, 1.0)
    };
    let fear = c.needs.fear * nw;
    let gate_av = drive.max(fear);
    if drive <= 0.0 && gate_av <= 0.0 {
        return (base, default_mode);
    }
    let av_amp = 1.0 + fear * c.fear_gain;
    let inv2s2 = 1.0 / (2.0 * c.mem_radius * c.mem_radius);
    // Contributions separees, pour lire ensuite laquelle a pese.
    let (mut avx, mut avy) = (0.0f32, 0.0f32); // aversif (repousse)
    let (mut bx, mut by) = (0.0f32, 0.0f32); // aubaine (attire)
    for m in c.episodic.iter() {
        let dx = c.pos.x - m.place.x;
        let dy = c.pos.y - m.place.y;
        let d2 = dx * dx + dy * dy;
        let d = d2.sqrt().max(1e-3);
        let k = m.strength * (-d2 * inv2s2).exp();
        if m.kind.is_aversive() {
            let s = c.caution * av_amp * if nw > 0.0 { gate_av } else { 1.0 };
            avx += s * k * dx / d;
            avy += s * k * dy / d;
        } else {
            let s = c.curiosity * if nw > 0.0 { drive } else { 1.0 };
            bx += -s * k * dx / d;
            by += -s * k * dy / d;
        }
    }
    let (vx, vy) = (avx + bx, avy + by);
    let mag = (vx * vx + vy * vy).sqrt();
    let mut tgt = if mag < 1e-4 {
        base
    } else {
        let b = if nw > 0.0 { mag.clamp(0.0, 1.0) } else { mag.clamp(0.0, 1.0) * drive };
        let w = (c.mem_weight * b).clamp(0.0, 0.9);
        let shift = c.mem_radius * w;
        c.clamp(Position { x: base.x + vx / mag * shift, y: base.y + vy / mag * shift })
    };
    // Glissement vers les siens si isole ; en partie vers l'ami familier si l'agent en a un.
    let mut soc_disp = 0.0f32;
    if nw > 0.0 && c.needs.solitude > 0.0 {
        if let Some(kin) = c.kin {
            let anchor = match c.friend {
                Some(f) => Position {
                    x: kin.x + (f.x - kin.x) * c.friend_pull.clamp(0.0, 1.0),
                    y: kin.y + (f.y - kin.y) * c.friend_pull.clamp(0.0, 1.0),
                },
                None => kin,
            };
            let w_soc = (c.needs.solitude * c.social_pull * nw).clamp(0.0, 0.8);
            let before = tgt;
            tgt = c.clamp(Position {
                x: tgt.x + (anchor.x - tgt.x) * w_soc,
                y: tgt.y + (anchor.y - tgt.y) * w_soc,
            });
            soc_disp = before.dist2(&tgt).sqrt();
        }
    }
    // Voix : l'appel de nourriture, applique en dernier, compose avec le reste.
    let pre_call = tgt;
    tgt = call_shift(tgt);
    let call_disp = pre_call.dist2(&tgt).sqrt();
    // Lecture du mode : la force qui a le plus deplace la cible loin de la nourriture.
    let av_m = (avx * avx + avy * avy).sqrt();
    let bo_m = (bx * bx + by * by).sqrt();
    let mem_shift = if mag < 1e-4 { 0.0 } else { c.mem_radius * (c.mem_weight * mag.clamp(0.0, 1.0)).clamp(0.0, 0.9) };
    if call_disp > 1.0 && call_disp >= mem_shift && call_disp >= soc_disp {
        return (tgt, BehaviorMode::SeekBounty);
    }
    let mode = if soc_disp > 1.0 && soc_disp >= mem_shift {
        BehaviorMode::Join
    } else if mem_shift > 1.0 && av_m > bo_m {
        BehaviorMode::Flee
    } else if mem_shift > 1.0 && bo_m > av_m {
        BehaviorMode::SeekBounty
    } else {
        default_mode
    };
    (tgt, mode)
}

/// Distance L1 entre deux traits, en unites de trait (un cran = 0.25).
fn trait_l1(a: &[f32; N_TRAITS], b: &[f32; N_TRAITS]) -> f32 {
    let mut d = 0.0f32;
    for i in 0..N_TRAITS {
        d += (a[i] - b[i]).abs();
    }
    d
}

/// Rayon moyen des positions autour de leur centroide (dispersion d'un groupe).
fn group_spread(ps: &[Position]) -> f32 {
    if ps.is_empty() {
        return 0.0;
    }
    let n = ps.len() as f32;
    let (mut mx, mut my) = (0.0f32, 0.0f32);
    for p in ps {
        mx += p.x;
        my += p.y;
    }
    mx /= n;
    my /= n;
    let mut d = 0.0f32;
    for p in ps {
        d += ((p.x - mx).powi(2) + (p.y - my).powi(2)).sqrt();
    }
    d / n
}

/// Forme d'un nuage de points : `(elongation, axe principal)`. `elongation` = racine du
/// rapport des deux valeurs propres de la covariance (1 = rond, >1 = etire). L'axe principal
/// est le vecteur unitaire de la plus grande variance. Deterministe (2x2 symetrique ferme).
fn cloud_shape(offsets: &[(f32, f32)]) -> (f32, (f32, f32)) {
    let n = offsets.len() as f32;
    if n < 2.0 {
        return (1.0, (1.0, 0.0));
    }
    let (mut sxx, mut syy, mut sxy) = (0.0f32, 0.0f32, 0.0f32);
    for &(x, y) in offsets {
        sxx += x * x;
        syy += y * y;
        sxy += x * y;
    }
    sxx /= n;
    syy /= n;
    sxy /= n;
    // valeurs propres de [[sxx, sxy], [sxy, syy]]
    let tr = sxx + syy;
    let det = sxx * syy - sxy * sxy;
    let disc = (tr * tr * 0.25 - det).max(0.0).sqrt();
    let l1 = tr * 0.5 + disc; // grande
    let l2 = (tr * 0.5 - disc).max(1e-6); // petite
    let elong = (l1 / l2).sqrt();
    // vecteur propre de l1 : (sxy, l1 - sxx) (ou (l1 - syy, sxy) si sxy ~ 0)
    let (mut ax, mut ay) = if sxy.abs() > 1e-6 {
        (sxy, l1 - sxx)
    } else if sxx >= syy {
        (1.0, 0.0)
    } else {
        (0.0, 1.0)
    };
    let m = (ax * ax + ay * ay).sqrt().max(1e-6);
    ax /= m;
    ay /= m;
    (elong, (ax, ay))
}

/// Distance L1 entre deux cles de genome (traits de corps), en unites de trait (un cran = 0.25).
fn key_distance(a: u16, b: u16) -> f32 {
    let mut d = 0.0f32;
    for i in 0..SPECIES_TRAITS {
        let qa = ((a >> (2 * i)) & 3) as i32;
        let qb = ((b >> (2 * i)) & 3) as i32;
        d += (qa - qb).unsigned_abs() as f32 * 0.25;
    }
    d
}

/// Detecteurs de veille. Deterministe : itere sur des `BTreeMap` et des plages fixes.
fn run_watchers(world: &mut WorldState, cfg: &SimConfig, t: u64, events: &mut Vec<Event>) {
    use std::collections::BTreeMap;
    let pop = world.entities.len() as u32;
    let wc = &cfg.watch;

    // -- Paliers de population, premier franchissement a la hausse --
    const LEVELS: [u32; 8] = [10, 25, 50, 100, 250, 500, 1000, 5000];
    for &lvl in LEVELS.iter() {
        if pop >= lvl && world.watch.milestone_hi < lvl {
            world.watch.milestone_hi = lvl;
            emit(events, &mut world.next_event_seq, t, EventKind::PopulationMilestone { level: lvl });
        }
    }

    // -- Effondrement : perte d'une fraction de la population sur la fenetre --
    world.watch.pop_history.push(pop);
    let win = wc.crash_window_checks.max(1) as usize;
    while world.watch.pop_history.len() > win + 1 {
        world.watch.pop_history.remove(0);
    }
    if world.watch.pop_history.len() >= 2 {
        let past = world.watch.pop_history[0];
        if past >= 20 && pop < past {
            let drop = (past - pop) as f32 / past as f32;
            if drop >= wc.crash_drop_frac {
                let causes = world.watch.deaths_since_check.clone();
                emit_caused(
                    events,
                    &mut world.next_event_seq,
                    t,
                    EventKind::PopulationCrash { from: past, to: pop },
                    causes,
                );
                world.watch.pop_history.clear();
                world.watch.pop_history.push(pop);
            }
        }
    }

    // -- Lignees fondatrices vivantes --
    let (lin_now, _, _) = world.lineage_stats();
    if lin_now < world.watch.lineages {
        let alive: std::collections::BTreeSet<u16> =
            world.entities.iter().map(|e| e.genome.lineage).collect();
        for l in 0..world.watch.lineages {
            if !alive.contains(&l) {
                let causes: Vec<u64> = world
                    .watch
                    .last_death_seq_by_lineage
                    .get(&l)
                    .copied()
                    .into_iter()
                    .collect();
                emit_caused(
                    events,
                    &mut world.next_event_seq,
                    t,
                    EventKind::LineageExtinct { lineage: l },
                    causes,
                );
            }
        }
    }
    world.watch.lineages = lin_now;

    // -- Especes : groupes de genome distincts, nombreux, persistants, spatialement groupes --
    if pop >= wc.species_min_size.saturating_mul(2) {
        let mut counts: BTreeMap<u16, u32> = BTreeMap::new();
        let mut pos_by_key: BTreeMap<u16, Vec<Position>> = BTreeMap::new();
        for e in world.entities.iter() {
            let k = genome_key(&e.genome.traits);
            *counts.entry(k).or_insert(0) += 1;
            pos_by_key.entry(k).or_default().push(e.position);
        }
        // cle dominante : plus grand effectif, egalite tranchee par plus petite cle
        let dominant = counts
            .iter()
            .max_by(|x, y| x.1.cmp(y.1).then(y.0.cmp(x.0)))
            .map(|(k, _)| *k)
            .unwrap_or(0);

        // Basculement du genome dominant : l'evolution a deplace le centre de la population.
        // On exige que la nouvelle cle domine plusieurs controles d'affilee (elle peut
        // flotter pres d'une egalite), comme une espece doit tenir avant d'etre reconnue.
        let far_enough = dominant != world.watch.dominant_genome_key
            && dominant != 0
            && key_distance(dominant, world.watch.dominant_genome_key) >= wc.species_min_distance;
        if world.watch.dominant_genome_key == 0 || wc.genome_shift_persist_checks == 0 {
            // premiere fixation, ou detecteur coupe : on suit la cle sans jamais emettre.
            world.watch.dominant_genome_key = dominant;
            world.watch.dominant_shift_cand = 0;
            world.watch.dominant_shift_streak = 0;
        } else if far_enough {
            // il faut la MEME cle distincte, tenue plusieurs controles : un flottement entre
            // deux bins voisines, ou entre deux cles eloignees, ne compte pas.
            if dominant == world.watch.dominant_shift_cand {
                world.watch.dominant_shift_streak += 1;
            } else {
                world.watch.dominant_shift_cand = dominant;
                world.watch.dominant_shift_streak = 1;
            }
            if world.watch.dominant_shift_streak >= wc.genome_shift_persist_checks {
                let from = world.watch.dominant_genome_key;
                let (mg, _) = world.generation_stats();
                emit(
                    events,
                    &mut world.next_event_seq,
                    t,
                    EventKind::GenomeShift { from, to: dominant, generation: mg.round() as u32 },
                );
                world.watch.dominant_genome_key = dominant;
                world.watch.dominant_shift_cand = 0;
                world.watch.dominant_shift_streak = 0;
            }
        } else {
            world.watch.dominant_shift_cand = 0;
            world.watch.dominant_shift_streak = 0;
        }

        let mut still: BTreeMap<u16, u16> = BTreeMap::new();
        for (&k, &c) in counts.iter() {
            if k == dominant
                || c < wc.species_min_size
                || key_distance(k, dominant) < wc.species_min_distance
                || group_spread(&pos_by_key[&k]) > wc.species_max_spread
            {
                continue;
            }
            let streak = world.watch.species_streak.get(&k).copied().unwrap_or(0) + 1;
            still.insert(k, streak);
            if streak >= wc.species_persist_checks && !world.watch.species.contains_key(&k) {
                let sid = world.watch.next_species_id;
                world.watch.next_species_id += 1;
                world.watch.species.insert(k, sid);
                emit(events, &mut world.next_event_seq, t, EventKind::SpeciesEmerged { species: sid, size: c });
            }
        }
        world.watch.species_streak = still;
        let floor = (wc.species_min_size / 2).max(1);
        world
            .watch
            .species
            .retain(|k, _| counts.get(k).copied().unwrap_or(0) >= floor);
    }

    // La fenetre de morts a servi aux liens causaux de ce controle : on la vide.
    world.watch.deaths_since_check.clear();
}

/// Cible de recherche de nourriture : le centre de masse des ressources dans le rayon de
/// perception, pondere par la quantite et attenue par la distance.
///
/// C'est un point continu, pas un centre de case : le deplacement se fait donc dans toutes
/// les directions, pas seulement selon les axes de la grille. C'est aussi de la chimiotaxie,
/// une entite qui remonte un gradient de concentration. Renvoie `None` si rien de mieux
/// qu'ici : l'entite erre alors au hasard.
fn forage_target(
    res: &ResourceField,
    space: &Space,
    pos: Position,
    radius: f32,
) -> Option<Position> {
    let sw = space.width as usize;
    let w = space.width as i64 - 1;
    let h = space.height as i64 - 1;
    let x0 = ((pos.x - radius).floor() as i64).clamp(0, w) as usize;
    let x1 = ((pos.x + radius).ceil() as i64).clamp(0, w) as usize;
    let y0 = ((pos.y - radius).floor() as i64).clamp(0, h) as usize;
    let y1 = ((pos.y + radius).ceil() as i64).clamp(0, h) as usize;

    let here = res.cell[res.index(space, pos)];

    let mut wx = 0.0f32;
    let mut wy = 0.0f32;
    let mut wsum = 0.0f32;
    let mut best = here;
    for cy in y0..=y1 {
        for cx in x0..=x1 {
            let v = res.cell[cy * sw + cx];
            if v < 0.3 {
                continue;
            }
            let px = cx as f32 + 0.5;
            let py = cy as f32 + 0.5;
            let d2 = (px - pos.x) * (px - pos.x) + (py - pos.y) * (py - pos.y);
            let weight = v / (1.0 + d2 * 0.15);
            wx += px * weight;
            wy += py * weight;
            wsum += weight;
            if v > best {
                best = v;
            }
        }
    }

    if wsum <= 0.0 || best <= here + 0.15 {
        return None;
    }
    Some(Position { x: wx / wsum, y: wy / wsum })
}
