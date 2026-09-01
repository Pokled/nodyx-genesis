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

use crate::config::SimConfig;
use crate::entity::{Action, Entity, EntityId, Position};
use crate::event::{DeathCause, Event, EventKind, ReplicationFail};
use crate::genome::{Genome, N_TRAITS};
use crate::spatial::SpatialHash;
use crate::world::{Cell, ResourceField, Space, WorldState};

// -- Profilage optionnel : `GENESIS_PROFILE=1` accumule le temps par phase, `profile_dump`
//    l'affiche. Aucun cout quand la variable n'est pas mise (un booleen lu une fois).
mod prof {
    use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
    use std::time::Instant;
    pub static ON: AtomicBool = AtomicBool::new(false);
    static INIT: std::sync::Once = std::sync::Once::new();
    pub static NS: [AtomicU64; 9] = [
        AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
        AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
        AtomicU64::new(0),
    ];
    pub const NAMES: [&str; 9] = [
        "p1 regen", "p2/3 decide", "p4 move", "p5 metab", "p5b cells", "p6 life", "p7 repro",
        "p8b watch", "cap",
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
        for i in 0..9 {
            eprintln!("  {:<14} {:>8.1}", NAMES[i], NS[i].load(Ordering::Relaxed) as f64 / 1e6);
        }
    }
}
pub use prof::dump as profile_dump;

/// Fait avancer le monde d'un tick. Renvoie les evenements produits (seq non encore attribue).
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
        let max_cell = cfg.resources.max_per_cell;
        let regen_rate = cfg.resources.regen_rate * regen_every as f32;
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
    let plans: Vec<Option<(Position, f32)>> = entities_ref
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
            let forage = match forage_target(resources_ref, space_ref, pos, radius) {
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
            let target = if wsum > 0.0 && e.genome.traits.cohesion > 0.0 && coh_ref.pull_max > 0.0 {
                let sated = ((e.energy / repro_threshold - 0.8) / 0.4).clamp(0.0, 1.0);
                let gate = coh_ref.hunger_damp + (1.0 - coh_ref.hunger_damp) * sated;
                let w = (e.genome.traits.cohesion * coh_ref.pull_max * gate).clamp(0.0, 1.0);
                Position {
                    x: forage.x + (cx / wsum - forage.x) * w,
                    y: forage.y + (cy / wsum - forage.y) * w,
                }
            } else {
                forage
            };
            Some((target, support))
        })
        .collect();

    // Application : les plans sont alignes sur l'ordre de `entities`, aucune recherche.
    for (e, plan) in world.entities.iter_mut().zip(plans) {
        if let Some((target, support)) = plan {
            e.target = Some(target);
            e.colony_support = support;
        }
    }

    drop(_sp.take()); _sp = prof::Span::start(2);
    // -- Phase 4, mouvement. Chaque entite ne touche qu'elle-meme : parallele direct sur
    // le `Vec`, qui se decoupe parfaitement.
    let move_cost = cfg.metabolism.move_cost;
    let sw = space.width as f32;
    let sh = space.height as f32;
    world.entities.par_iter_mut().for_each(|e| {
        let Some(tg) = e.target else { return };
        let pos = e.position;
        let max_step = 0.3 + e.genome.traits.speed * 1.2;
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
    let energy_ceiling = cfg.reproduction.energy_threshold * 2.0;
    let support_cap = coh.support_cap.max(0.001);
    let base_burn = cfg.metabolism.base_burn;
    let eat_rate = cfg.metabolism.eat_rate;
    let strain_per_harvest = cfg.environment.strain_per_harvest;
    for i in 0..world.entities.len() {
        let (pos, burn, want0, restraint) = {
            let e = &world.entities[i];
            let restraint = (e.genome.traits.cohesion
                * (e.colony_support / support_cap).clamp(0.0, 1.0))
                .clamp(0.0, 1.0);
            (
                e.position,
                base_burn * (0.5 + e.genome.traits.metabolism),
                eat_rate * (0.5 + e.genome.traits.efficiency),
                restraint,
            )
        };
        let idx = world.resources.index(&space, pos);
        let want = want0 * (1.0 - restraint * coh.eat_restraint);
        let gain = want.min(world.resources.cell[idx]).max(0.0);
        world.resources.cell[idx] -= gain;
        world.resources.strain[idx] +=
            gain * strain_per_harvest * (1.0 - restraint * coh.strain_restraint);

        let e = &mut world.entities[i];
        e.energy = (e.energy - burn + gain).min(energy_ceiling);
        e.last_action = if gain > 0.05 { Action::Eat } else { Action::Forage };
    }

    drop(_sp.take()); _sp = prof::Span::start(4);
    // -- Phase 5b, cellules (0.0.2, tranche 2, etape 1). Entretien des cellules existantes
    //    (bilan, partage d'energie, departs, dissolution) chaque tick ; detection de
    //    nouvelles cellules tous les `check_every` ticks. Sequentiel, avant la mort pour que
    //    le partage d'energie puisse sauver un membre affame.
    cell_phase(world, cfg, &space, t, &mut events);

    drop(_sp.take()); _sp = prof::Span::start(5);
    // -- Phase 6, cycle de vie : vieillissement, mort par famine ou par age. Sequentiel :
    // travail par entite minuscule. Le retrait des morts et le depot des cadavres ecrivent
    // sur les cases, donc de toute facon sequentiels.
    let starve_at = cfg.lifecycle.starve_at;
    let lifespan_mean = cfg.lifecycle.lifespan_ticks_mean as f32;
    let age_curve = cfg.lifecycle.age_death_curve;
    let corpse_nut = cfg.environment.corpse_nutrients;
    let corpse_ret = cfg.environment.corpse_energy_return;
    let body_matter = cfg.bricks.body_matter;

    // Decision : `dead` reste dans l'ordre des id.
    let mut dead: Vec<(EntityId, DeathCause)> = Vec::new();
    for i in 0..world.entities.len() {
        let roll = world.rng.next_f32();
        let e = &world.entities[i];
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
        };
        if roll < p_death {
            dead.push((e.id, DeathCause::Age));
        }
    }

    for e in world.entities.iter_mut() {
        e.age_ticks += 1;
        e.cooldown = e.cooldown.saturating_sub(1);
    }

    // Depot des cadavres puis retrait, une seule passe de `retain`.
    for &(id, cause) in &dead {
        let dead_cell = world.get(id).and_then(|e| e.cell_id);
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
        }
        events.push(Event::now(t, EventKind::EntityDied { entity: id, cause }));
    }
    if !dead.is_empty() {
        let gone: std::collections::HashSet<EntityId> = dead.iter().map(|&(id, _)| id).collect();
        world.entities.retain(|e| !gone.contains(&e.id));
    }

    drop(_sp.take()); _sp = prof::Span::start(6);
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
            events.push(Event::now(
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Materials },
            ));
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
            events.push(Event::now(
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Environment },
            ));
            continue;
        }

        // 2) copie du genome, avec chance de mutation letale (tirages RNG dans divide).
        let parent_genome = world.get(a).unwrap().genome.clone();
        let child_genome =
            match Genome::divide(&parent_genome, a, &cfg.reproduction, &mut world.rng) {
                Some(g) => g,
                None => {
                    events.push(Event::now(
                        t,
                        EventKind::ReplicationFailed {
                            parent: a,
                            reason: ReplicationFail::LethalMutation,
                        },
                    ));
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
            events.push(Event::now(
                t,
                EventKind::ReplicationFailed { parent: a, reason: ReplicationFail::Materials },
            ));
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
        });
        events.push(Event::now(t, EventKind::EntityDivided { parent: a, child: cid }));
    }
    // Les nouveaux nes ont des id croissants, tous superieurs aux existants : un `push`
    // en fin de `Vec` garde le tri par id.
    for nb in newborns {
        let id = nb.id;
        world.entities.push(nb);
        world.births_total += 1;
        events.push(Event::now(t, EventKind::EntitySpawned { entity: id }));
    }

    drop(_sp.take()); _sp = prof::Span::start(7);
    // -- Phase 8b, veilleurs : detecteurs mecanises. Ils ne mutent que `world.watch` et
    //    produisent des evenements saillants (le materiau des chapitres). Jamais un `if`
    //    qui nomme le resultat (tranchee 7).
    if cfg.watch.interval_ticks > 0 && t % cfg.watch.interval_ticks == 0 {
        run_watchers(world, cfg, t, &mut events);
    }

    drop(_sp.take()); _sp = prof::Span::start(8);
    // -- Phase 8 (journal) et 9 (instantane) sont pilotees par l'appelant (CLI).

    // Garde-fou anti-cascade : on plafonne les evenements du tick, en gardant les plus
    // saillants (tri stable, donc deterministe).
    let event_cap = cfg.events.max_events_per_tick as usize;
    if events.len() > event_cap {
        events.sort_by(|a, b| b.salience.cmp(&a.salience));
        events.truncate(event_cap);
    }
    events
}

/// Quantifie un genome en une cle : chaque trait sur 2 bits (0..3), N_TRAITS traits.
fn genome_key(t: &crate::genome::Traits) -> u16 {
    let mut k = 0u16;
    for (i, &q) in t.quantized().iter().enumerate() {
        k |= (q as u16) << (2 * i);
    }
    k
}

/// Meme cle, a partir d'un tableau de traits moyens (cellule).
fn genome_key_arr(a: &[f32; N_TRAITS]) -> u16 {
    let mut k = 0u16;
    for (i, &v) in a.iter().enumerate() {
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
        for &i in idxs {
            let e = &world.entities[i];
            cx += e.position.x;
            cy += e.position.y;
            e_sum += e.energy;
            let a = e.genome.traits.as_array();
            for j in 0..N_TRAITS {
                mean[j] += a[j];
            }
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

        let c = &mut world.cells[k];
        c.position = Position { x: cx, y: cy };
        c.radius = rad;
        c.member_count = count;
        c.mean_traits = mean;
        c.genome_key = genome_key_arr(&mean);
    }
    if !dissolved.is_empty() {
        let gone: std::collections::HashSet<u32> = dissolved.iter().copied().collect();
        world.cells.retain(|c| !gone.contains(&c.id));
        for id in dissolved {
            world.cells_dissolved_total += 1;
            events.push(Event::now(t, EventKind::CellDissolved { cell: id }));
        }
    }

    // -- 4. Detection de nouvelles cellules, tous les `check_every` ticks.
    if cc.check_every == 0 || t % cc.check_every != 0 {
        return;
    }
    cell_detect(world, &cc, space, t, events);
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
            for &i in &g {
                world.entities[i].cell_id = Some(cid);
                let a = world.entities[i].genome.traits.as_array();
                for j in 0..N_TRAITS {
                    mean[j] += a[j];
                }
            }
            for m in mean.iter_mut() {
                *m /= n;
            }
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
            });
            world.cells_formed_total += 1;
            events.push(Event::now(
                t,
                EventKind::CellFormed { cell: cid, size: g.len() as u32 },
            ));
        } else {
            new_pending.push((cpos, streak));
        }
    }
    world.watch.cell_pending = new_pending;
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

/// Distance L1 entre deux cles de genome, en unites de trait (un cran = 0.25).
fn key_distance(a: u16, b: u16) -> f32 {
    let mut d = 0.0f32;
    for i in 0..N_TRAITS {
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
            events.push(Event::now(t, EventKind::PopulationMilestone { level: lvl }));
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
                events.push(Event::now(t, EventKind::PopulationCrash { from: past, to: pop }));
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
                events.push(Event::now(t, EventKind::LineageExtinct { lineage: l }));
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
                events.push(Event::now(t, EventKind::SpeciesEmerged { species: sid, size: c }));
            }
        }
        world.watch.species_streak = still;
        let floor = (wc.species_min_size / 2).max(1);
        world
            .watch
            .species
            .retain(|k, _| counts.get(k).copied().unwrap_or(0) >= floor);
    }
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
