//! `live.json`, `scene.json`, `records.json` : le materiau de l'overlay Twitch (`stream.html`).
//!
//! `live.json` est petit et reecrit a chaque tranche par `serve` : l'overlay le relit toutes
//! les quelques secondes et anime les changements. `scene.json` porte la derniere image du
//! monde. `records.json` garde les records a travers les reprises.

use serde::{Deserialize, Serialize};

/// Records d'un monde, persistes dans `records.json` et battus au fil de sa vie.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Records {
    pub largest_population: u32,
    /// Plus petite population apres que le monde a passe les 500 individus (un vrai creux).
    pub smallest_population: u32,
    pub deepest_generation: u32,
    pub most_agents_at_once: u32,
    pub longest_agent_life_ticks: u64,
    pub longest_agent_lineage: String,
    pub species_count: u32,
    pub peak_diversity: f32,
}

/// Le pouls du monde : cinq jauges de 0 a 1, chacune racontant quelque chose de l'etat
/// general (le brief : pas de chiffre decoratif).
#[derive(Debug, Clone, Serialize)]
pub struct Pulse {
    /// population / capacite de charge : a quel point le monde est plein.
    pub fill: f32,
    /// part de la carte qui porte encore de la ressource.
    pub resources: f32,
    /// diversite genetique rapportee au record du monde.
    pub diversity: f32,
    /// part des evenements vitaux recents qui sont des naissances (0,5 = a l'equilibre).
    pub renewal: f32,
    /// surexploitation moyenne du sol.
    pub strain: f32,
}

/// L'etat vivant du monde, relu en boucle par l'overlay.
#[derive(Debug, Clone, Serialize)]
pub struct LiveState {
    pub world: String,
    pub seed: u64,
    pub tick: u64,
    /// Age du monde en secondes-monde (tick * duree du tick).
    pub age_world_seconds: u64,
    /// Le stade de la simulation, dit sans le farder.
    pub stage: &'static str,
    /// STABLE | EN CROISSANCE | EN DECLIN | ETEINT, d'apres la tendance recente.
    pub status: &'static str,
    /// Dimensions du monde en cases.
    pub grid: [u32; 2],
    /// Matiere structurelle libre (briques disponibles pour de nouveaux corps).
    pub free_matter: f32,
    /// Le climat de la planete : milieu, gravite (x Terre), pression (atmospheres), constants ;
    /// `temperature_c` est la temperature EFFECTIVE, qui varie avec la saison thermique.
    pub temperature_c: f32,
    pub medium: String,
    pub gravity: f32,
    pub pressure_atm: f32,
    /// Les saisons : phase nourriciere dans [-1, 1] (`+1` abondance, `-1` disette). `0` fixe
    /// quand les saisons sont coupees. `season_label` la nomme. `season_temp_phase` est la
    /// phase thermique, decalee d'un quart de cycle (`+1` plein ete, `-1` plein hiver).
    pub season_phase: f32,
    pub season_temp_phase: f32,
    pub season_label: &'static str,

    // -- Vie du monde
    pub population: u32,
    pub births: u64,
    pub deaths_starv: u64,
    pub deaths_age: u64,
    pub mean_age_ticks: f64,
    pub mean_energy_pct: f32,
    pub biomass: f64,

    // -- Evolution
    pub diversity: f32,
    pub generation_max: u32,
    pub generation_mean: f32,
    pub lineages: u16,
    pub dominant_lineage: String,
    pub dominant_share: f32,

    // -- Ecosysteme
    pub resource_total: f64,
    pub resource_mean: f32,
    pub depleted_pct: f32,
    pub strain: f32,
    pub occupied_cells: u32,
    pub carrying_capacity: u32,

    // -- Cognition / Voix
    pub agents_alive: u32,
    pub agents_awoke_total: u64,
    pub mean_memories: f32,
    /// Signaux vivants a cet instant (Voix tranche 2) : appels « bon coin » et cris d'alarme.
    pub calls_live: u32,
    pub alarms_live: u32,

    // -- Cellules
    pub cells_alive: u32,
    pub cells_in_pct: f32,
    pub cell_size_mean: f32,
    pub cells_formed: u64,
    pub cells_dissolved: u64,
    /// Fusions de cellules depuis le debut du monde (schema v15).
    pub cells_merged: u64,
    pub last_fusion_tick: u64,

    // -- Pouls
    pub pulse: Pulse,

    // -- Evenements
    /// Les derniers evenements, du plus recent au plus ancien.
    pub events: Vec<LiveEvent>,
    /// Toute la chronique du monde, du plus ancien au plus recent : `[tick, genre]`. De quoi
    /// poser les grands tournants sur la courbe de population (l'histoire d'un coup d'oeil).
    pub chronicle: Vec<ChroniclePoint>,
    pub last_birth_tick: u64,
    pub last_death_tick: u64,
    /// Evenements vitaux (naissances + morts) par tick, sur la fenetre recente.
    pub events_per_tick: f32,
    /// Nombre de grands tournants depuis la genese, et le tick du dernier.
    pub notable_count: u32,
    pub last_notable_tick: u64,

    // -- Histoire (echantillonnee, ~170 points)
    pub pop_history: Vec<[u64; 2]>,
    pub cap_history: Vec<u64>,
    pub div_history: Vec<f32>,
    pub gen_history: Vec<u32>,
    pub tick_history: Vec<u64>,
    /// La derive de chaque trait au fil de la vie du monde (moyenne de population). Un point
    /// par echantillon, les traits dans l'ordre du brin d'ADN. L'evolution qu'on voit bouger.
    pub trait_history: Vec<[f32; genesis_core::genome::N_TRAITS]>,

    pub records: Records,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveEvent {
    pub tick: u64,
    pub kind: &'static str,
    /// Une ligne pour le fil.
    pub text: String,
    /// Presente pour les grands tournants : de quoi faire une carte a l'ecran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<EventCard>,
    /// Position dans la grille (`[x, y]` en cases) quand l'evenement a un lieu : l'overlay y
    /// pose un effet sur la scene. Aujourd'hui : les fusions de cellules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<[f32; 2]>,
}

/// La carte plein ecran d'un grand tournant.
#[derive(Debug, Clone, Serialize)]
pub struct EventCard {
    pub badge: &'static str,
    pub head: String,
    pub sub: String,
    pub tone: &'static str,
}

/// Un point de la chronique, pose sur la courbe de population.
#[derive(Debug, Clone, Serialize)]
pub struct ChroniclePoint {
    pub tick: u64,
    pub kind: &'static str,
    pub text: String,
}

use genesis_core::event::{Event, EventKind};
use genesis_core::names;
use genesis_core::persist::WorldDir;
use genesis_core::{SimConfig, WorldState};
use genesis_view::{SeriesRow, ViewFrame};

use crate::AgentLife;

fn read_json<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Option<T> {
    std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str(&t).ok())
}

fn write_json<T: Serialize>(path: &std::path::Path, v: &T) -> std::io::Result<()> {
    std::fs::write(path, serde_json::to_string_pretty(v).unwrap_or_default())
}

/// Nomme la saison d'apres sa phase. `""` quand les saisons sont coupees.
fn season_label(phase: f32, amplitude: f32) -> &'static str {
    if amplitude <= 0.0 {
        return "";
    }
    if phase >= 0.5 {
        "abondance"
    } else if phase <= -0.5 {
        "disette"
    } else if phase >= 0.0 {
        "vers l'abondance"
    } else {
        "vers la disette"
    }
}

/// Echantillonne un vecteur a au plus `n` points, en forcant le dernier point d'origine.
fn downsample<T: Copy>(src: &[T], n: usize) -> Vec<T> {
    if src.len() <= n || n == 0 {
        return src.to_vec();
    }
    let step = src.len() as f64 / n as f64;
    let mut out = Vec::with_capacity(n + 1);
    let mut i = 0.0;
    while (i as usize) < src.len() {
        out.push(src[i as usize]);
        i += step;
    }
    if let Some(&last) = src.last() {
        *out.last_mut().unwrap() = last;
    }
    out
}

/// Etiquette d'un evenement de frame pour le fil. `None` = trop bruyant pour le direct.
fn ev_label(kind: &str, subjects: &[genesis_core::EntityId]) -> Option<(&'static str, String)> {
    let who = subjects.first().map(|id| format!(" #{id}")).unwrap_or_default();
    match kind {
        "naissance" => Some(("naissance", format!("une entite est nee{who}"))),
        "division" => Some(("division", format!("l'entite{who} s'est scindee"))),
        "mort" => Some(("mort", format!("l'entite{who} est morte"))),
        "agent_eveille" => Some(("eveil", format!("l'individu{who} s'eveille"))),
        _ => None,
    }
}

/// Un grand tournant de la chronique en (genre pour le fil, ligne du fil, carte).
fn chronicle(e: &Event) -> Option<(&'static str, String, EventCard, Option<[f32; 2]>)> {
    let mut at: Option<[f32; 2]> = None;
    let (kind, text, card) = match &e.kind {
        EventKind::WorldCreated => (
            "genese",
            "genese du monde".to_string(),
            EventCard {
                badge: "GENESE",
                head: "Le monde commence".into(),
                sub: "deux organismes, une graine, rien d'ecrit pour la suite".into(),
                tone: "genese",
            },
        ),
        EventKind::SpeciesEmerged { species, size } => {
            let name = names::species_name(*species);
            (
                "espece",
                format!("{name} apparait, {size} individus"),
                EventCard {
                    badge: "NOUVELLE ESPECE",
                    head: name,
                    sub: format!("{size} individus se detachent du genome dominant"),
                    tone: "espece",
                },
            )
        }
        EventKind::LineageExtinct { lineage } => {
            let name = names::lineage_name(*lineage);
            (
                "extinction",
                format!("la lignee {name} s'eteint"),
                EventCard {
                    badge: "LIGNEE ETEINTE",
                    head: name,
                    sub: "plus aucun descendant vivant".into(),
                    tone: "extinction",
                },
            )
        }
        EventKind::PopulationCrash { from, to } => {
            let lost = from.saturating_sub(*to);
            let pct = if *from > 0 { lost * 100 / from } else { 0 };
            (
                "effondrement",
                format!("effondrement, {from} vers {to}"),
                EventCard {
                    badge: "EFFONDREMENT",
                    head: format!("{from} vers {to}"),
                    sub: format!("la population perd {lost} individus, {pct} %"),
                    tone: "effondrement",
                },
            )
        }
        EventKind::PopulationMilestone { level } => (
            "palier",
            format!("la population franchit {level}"),
            EventCard {
                badge: "CAP FRANCHI",
                head: format!("{level} individus"),
                sub: "la population n'avait jamais ete aussi nombreuse".into(),
                tone: "palier",
            },
        ),
        EventKind::CellsMerged { size, at: pos, .. } => {
            at = Some(*pos);
            (
                "fusion",
                format!("deux cellules fusionnent, {size} membres"),
                EventCard {
                    badge: "FUSION CELLULAIRE",
                    head: "Deux membranes n'en font plus qu'une".into(),
                    sub: format!("{size} organismes sous une seule membrane, un genome remanie"),
                    tone: "fusion",
                },
            )
        }
        EventKind::GenomeShift { generation, .. } => (
            "bascule",
            format!("le genome dominant du monde bascule, generation {generation}"),
            EventCard {
                badge: "BASCULE DU GENOME",
                head: "Le genome dominant du monde a change".into(),
                sub: format!(
                    "a la generation {generation}, l'evolution a deplace le centre de la population"
                ),
                tone: "bascule",
            },
        ),
        _ => return None,
    };
    Some((kind, text, card, at))
}

/// Ecrit `live.json`, `scene.json`, met a jour `records.json`, et renvoie
/// `(live_json, scene_json)` pour les embarquer dans `stream.html`.
#[allow(clippy::too_many_arguments)]
pub fn write_live(
    wdir: &WorldDir,
    world: &WorldState,
    cfg: &SimConfig,
    seed: u64,
    awoke_total: u64,
    world_name: &str,
    frames: &[ViewFrame],
    series: &[SeriesRow],
    notable: &[Event],
    lives_vec: &[AgentLife],
) -> std::io::Result<(String, String)> {
    let last = match frames.last() {
        Some(f) => f,
        None => return Ok(("{}".into(), "{}".into())),
    };
    let st = &last.stats;

    // --- Records : on charge, on bat, on sauve.
    let mut rec: Records = read_json(&wdir.root.join("records.json")).unwrap_or_default();
    // `series` porte tout l'historique (de la genese a maintenant), on peut donc rebalayer.
    // Le creux ne compte qu'apres que le monde a passe les 500 individus une premiere fois,
    // sinon on ne capterait que la population de la genese.
    let mut smallest = 0u32;
    let mut established = false;
    for r in series.iter() {
        rec.largest_population = rec.largest_population.max(r.population);
        rec.most_agents_at_once = rec.most_agents_at_once.max(r.agents_alive);
        rec.deepest_generation = rec.deepest_generation.max(r.max_generation);
        rec.peak_diversity = rec.peak_diversity.max(r.genetic_diversity);
        if r.population >= 500 {
            established = true;
        }
        if established && r.population > 0 {
            smallest = if smallest == 0 { r.population } else { smallest.min(r.population) };
        }
    }
    rec.smallest_population = smallest;
    let world_tick = world.tick;
    let span = |l: &AgentLife| l.ended_tick.unwrap_or(world_tick).saturating_sub(l.awoke_tick);
    if let Some(top) = lives_vec.iter().max_by_key(|l| span(l)) {
        if span(top) > rec.longest_agent_life_ticks {
            rec.longest_agent_life_ticks = span(top);
            rec.longest_agent_lineage = names::lineage_name(top.lineage);
        }
    }
    rec.species_count = rec
        .species_count
        .max(notable.iter().filter(|e| matches!(e.kind, EventKind::SpeciesEmerged { .. })).count() as u32);
    write_json(&wdir.root.join("records.json"), &rec)?;

    // --- Le fil : les derniers evenements de la fenetre de frames, plus les grands tournants.
    // On garde `last_birth` / `last_death` monotones : la fenetre peut ne pas contenir de
    // mort sur un intervalle calme (plateau de capacite), on repart alors du live.json d'avant.
    #[derive(serde::Deserialize)]
    struct PrevTicks {
        #[serde(default)]
        last_birth_tick: u64,
        #[serde(default)]
        last_death_tick: u64,
    }
    let prev: Option<PrevTicks> = read_json(&wdir.root.join("live.json"));
    let mut events: Vec<LiveEvent> = Vec::new();
    let mut last_birth = prev.as_ref().map(|p| p.last_birth_tick).unwrap_or(0);
    let mut last_death = prev.as_ref().map(|p| p.last_death_tick).unwrap_or(0);
    let mut eveils = 0;
    // Le fil ne montre que la queue recente ; les compteurs balaient toute la fenetre.
    let tail = frames.len().saturating_sub(14);
    for (i, f) in frames.iter().enumerate() {
        for e in f.events.iter() {
            if e.kind == "naissance" {
                last_birth = last_birth.max(e.tick);
            }
            if e.kind == "mort" {
                last_death = last_death.max(e.tick);
            }
            if i < tail {
                continue;
            }
            if let Some((k, text)) = ev_label(e.kind, &e.subjects) {
                // On borne les eveils : ils sont trop nombreux pour le fil.
                if k == "eveil" {
                    eveils += 1;
                    if eveils > 3 {
                        continue;
                    }
                }
                events.push(LiveEvent { tick: e.tick, kind: k, text, card: None, at: None });
            }
        }
    }
    let mut last_notable = 0u64;
    let mut last_fusion = 0u64;
    for e in notable.iter().rev().take(8) {
        if let Some((kind, text, card, at)) = chronicle(e) {
            last_notable = last_notable.max(e.tick);
            if kind == "fusion" {
                last_fusion = last_fusion.max(e.tick);
            }
            events.push(LiveEvent { tick: e.tick, kind, text, card: Some(card), at });
        }
    }
    events.sort_by_key(|a| std::cmp::Reverse(a.tick));
    events.dedup_by(|a, b| a.tick == b.tick && a.text == b.text);
    events.truncate(24);

    // La chronique complete (du plus ancien au plus recent) pour la poser sur la courbe.
    let mut chron: Vec<ChroniclePoint> = notable
        .iter()
        .filter_map(|e| chronicle(e).map(|(kind, text, _, _)| ChroniclePoint { tick: e.tick, kind, text }))
        .collect();
    chron.sort_by_key(|c| c.tick);
    chron.dedup_by(|a, b| a.tick == b.tick && a.kind == b.kind);
    // Un monde tres ancien peut accumuler beaucoup de fusions : on garde tout ce qui n'est
    // pas fusion, et on echantillonne les fusions si elles sont nombreuses.
    if chron.iter().filter(|c| c.kind == "fusion").count() > 60 {
        let mut seen = 0usize;
        chron.retain(|c| {
            if c.kind != "fusion" {
                return true;
            }
            seen += 1;
            seen % 4 == 0
        });
    }

    let pop_hist: Vec<u32> = series.iter().map(|r| r.population).collect();
    let cap_hist: Vec<u32> = series.iter().map(|r| r.carrying_capacity).collect();
    let div_hist: Vec<f32> = series.iter().map(|r| r.genetic_diversity).collect();
    let gen_hist: Vec<u32> = series.iter().map(|r| r.max_generation).collect();
    let tick_hist: Vec<u64> = series.iter().map(|r| r.tick).collect();
    let trait_hist: Vec<[f32; genesis_core::genome::N_TRAITS]> =
        series.iter().map(|r| r.trait_mean).collect();
    let n = 170;

    let (agents_alive, mean_mem) = world.agent_stats();

    // --- Fenetre recente : tendance de population, cadence des evenements vitaux.
    let win = series.len().saturating_sub(9);
    let (ref_row, cur_row) = (series.get(win), series.last());
    let (dpop_pct, ev_per_tick, births_w, deaths_w) = match (ref_row, cur_row) {
        (Some(a), Some(b)) if b.tick > a.tick => {
            let dt = (b.tick - a.tick) as f32;
            let db = b.births_total.saturating_sub(a.births_total) as f32;
            let dd = b.deaths_total.saturating_sub(a.deaths_total) as f32;
            let dp = if a.population > 0 {
                (b.population as f32 - a.population as f32) / a.population as f32
            } else {
                0.0
            };
            (dp, (db + dd) / dt, db, dd)
        }
        _ => (0.0, 0.0, 0.0, 0.0),
    };
    let status = if st.population == 0 {
        "ETEINT"
    } else if dpop_pct > 0.05 {
        "EN CROISSANCE"
    } else if dpop_pct < -0.05 {
        "EN DECLIN"
    } else {
        "STABLE"
    };
    // Genesis, dit sans le farder : molecules, premieres cellules, premiers signaux.
    let stage = if agents_alive > 0 {
        "Vie moleculaire, cellules et premiers signaux"
    } else if st.cells_alive > 0 {
        "Vie moleculaire, premieres cellules"
    } else {
        "Vie moleculaire"
    };
    let pulse = Pulse {
        fill: if st.carrying_capacity > 0 {
            (st.population as f32 / st.carrying_capacity as f32).clamp(0.0, 1.0)
        } else {
            0.0
        },
        resources: (1.0 - st.depleted_fraction).clamp(0.0, 1.0),
        diversity: if rec.peak_diversity > 0.0 {
            (st.genetic_diversity / rec.peak_diversity).clamp(0.0, 1.0)
        } else {
            0.0
        },
        renewal: if births_w + deaths_w > 0.0 {
            births_w / (births_w + deaths_w)
        } else {
            0.5
        },
        strain: st.mean_strain.clamp(0.0, 1.0),
    };

    let sphase = genesis_core::sim::season_phase(cfg, world.tick);
    let stphase = genesis_core::sim::season_temp_phase(cfg, world.tick);
    let live = LiveState {
        world: world_name.to_string(),
        seed,
        tick: world.tick,
        age_world_seconds: world.tick.saturating_mul(cfg.time.tick_duration_seconds),
        stage,
        status,
        grid: [world.space.width, world.space.height],
        free_matter: st.free_matter,
        temperature_c: cfg.planet.temperature_c + cfg.season.temp_amplitude_c * stphase,
        medium: cfg.planet.medium.clone(),
        gravity: cfg.planet.gravity,
        pressure_atm: cfg.planet.pressure_atm,
        season_phase: sphase,
        season_temp_phase: stphase,
        season_label: season_label(sphase, cfg.season.amplitude),
        population: st.population,
        births: st.births_total,
        deaths_starv: st.deaths_starvation,
        deaths_age: st.deaths_age,
        mean_age_ticks: st.mean_age_ticks,
        mean_energy_pct: st.mean_energy_pct,
        biomass: st.biomass_energy,
        diversity: st.genetic_diversity,
        generation_max: st.max_generation,
        generation_mean: series.last().map(|r| r.mean_generation).unwrap_or(0.0),
        lineages: st.lineages_alive,
        dominant_lineage: names::lineage_name(st.dominant_lineage),
        dominant_share: st.dominant_lineage_share,
        resource_total: st.resource_total,
        resource_mean: st.resource_mean,
        depleted_pct: st.depleted_fraction * 100.0,
        strain: st.mean_strain,
        occupied_cells: st.occupied_cells,
        carrying_capacity: st.carrying_capacity,
        agents_alive,
        agents_awoke_total: awoke_total,
        mean_memories: mean_mem,
        calls_live: world
            .signals
            .iter()
            .filter(|s| s.kind == genesis_core::SignalKind::Bounty)
            .count() as u32,
        alarms_live: world
            .signals
            .iter()
            .filter(|s| s.kind == genesis_core::SignalKind::Alarm)
            .count() as u32,
        cells_alive: st.cells_alive,
        cells_in_pct: if st.population > 0 {
            st.entities_in_cells as f32 / st.population as f32 * 100.0
        } else {
            0.0
        },
        cell_size_mean: st.mean_cell_size,
        cells_formed: st.cells_formed_total,
        cells_dissolved: st.cells_dissolved_total,
        cells_merged: world.cells_merged_total,
        last_fusion_tick: last_fusion,
        pulse,
        events,
        chronicle: chron,
        last_birth_tick: last_birth,
        last_death_tick: last_death,
        events_per_tick: ev_per_tick,
        notable_count: notable.iter().filter(|e| chronicle(e).is_some()).count() as u32,
        last_notable_tick: last_notable,
        pop_history: downsample(&pop_hist, n)
            .into_iter()
            .zip(downsample(&tick_hist, n))
            .map(|(p, t)| [t, p as u64])
            .collect(),
        cap_history: downsample(&cap_hist, n).into_iter().map(|c| c as u64).collect(),
        div_history: downsample(&div_hist, n),
        gen_history: downsample(&gen_hist, n),
        tick_history: downsample(&tick_hist, n),
        trait_history: downsample(&trait_hist, 120),
        records: rec,
    };
    let live_json = serde_json::to_string(&live).unwrap_or_else(|_| "{}".into());
    let scene_json = serde_json::to_string(last).unwrap_or_else(|_| "{}".into());
    std::fs::write(wdir.root.join("live.json"), &live_json)?;
    // La derniere image du monde, pour la scene de l'overlay.
    std::fs::write(wdir.root.join("scene.json"), &scene_json)?;
    Ok((live_json, scene_json))
}
