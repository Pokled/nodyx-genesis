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

/// L'etat vivant du monde, relu en boucle par l'overlay.
#[derive(Debug, Clone, Serialize)]
pub struct LiveState {
    pub world: String,
    pub seed: u64,
    pub tick: u64,
    /// Age du monde en secondes-monde (tick * duree du tick).
    pub age_world_seconds: u64,

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
    pub cells_alive: u32,

    // -- Evenements
    /// Les derniers evenements, du plus recent au plus ancien : [tick, genre, phrase].
    pub events: Vec<LiveEvent>,
    pub last_birth_tick: u64,
    pub last_death_tick: u64,

    // -- Histoire (echantillonnee, ~160 points)
    pub pop_history: Vec<[u64; 2]>,
    pub cap_history: Vec<u64>,
    pub div_history: Vec<f32>,
    pub tick_history: Vec<u64>,

    pub records: Records,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveEvent {
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

    // --- Le fil : les derniers evenements des dernieres frames, plus les grands tournants.
    let mut events: Vec<LiveEvent> = Vec::new();
    let mut last_birth = 0u64;
    let mut last_death = 0u64;
    let mut eveils = 0;
    let from = frames.len().saturating_sub(12);
    for f in &frames[from..] {
        for e in f.events.iter() {
            if e.kind == "naissance" {
                last_birth = last_birth.max(e.tick);
            }
            if e.kind == "mort" {
                last_death = last_death.max(e.tick);
            }
            if let Some((k, text)) = ev_label(e.kind, &e.subjects) {
                // On borne les eveils : ils sont trop nombreux pour le fil.
                if k == "eveil" {
                    eveils += 1;
                    if eveils > 3 {
                        continue;
                    }
                }
                events.push(LiveEvent { tick: e.tick, kind: k, text });
            }
        }
    }
    for e in notable.iter().rev().take(6) {
        let (k, text): (&'static str, String) = match &e.kind {
            EventKind::WorldCreated => ("tournant", "genese du monde".into()),
            EventKind::SpeciesEmerged { species, size } => (
                "tournant",
                format!("{} apparait, {size} individus", names::species_name(*species)),
            ),
            EventKind::LineageExtinct { lineage } => (
                "tournant",
                format!("la lignee {} s'eteint", names::lineage_name(*lineage)),
            ),
            EventKind::PopulationCrash { from, to } => {
                ("tournant", format!("effondrement : {from} vers {to}"))
            }
            EventKind::PopulationMilestone { level } => {
                ("tournant", format!("la population franchit {level}"))
            }
            _ => continue,
        };
        events.push(LiveEvent { tick: e.tick, kind: k, text });
    }
    events.sort_by_key(|a| std::cmp::Reverse(a.tick));
    events.dedup_by(|a, b| a.tick == b.tick && a.text == b.text);
    events.truncate(22);

    let pop_hist: Vec<u32> = series.iter().map(|r| r.population).collect();
    let cap_hist: Vec<u32> = series.iter().map(|r| r.carrying_capacity).collect();
    let div_hist: Vec<f32> = series.iter().map(|r| r.genetic_diversity).collect();
    let tick_hist: Vec<u64> = series.iter().map(|r| r.tick).collect();
    let n = 170;

    let (agents_alive, mean_mem) = world.agent_stats();

    let live = LiveState {
        world: world_name.to_string(),
        seed,
        tick: world.tick,
        age_world_seconds: world.tick.saturating_mul(cfg.time.tick_duration_seconds),
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
        cells_alive: st.cells_alive,
        events,
        last_birth_tick: last_birth,
        last_death_tick: last_death,
        pop_history: downsample(&pop_hist, n)
            .into_iter()
            .zip(downsample(&tick_hist, n))
            .map(|(p, t)| [t, p as u64])
            .collect(),
        cap_history: downsample(&cap_hist, n).into_iter().map(|c| c as u64).collect(),
        div_history: downsample(&div_hist, n),
        tick_history: downsample(&tick_hist, n),
        records: rec,
    };
    let live_json = serde_json::to_string(&live).unwrap_or_else(|_| "{}".into());
    let scene_json = serde_json::to_string(last).unwrap_or_else(|_| "{}".into());
    std::fs::write(wdir.root.join("live.json"), &live_json)?;
    // La derniere image du monde, pour la scene de l'overlay.
    std::fs::write(wdir.root.join("scene.json"), &scene_json)?;
    Ok((live_json, scene_json))
}


