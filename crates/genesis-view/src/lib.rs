//! Le contrat ViewState.
//!
//! Tranchee 3 : le moteur ne dessine rien, il emet un flux d'etat observable. Le web,
//! Godot, Nodyx, la CLI consomment le meme flux.
//!
//! Proprietes (voir BIBLE/02_ARCHITECTURE.md) :
//!  - lecture seule, un client ne peut rien reecrire par ce canal,
//!  - `ViewFrame` est une fonction pure de `(WorldState, config)`. Deux clients avec le
//!    meme instantane voient exactement la meme chose,
//!  - `view_version` evolue independamment du schema du World State,
//!  - le mouvement n'apparait jamais comme evenement, les positions vivent dans `entities`.

use serde::Serialize;

use genesis_core::event::{Event, EventKind};
use genesis_core::genome::N_TRAITS;
use genesis_core::names;
use genesis_core::{EntityId, SimConfig, WorldState};

pub const VIEW_VERSION: u16 = 8;

#[derive(Debug, Clone, Serialize)]
pub struct ViewFrame {
    pub view_version: u16,
    pub world_id: u64,
    pub tick: u64,
    pub world_clock: WorldClock,
    /// Ticks par seconde reelle, informatif.
    pub speed: f32,
    pub grid: [u32; 2],
    /// "detail" : `entities` porte chaque individu. "region" : `clusters` porte des amas,
    /// `entities` est vide. Le moteur choisit selon la population (tranchee 5 : le regard
    /// ne change pas la simulation).
    pub lod: &'static str,
    pub resources: ResourceView,
    pub entities: Vec<EntityView>,
    pub clusters: Vec<ClusterView>,
    /// Cellules (0.0.2, tranche 2). Toujours rempli, dans les deux LOD : une cellule est une
    /// unite persistante, pas un amas de densite.
    pub cells: Vec<CellView>,
    /// La Voix (0.0.4) : les alarmes vivantes, pour que la panique se voie. `[x, y, age]` en
    /// quarts de case et en ticks depuis l'emission.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<[u16; 3]>,
    pub events: Vec<EventView>,
    pub stats: WorldStats,
}

/// Une cellule : un amas coherent de parents reconnu comme unite. Le lecteur en dessine
/// une membrane.
#[derive(Debug, Clone, Serialize)]
pub struct CellView {
    /// centroide * POS_SCALE.
    pub pos: [u16; 2],
    /// rayon des membres * POS_SCALE.
    pub radius: u16,
    pub count: u32,
    /// teinte du genome moyen de la cellule.
    pub hue: u16,
    /// signature du genome moyen, chaque trait quantifie en niveau 0..3.
    pub genome: [u8; N_TRAITS],
}

/// Un amas : plusieurs entites d'une meme region resumees en un point. Ce qui rend un
/// monde peuple observable sans faire exploser le flux.
#[derive(Debug, Clone, Serialize)]
pub struct ClusterView {
    /// centroide * POS_SCALE.
    pub pos: [u16; 2],
    /// rayon moyen des membres * POS_SCALE.
    pub radius: u16,
    pub count: u32,
    /// energie moyenne en pourcents, 0..100.
    pub energy: u8,
    pub hue: u16,
    /// action dominante de l'amas.
    pub state: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorldClock {
    pub year: u64,
    pub day: u16,
    pub hour: u8,
}

impl WorldClock {
    fn from_tick(tick: u64, seconds_per_tick: u64) -> Self {
        let total = tick.saturating_mul(seconds_per_tick);
        let hours = total / 3600;
        let days = hours / 24;
        WorldClock {
            year: days / 365,
            day: (days % 365) as u16,
            hour: (hours % 24) as u8,
        }
    }
}

/// Grille de densite de ressources, sous-echantillonnee et quantifiee.
#[derive(Debug, Clone, Serialize)]
pub struct ResourceView {
    pub w: u32,
    pub h: u32,
    /// densite de ressource 0..255 par case sous-echantillonnee, ligne par ligne.
    pub cells: Vec<u8>,
    /// fertilite du sol 0..255, le relief statique du monde.
    pub fertility: Vec<u8>,
    /// surexploitation 0..255, la trace de l'activite des entites.
    pub strain: Vec<u8>,
}

// Tout est quantifie en entiers : la serialisation JSON d'un f32 coute ~18 caracteres
// (promotion en f64), un entier en coute 1 a 4. Sur des milliers d'entites par frame,
// c'est la difference entre un fichier de 50 Mo et un fichier de 4 Mo.

/// Position en quarts de case : `world_pos * 4`. Le lecteur redivise par 4.
pub const POS_SCALE: f32 = 4.0;

fn is_full_hp(v: &u8) -> bool {
    *v >= 100
}
fn is_zero_u32(v: &u32) -> bool {
    *v == 0
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityView {
    pub id: EntityId,
    /// position * POS_SCALE.
    pub pos: [u16; 2],
    /// energie en pourcents, 0..100.
    pub energy: u8,
    /// age en pourcents de l'esperance, 0..150.
    pub age: u8,
    /// sante en pourcents, 0..100 (biologie de fond, 0.0.3 tranche 8). Omis si intacte (100).
    #[serde(skip_serializing_if = "is_full_hp")]
    pub hp: u8,
    /// teinte 0..359, deja calculee depuis le genome.
    pub hue: u16,
    pub state: &'static str,
    /// lignee fondatrice et generation : de quoi presenter l'individu au clic.
    pub lin: u16,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub gen: u32,
    /// `true` si l'entite est un agent (elle porte un `Mind`, 0.0.3). Omis sinon.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub agent: bool,
    /// L'esprit de l'agent, resume pour l'inspecteur du lecteur. `None` pour une entite de
    /// fond.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mind: Option<MindView>,
}

/// L'interieur d'un agent, tel qu'on peut le lire en cliquant dessus dans le lecteur.
#[derive(Debug, Clone, Serialize)]
pub struct MindView {
    /// tick d'eveil.
    pub awoke: u64,
    /// mode de comportement du dernier choix : forage | flee | join | seek_bounty | wander.
    pub mode: &'static str,
    /// faim, peur, solitude en pourcents.
    pub needs: [u8; 3],
    /// souvenirs, les plus forts d'abord : [nature (0 peril, 1 aubaine, 2 mort vue), force %].
    pub mem: Vec<[u8; 2]>,
    /// nombre de relations sociales tenues.
    pub ties: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventView {
    pub tick: u64,
    pub kind: &'static str,
    /// Saillance 0..255, reprise de l'evenement moteur. Les chapitres filtrent dessus.
    pub salience: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<EntityId>,
    /// `seq` des evenements qui ont cause celui-ci (0.0.2 : crash de population, extinction
    /// de lignee ; vide ailleurs). Le graphe causal complet est a 0.0.6.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub causes: Vec<u64>,
    /// Precision lisible pour les evenements saillants (palier, effondrement, espece).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorldStats {
    pub population: u32,
    pub births_total: u64,
    pub deaths_total: u64,
    pub deaths_starvation: u64,
    pub deaths_age: u64,
    pub mean_age_ticks: f64,
    pub genetic_diversity: f32,
    pub mean_energy_pct: f32,
    pub biomass_energy: f64,

    /// Lignees fondatrices vivantes, part de la dominante, generation maximale.
    pub lineages_alive: u16,
    pub dominant_lineage_share: f32,
    /// Indice de la lignee fondatrice la plus repandue (le lecteur en tire un nom).
    pub dominant_lineage: u16,
    pub max_generation: u32,

    /// Moyenne et ecart-type par trait : metabolism, speed, perception, efficiency,
    /// fertility, lifespan, cohesion.
    pub trait_mean: [f32; N_TRAITS],
    pub trait_spread: [f32; N_TRAITS],
    /// Signature de l'espece dominante : chaque trait quantifie en niveau 0..3. Le brin d'ADN.
    pub dominant_genome: [u8; N_TRAITS],

    /// Milieu.
    pub resource_total: f64,
    pub resource_mean: f32,
    pub depleted_fraction: f32,
    pub mean_strain: f32,
    pub occupied_cells: u32,

    /// Matiere structurelle libre du monde (briques). Basse au plateau de capacite.
    pub free_matter: f32,
    /// Capacite de charge : plafond de population que la matiere totale permet.
    pub carrying_capacity: u32,
    /// Fraction de la matiere totale immobilisee dans les corps vivants, 0..1. Approche 1
    /// au plateau : c'est la matiere qui limite alors, pas l'energie.
    pub matter_locked_fraction: f32,
    /// Divisions par ailleurs eligibles bloquees faute de matiere libre, cumule.
    pub repro_blocked_materials: u64,

    /// Cellules (0.0.2, tranche 2) : vivantes, entites en cellule, taille moyenne.
    pub cells_alive: u32,
    pub entities_in_cells: u32,
    pub mean_cell_size: f32,
    pub cells_formed_total: u64,
    pub cells_dissolved_total: u64,

    /// Agents (0.0.3, tranche 1) : entites vivantes qui portent une memoire, et le nombre
    /// moyen de souvenirs episodiques parmi elles.
    pub agents_alive: u32,
    pub mean_memories: f32,
}

/// Une ligne de la serie temporelle (`series.jsonl`). Fonction pure de `(WorldState, config)`
/// comme `project` (tranchee 3). Le materiau du graphe d'evolution genetique (`series.html`).
/// Echantillonnee tous les `[persistence] series_every` ticks.
#[derive(Debug, Clone, Serialize)]
pub struct SeriesRow {
    pub tick: u64,
    pub year: u64,
    pub population: u32,
    pub births_total: u64,
    pub deaths_total: u64,
    pub deaths_starvation: u64,
    pub deaths_age: u64,
    /// Generation genomique : moyenne, ecart-type, maximum. L'axe « generations ecoulees ».
    pub mean_generation: f32,
    pub generation_spread: f32,
    pub max_generation: u32,
    pub genetic_diversity: f32,
    /// Indice de la lignee fondatrice la plus repandue (le lecteur en tire un nom).
    pub dominant_lineage: u16,
    pub cells_alive: u32,
    pub entities_in_cells: u32,
    /// Agents vivants (0.0.3, tranche 1).
    pub agents_alive: u32,
    pub carrying_capacity: u32,
    /// Par trait (metabolism, speed, perception, efficiency, fertility, lifespan, cohesion).
    pub trait_mean: [f32; N_TRAITS],
    pub trait_spread: [f32; N_TRAITS],
    pub trait_p10: [f32; N_TRAITS],
    pub trait_p50: [f32; N_TRAITS],
    pub trait_p90: [f32; N_TRAITS],
}

/// Construit une ligne de serie a partir de l'etat du monde. Pure.
pub fn series_row(world: &WorldState, cfg: &SimConfig) -> SeriesRow {
    let (trait_mean, trait_spread) = world.trait_stats();
    let q = world.trait_quantiles();
    let (mean_generation, generation_spread) = world.generation_stats();
    let (_, _, max_generation) = world.lineage_stats();
    let (_, carrying_capacity, _) = world.matter_stats(cfg.bricks.body_matter);
    let (cells_alive, entities_in_cells, _) = world.cell_stats();
    let (agents_alive, _) = world.agent_stats();
    let clock = WorldClock::from_tick(world.tick, cfg.time.tick_duration_seconds);
    SeriesRow {
        tick: world.tick,
        year: clock.year,
        population: world.population(),
        births_total: world.births_total,
        deaths_total: world.deaths_total,
        deaths_starvation: world.deaths_starvation,
        deaths_age: world.deaths_age,
        mean_generation,
        generation_spread,
        max_generation,
        genetic_diversity: world.genetic_diversity(),
        dominant_lineage: world.dominant_lineage(),
        cells_alive,
        entities_in_cells,
        agents_alive,
        carrying_capacity,
        trait_mean,
        trait_spread,
        trait_p10: q[0],
        trait_p50: q[1],
        trait_p90: q[2],
    }
}

const VIEW_GRID: u32 = 48;

/// En mode region, nombre maximal d'agents envoyes comme individus (les plus ages). Borne le
/// poids du flux quand presque toute la population est eveillee.
const AGENT_VIEW_CAP: usize = 300;

/// Projette le monde en une frame. Fonction pure.
pub fn project(
    world: &WorldState,
    cfg: &SimConfig,
    speed: f32,
    notable_events: &[Event],
) -> ViewFrame {
    let energy_ceiling = cfg.reproduction.energy_threshold * 2.0;

    let qpos = |x: f32| (x * POS_SCALE).round().clamp(0.0, 65535.0) as u16;
    let hue_of = |t: &genesis_core::Traits| {
        (20.0 + (t.speed * 0.6 + t.perception * 0.4) * 260.0).rem_euclid(360.0)
    };

    let mut energy_sum = 0.0f32;
    for e in world.entities.iter() {
        energy_sum += (e.energy / energy_ceiling).clamp(0.0, 1.0);
    }
    let pop = world.entities.len().max(1) as f32;

    // Detail ou region : au dela d'un seuil de population, on agrege en amas.
    let detail = world.entities.len() as u32 <= cfg.view.detail_max_entities;
    let mut entities: Vec<EntityView> = Vec::new();
    let mut clusters: Vec<ClusterView> = Vec::new();
    let lod: &'static str;

    // Une entite -> sa vue. `full` : detail complet (mode detail) ; sinon on ne garde que les
    // agents (mode region), pour qu'un individu qui se souvient reste visible et cliquable
    // meme quand le fond de population est resume en amas.
    let entity_view = |e: &genesis_core::Entity, full: bool| -> EntityView {
        let expected =
            cfg.lifecycle.lifespan_ticks_mean as f32 * (0.5 + e.genome.traits.lifespan);
        let energy_pct = (e.energy / energy_ceiling).clamp(0.0, 1.0);
        let age_pct = (e.age_ticks as f32 / expected.max(1.0)).clamp(0.0, 1.5);
        let mind = e.mind.as_deref().filter(|_| full).map(|m| {
            let mut mem: Vec<[u8; 2]> = m
                .episodic
                .iter()
                .map(|s| {
                    let k = match s.kind {
                        genesis_core::MemoryKind::Peril => 0u8,
                        genesis_core::MemoryKind::Bounty => 1,
                        genesis_core::MemoryKind::Witnessed => 2,
                    };
                    [k, (s.strength.clamp(0.0, 1.0) * 100.0).round() as u8]
                })
                .collect::<Vec<_>>();
            mem.sort_by(|a, b| b[1].cmp(&a[1]));
            mem.truncate(3);
            let pct = |v: f32| (v.clamp(0.0, 1.0) * 100.0).round() as u8;
            MindView {
                awoke: m.awoke_tick,
                mode: m.mode.as_str(),
                needs: [pct(m.needs.hunger), pct(m.needs.fear), pct(m.needs.solitude)],
                mem,
                ties: m.social.len() as u16,
            }
        });
        EntityView {
            id: e.id,
            pos: [qpos(e.position.x), qpos(e.position.y)],
            energy: (energy_pct * 100.0).round() as u8,
            age: (age_pct * 100.0).round() as u8,
            hp: (e.health.clamp(0.0, 1.0) * 100.0).round() as u8,
            hue: hue_of(&e.genome.traits) as u16,
            state: e.last_action.as_str(),
            lin: e.genome.lineage,
            gen: e.genome.generation,
            agent: e.mind.is_some(),
            mind,
        }
    };

    if detail {
        lod = "detail";
        entities.reserve(world.entities.len());
        for e in world.entities.iter() {
            // Detail complet : la population tient a l'ecran, on peut tout inspecter.
            entities.push(entity_view(e, true));
        }
    } else {
        lod = "region";
        clusters = build_clusters(world, energy_ceiling, cfg.view.cluster_grid.max(2), &hue_of);
        // Le fond de population est resume en amas ; les agents les plus ages restent des
        // individus, visibles et cliquables. Ce sont « les anciens », ceux dont la memoire
        // pese le plus. Tri deterministe (age puis id). Sans le detail de l'esprit : sur une
        // grosse population ce serait des megaoctets. Il se lit en periode moins peuplee ou
        // dans la biographie.
        let mut elders: Vec<&genesis_core::Entity> =
            world.entities.iter().filter(|e| e.mind.is_some()).collect();
        elders.sort_by(|a, b| b.age_ticks.cmp(&a.age_ticks).then(a.id.cmp(&b.id)));
        elders.truncate(AGENT_VIEW_CAP);
        for e in elders {
            entities.push(entity_view(e, false));
        }
    }

    // Cellules : toujours envoyees, quel que soit le LOD.
    // hue depuis les traits moyens (speed = index 1, perception = index 2), meme formule
    // que `hue_of` pour une entite.
    let cells: Vec<CellView> = world
        .cells
        .iter()
        .map(|c| {
            let mut genome = [0u8; N_TRAITS];
            for (k, v) in c.mean_traits.iter().enumerate() {
                genome[k] = (v * 4.0).clamp(0.0, 3.0) as u8;
            }
            let hue = (20.0 + (c.mean_traits[1] * 0.6 + c.mean_traits[2] * 0.4) * 260.0)
                .rem_euclid(360.0) as u16;
            CellView {
                pos: [qpos(c.position.x), qpos(c.position.y)],
                radius: (c.radius * POS_SCALE).round().clamp(1.0, 65535.0) as u16,
                count: c.member_count,
                hue,
                genome,
            }
        })
        .collect();

    // Une frame ne porte pas tous les evenements de l'intervalle : sur une grosse
    // population c'est des milliers de naissances et de morts. On garde tous les saillants
    // (chapitres) et une queue des plus recents evenements de routine, assez pour nourrir
    // le journal roulant du lecteur.
    const ROUTINE_KEEP: usize = 24;
    let all: Vec<EventView> = notable_events.iter().filter_map(event_view).collect();
    let events: Vec<EventView> = if all.len() <= ROUTINE_KEEP + 8 {
        all
    } else {
        let mut kept: Vec<EventView> =
            all.iter().filter(|e| e.salience >= 150).cloned().collect();
        let routine: Vec<EventView> =
            all.iter().filter(|e| e.salience < 150).cloned().collect();
        let start = routine.len().saturating_sub(ROUTINE_KEEP);
        kept.extend_from_slice(&routine[start..]);
        kept.sort_by_key(|e| e.tick);
        kept
    };

    let (trait_mean, trait_spread) = world.trait_stats();
    let (lineages_alive, dominant_lineage_share, max_generation) = world.lineage_stats();
    let (resource_mean, depleted_fraction, mean_strain) =
        world.environment_stats(cfg.resources.max_per_cell);
    let (free_matter, carrying_capacity, matter_locked_fraction) =
        world.matter_stats(cfg.bricks.body_matter);
    let (cells_alive, entities_in_cells, mean_cell_size) = world.cell_stats();
    let (agents_alive, mean_memories) = world.agent_stats();

    ViewFrame {
        view_version: VIEW_VERSION,
        world_id: world.world_id,
        tick: world.tick,
        world_clock: WorldClock::from_tick(world.tick, cfg.time.tick_duration_seconds),
        speed,
        grid: [world.space.width, world.space.height],
        lod,
        resources: downsample(world, cfg.resources.max_per_cell),
        entities,
        clusters,
        cells,
        signals: world
            .signals
            .iter()
            .map(|s| {
                [
                    qpos(s.pos.x),
                    qpos(s.pos.y),
                    world.tick.saturating_sub(s.born).min(255) as u16,
                ]
            })
            .collect(),
        events,
        stats: WorldStats {
            population: world.population(),
            births_total: world.births_total,
            deaths_total: world.deaths_total,
            deaths_starvation: world.deaths_starvation,
            deaths_age: world.deaths_age,
            mean_age_ticks: world.mean_age(),
            genetic_diversity: world.genetic_diversity(),
            mean_energy_pct: energy_sum / pop,
            biomass_energy: world.biomass_energy(),
            lineages_alive,
            dominant_lineage_share,
            dominant_lineage: world.dominant_lineage(),
            max_generation,
            trait_mean,
            trait_spread,
            dominant_genome: world.dominant_genome(),
            resource_total: world.resources.total(),
            resource_mean,
            depleted_fraction,
            mean_strain,
            occupied_cells: world.occupied_cells(),
            free_matter,
            carrying_capacity,
            matter_locked_fraction,
            repro_blocked_materials: world.repro_blocked_materials,
            cells_alive,
            entities_in_cells,
            mean_cell_size,
            cells_formed_total: world.cells_formed_total,
            cells_dissolved_total: world.cells_dissolved_total,
            agents_alive,
            mean_memories,
        },
    }
}

/// Agrege les entites en amas sur une grille `grid x grid`. Deterministe : on parcourt
/// les entites dans l'ordre des `EntityId`, une case = un amas.
fn build_clusters(
    world: &WorldState,
    energy_ceiling: f32,
    grid: u32,
    hue_of: &impl Fn(&genesis_core::Traits) -> f32,
) -> Vec<ClusterView> {
    use std::collections::BTreeMap;
    let gw = world.space.width as f32;
    let gh = world.space.height as f32;
    let g = grid as f32;

    struct Acc {
        n: u32,
        sx: f32,
        sy: f32,
        se: f32,
        shx: f32,
        shy: f32, // teinte moyenne via somme des vecteurs unites (evite le repli 360/0)
        states: [u32; 4], // forage, eat, divide, dying
    }
    let mut cells: BTreeMap<(u32, u32), Acc> = BTreeMap::new();
    for e in world.entities.iter() {
        let cx = ((e.position.x / gw * g) as u32).min(grid - 1);
        let cy = ((e.position.y / gh * g) as u32).min(grid - 1);
        let a = cells.entry((cx, cy)).or_insert(Acc {
            n: 0,
            sx: 0.0,
            sy: 0.0,
            se: 0.0,
            shx: 0.0,
            shy: 0.0,
            states: [0; 4],
        });
        a.n += 1;
        a.sx += e.position.x;
        a.sy += e.position.y;
        a.se += (e.energy / energy_ceiling).clamp(0.0, 1.0);
        let h = hue_of(&e.genome.traits).to_radians();
        a.shx += h.cos();
        a.shy += h.sin();
        let si = match e.last_action.as_str() {
            "eat" => 1,
            "divide" => 2,
            "dying" => 3,
            _ => 0,
        };
        a.states[si] += 1;
    }

    let names = ["forage", "eat", "divide", "dying"];
    cells
        .into_iter()
        .map(|((_, _), a)| {
            let n = a.n as f32;
            let cx = a.sx / n;
            let cy = a.sy / n;
            // rayon moyen : approxime par le rayon d'un disque de meme densite que la case
            let radius = (gw / g).max(gh / g) * 0.5 * (n.min(40.0) / 40.0).sqrt().max(0.25);
            let hue = a.shy.atan2(a.shx).to_degrees().rem_euclid(360.0) as u16;
            let dom = (0..4).max_by_key(|&i| a.states[i]).unwrap_or(0);
            ClusterView {
                pos: [(cx * POS_SCALE).round() as u16, (cy * POS_SCALE).round() as u16],
                radius: (radius * POS_SCALE).round().clamp(1.0, 65535.0) as u16,
                count: a.n,
                energy: ((a.se / n) * 100.0).round() as u8,
                hue,
                state: names[dom],
            }
        })
        .collect()
}

fn downsample(world: &WorldState, max_per_cell: f32) -> ResourceView {
    let sw = world.space.width;
    let sh = world.space.height;
    let ow = VIEW_GRID.min(sw);
    let oh = VIEW_GRID.min(sh);
    let n_out = (ow * oh) as usize;
    let mut cells = vec![0u8; n_out];
    let mut fertility = vec![0u8; n_out];
    let mut strain = vec![0u8; n_out];
    for oy in 0..oh {
        for ox in 0..ow {
            let x0 = ox * sw / ow;
            let x1 = ((ox + 1) * sw / ow).max(x0 + 1);
            let y0 = oy * sh / oh;
            let y1 = ((oy + 1) * sh / oh).max(y0 + 1);
            let (mut sr, mut sf, mut ss) = (0.0f32, 0.0f32, 0.0f32);
            let mut n = 0u32;
            for yy in y0..y1 {
                for xx in x0..x1 {
                    let i = (yy * sw + xx) as usize;
                    sr += world.resources.cell[i];
                    sf += world.resources.fertility[i];
                    ss += world.resources.strain[i];
                    n += 1;
                }
            }
            let nf = n.max(1) as f32;
            let o = (oy * ow + ox) as usize;
            cells[o] = ((sr / nf / max_per_cell).clamp(0.0, 1.0) * 255.0) as u8;
            fertility[o] = ((sf / nf).clamp(0.0, 1.0) * 255.0) as u8;
            strain[o] = ((ss / nf).clamp(0.0, 1.0) * 255.0) as u8;
        }
    }
    ResourceView { w: ow, h: oh, cells, fertility, strain }
}

fn event_view(e: &Event) -> Option<EventView> {
    let (kind, subjects, note): (&'static str, Vec<EntityId>, String) = match &e.kind {
        EventKind::EntitySpawned { entity } => ("naissance", vec![*entity], String::new()),
        EventKind::EntityDivided { parent, child } => {
            ("division", vec![*parent, *child], String::new())
        }
        EventKind::EntityDied { entity, .. } => ("mort", vec![*entity], String::new()),
        EventKind::ReplicationFailed { parent, .. } => ("echec", vec![*parent], String::new()),
        EventKind::WorldCreated => ("genese", vec![], String::new()),
        EventKind::LineageExtinct { lineage } => (
            "lignee_eteinte",
            vec![],
            format!("lignee {}", names::lineage_name(*lineage)),
        ),
        EventKind::SpeciesEmerged { species, size } => (
            "espece",
            vec![],
            format!("{}, {} individus", names::species_name(*species), size),
        ),
        EventKind::PopulationMilestone { level } => {
            ("palier", vec![], format!("{} individus", level))
        }
        EventKind::PopulationCrash { from, to } => {
            ("effondrement", vec![], format!("{} vers {}", from, to))
        }
        EventKind::CellFormed { cell, size } => {
            ("cellule", vec![], format!("cellule {}, {} membres", cell, size))
        }
        EventKind::CellDissolved { cell } => {
            ("cellule_dissoute", vec![], format!("cellule {}", cell))
        }
        EventKind::AgentAwoke { entity } => ("agent_eveille", vec![*entity], String::new()),
        EventKind::AgentLapsed { entity } => ("agent_endormi", vec![*entity], String::new()),
        // EntityAte et SnapshotTaken sont trop bruyants, on ne les remonte pas.
        _ => return None,
    };
    Some(EventView {
        tick: e.tick,
        kind,
        salience: e.salience,
        subjects,
        causes: e.causes.clone(),
        note,
    })
}
