//! World State : la verite objective du monde a un instant donne.
//!
//! Seuls les systemes de simulation ecrivent dedans (voir sim.rs). Le LLM et Nodyx
//! n'y touchent jamais.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::config::SimConfig;
use crate::entity::{Action, Entity, EntityId, Position};
use crate::genome::{Genome, N_TRAITS};
use crate::rng::Rng;
use crate::{ENGINE_VERSION, SCHEMA_VERSION};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceField {
    /// Energie disponible par case. Longueur = width * height, indexee ligne par ligne.
    pub cell: Vec<f32>,
    /// Fertilite statique par case, dans [0, 1]. Plafond et vitesse de regeneration de la
    /// case en sont proportionnels : le monde a des zones riches et des zones mortes.
    /// C'est la premiere structure spatiale, l'ancetre des biomes.
    pub fertility: Vec<f32>,
    /// Tension de surexploitation par case. Monte avec la recolte, decroit lentement,
    /// freine la regeneration. Trace laissee par l'activite des entites sur le milieu.
    pub strain: Vec<f32>,
}

impl ResourceField {
    pub fn index(&self, space: &Space, p: Position) -> usize {
        let cx = (p.x.floor() as i64).clamp(0, space.width as i64 - 1) as usize;
        let cy = (p.y.floor() as i64).clamp(0, space.height as i64 - 1) as usize;
        cy * space.width as usize + cx
    }

    pub fn total(&self) -> f64 {
        self.cell.iter().map(|&c| c as f64).sum()
    }

    /// Champ de fertilite : quelques bosses radiales douces sur un fond pauvre.
    /// Deterministe, tire du RNG du monde a la creation.
    fn make_fertility(space: &Space, rng: &mut Rng) -> Vec<f32> {
        let w = space.width as usize;
        let h = space.height as usize;
        let mut fert = vec![0.12f32; w * h];
        let bumps = 14;
        for _ in 0..bumps {
            let bx = rng.next_f32() * space.width as f32;
            let by = rng.next_f32() * space.height as f32;
            let br = 6.0 + rng.next_f32() * 20.0;
            let amp = 0.35 + rng.next_f32() * 0.65;
            let inv = 1.0 / (br * br);
            for y in 0..h {
                for x in 0..w {
                    let dx = x as f32 + 0.5 - bx;
                    let dy = y as f32 + 0.5 - by;
                    fert[y * w + x] += amp * (-(dx * dx + dy * dy) * inv).exp();
                }
            }
        }
        for f in fert.iter_mut() {
            *f = f.clamp(0.0, 1.0);
        }
        fert
    }
}

/// Une cellule (0.0.2, tranche 2, etape 1). Un amas coherent de parents, reconnu comme
/// unite. En etape 1 les membres restent dans `WorldState.entities` (taggues `cell_id`) ;
/// ce bilan est rafraichi a chaque tick par la phase 5b. L'etape 2 de-simulera les membres.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: u32,
    pub formed_tick: u64,
    /// Centroide des membres.
    pub position: Position,
    /// Rayon moyen des membres autour du centroide.
    pub radius: f32,
    pub member_count: u32,
    /// Cle de genome quantifiee (comme l'espece) : la signature de la cellule.
    pub genome_key: u16,
    /// Moyenne de chaque trait sur les membres. Sert au brin d'ADN et aux stats.
    pub mean_traits: [f32; N_TRAITS],
}

/// Etat des veilleurs. Fait partie du World State (donc des instantanes et du rejeu).
/// Seule la phase 8b de `sim.rs` y ecrit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watch {
    /// Population aux derniers controles, pour la detection d'effondrement.
    pub pop_history: Vec<u32>,
    /// Plus haut palier de population deja franchi.
    pub milestone_hi: u32,
    /// Cle de genome quantifiee -> identifiant d'espece attribue.
    pub species: BTreeMap<u16, u32>,
    /// Cle candidate -> nombre de controles consecutifs tenus.
    pub species_streak: BTreeMap<u16, u16>,
    pub next_species_id: u32,
    /// Nombre de lignees fondatrices vivantes au dernier controle.
    pub lineages: u16,
    /// Amas candidats a la formation d'une cellule : centroide et nombre de controles
    /// consecutifs tenus. Sert a exiger la persistance avant de former (0.0.2, tranche 2).
    #[serde(default)]
    pub cell_pending: Vec<(Position, u16)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub world_id: u64,
    pub seed: u64,
    pub tick: u64,
    pub schema_version: u32,
    pub engine_version: String,

    pub space: Space,
    pub resources: ResourceField,
    /// Toujours trie par `id` croissant, dense. Les nouveaux nes ont un id superieur a
    /// tous les existants (`next_entity_id` est monotone), donc un simple `push` garde le
    /// tri. `Vec` plutot que `BTreeMap` : parcours et parallelisme bien plus rapides.
    pub entities: Vec<Entity>,
    pub next_entity_id: EntityId,

    /// Cellules vivantes (0.0.2, tranche 2). Triees par `id` croissant. En etape 1 leurs
    /// membres sont encore dans `entities`, taggues `cell_id`.
    #[serde(default)]
    pub cells: Vec<Cell>,
    #[serde(default)]
    pub next_cell_id: u32,

    /// L'etat du RNG fait partie du World State (tranchee 5). Il est donc dans les instantanes.
    pub rng: Rng,

    /// Matiere structurelle libre du monde (briques, 0.0.2). Le reste de la matiere totale
    /// est immobilise dans les corps vivants : `free_matter + population * body_matter` est
    /// constant. Une division la ponctionne, une mort la reapprovisionne.
    #[serde(default)]
    pub free_matter: f32,

    pub births_total: u64,
    pub deaths_total: u64,
    pub deaths_starvation: u64,
    pub deaths_age: u64,
    /// Divisions par ailleurs eligibles qui ont echoue faute de matiere libre. Cumule.
    /// Rend visible quand la matiere est le facteur limitant.
    #[serde(default)]
    pub repro_blocked_materials: u64,
    /// Cellules formees et dissoutes depuis le debut du monde. Cumule.
    #[serde(default)]
    pub cells_formed_total: u64,
    #[serde(default)]
    pub cells_dissolved_total: u64,

    pub watch: Watch,
}

impl WorldState {
    /// Cree un monde neuf : ressources remplies, deux entites fondatrices.
    pub fn new(seed: u64, cfg: &SimConfig) -> Self {
        let mut rng = Rng::from_seed(seed);
        let w = cfg.world.grid_width;
        let h = cfg.world.grid_height;

        let space = Space { width: w, height: h };
        let fertility = ResourceField::make_fertility(&space, &mut rng);
        // Chaque case demarre remplie a `initial_fill` de son propre plafond (fertilite).
        let max = cfg.resources.max_per_cell;
        let fill = cfg.resources.initial_fill;
        let cell: Vec<f32> = fertility.iter().map(|&f| max * f * fill).collect();
        let strain = vec![0.0f32; cell.len()];

        let mut entities: Vec<Entity> = Vec::with_capacity(2);
        for id in 0u64..2 {
            let genome = Genome::founder(&mut rng, id as u16);
            // Reproduction asexuee : chaque fondateur amorce sa lignee seul, pas besoin
            // de se rencontrer. On les repartit dans la moitie centrale de la grille pour
            // qu'ils aient de la place et restent visibles.
            let position = Position {
                x: (w as f32 * 0.25 + rng.next_f32() * w as f32 * 0.5).clamp(0.0, w as f32 - 0.001),
                y: (h as f32 * 0.25 + rng.next_f32() * h as f32 * 0.5).clamp(0.0, h as f32 - 0.001),
            };
            entities.push(Entity {
                id,
                genome,
                position,
                energy: cfg.reproduction.energy_threshold * 0.8,
                // Les fondateurs sont des organismes deja etablis, pas des oeufs :
                // ils demarrent adultes pour ne pas etre bloques par la maturite.
                age_ticks: (cfg.lifecycle.lifespan_ticks_mean as f32 * 0.15) as u64,
                cooldown: 0,
                last_action: Action::Forage,
                target: None,
                colony_support: 0.0,
                cell_id: None,
            });
        }

        WorldState {
            world_id: seed,
            seed,
            tick: 0,
            schema_version: SCHEMA_VERSION,
            engine_version: ENGINE_VERSION.to_string(),
            space,
            resources: ResourceField { cell, fertility, strain },
            entities,
            next_entity_id: 2,
            cells: Vec::new(),
            next_cell_id: 0,
            rng,
            // Matiere totale = matter_per_cell * cases ; deux corps fondateurs deja batis.
            free_matter: (cfg.bricks.matter_per_cell * (w as f32) * (h as f32)
                - 2.0 * cfg.bricks.body_matter)
                .max(0.0),
            births_total: 0,
            deaths_total: 0,
            deaths_starvation: 0,
            deaths_age: 0,
            repro_blocked_materials: 0,
            cells_formed_total: 0,
            cells_dissolved_total: 0,
            watch: Watch {
                pop_history: Vec::new(),
                milestone_hi: 0,
                species: BTreeMap::new(),
                species_streak: BTreeMap::new(),
                next_species_id: 0,
                lineages: 2,
                cell_pending: Vec::new(),
            },
        }
    }

    /// Reference vers l'entite `id` (recherche binaire, le `Vec` est trie par id).
    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.entities
            .binary_search_by_key(&id, |e| e.id)
            .ok()
            .map(|i| &self.entities[i])
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        match self.entities.binary_search_by_key(&id, |e| e.id) {
            Ok(i) => Some(&mut self.entities[i]),
            Err(_) => None,
        }
    }

    /// Reference mutable vers la cellule `id` (les cellules sont triees par id).
    pub fn cell_mut(&mut self, id: u32) -> Option<&mut Cell> {
        match self.cells.binary_search_by_key(&id, |c| c.id) {
            Ok(i) => Some(&mut self.cells[i]),
            Err(_) => None,
        }
    }

    pub fn population(&self) -> u32 {
        self.entities.len() as u32
    }

    pub fn mean_age(&self) -> f64 {
        if self.entities.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.entities.iter().map(|e| e.age_ticks).sum();
        sum as f64 / self.entities.len() as f64
    }

    /// Diversite genetique : distance moyenne entre traits, sur un echantillon de paires.
    pub fn genetic_diversity(&self) -> f32 {
        let list: Vec<[f32; N_TRAITS]> =
            self.entities.iter().map(|e| e.genome.traits.as_array()).collect();
        if list.len() < 2 {
            return 0.0;
        }
        let mut acc = 0.0f32;
        let mut count = 0u32;
        for i in 0..list.len() {
            for j in (i + 1)..list.len() {
                let mut d = 0.0f32;
                for k in 0..N_TRAITS {
                    d += (list[i][k] - list[j][k]).abs();
                }
                acc += d / N_TRAITS as f32;
                count += 1;
                if count >= 2000 {
                    return acc / count as f32;
                }
            }
        }
        acc / count as f32
    }

    /// Moyenne et ecart-type de chaque trait sur la population vivante.
    pub fn trait_stats(&self) -> ([f32; N_TRAITS], [f32; N_TRAITS]) {
        let n = self.entities.len();
        if n == 0 {
            return ([0.0; N_TRAITS], [0.0; N_TRAITS]);
        }
        let mut mean = [0.0f64; N_TRAITS];
        for e in self.entities.iter() {
            let a = e.genome.traits.as_array();
            for k in 0..N_TRAITS {
                mean[k] += a[k] as f64;
            }
        }
        for m in mean.iter_mut() {
            *m /= n as f64;
        }
        let mut var = [0.0f64; N_TRAITS];
        for e in self.entities.iter() {
            let a = e.genome.traits.as_array();
            for k in 0..N_TRAITS {
                let d = a[k] as f64 - mean[k];
                var[k] += d * d;
            }
        }
        let mut mean_f = [0.0f32; N_TRAITS];
        let mut sd_f = [0.0f32; N_TRAITS];
        for k in 0..N_TRAITS {
            mean_f[k] = mean[k] as f32;
            sd_f[k] = (var[k] / n as f64).sqrt() as f32;
        }
        (mean_f, sd_f)
    }

    /// Quantiles p10, p50 (mediane), p90 de chaque trait sur la population vivante.
    /// `[p10; p50; p90]`. Montre la distribution qui s'elargit ou se scinde (bimodale =
    /// speciation), ce que moyenne plus ecart-type cache. Echantillon deterministe :
    /// toutes les entites jusqu'a un plafond, sinon un pas fixe par ordre d'id.
    pub fn trait_quantiles(&self) -> [[f32; N_TRAITS]; 3] {
        let n = self.entities.len();
        if n == 0 {
            return [[0.0; N_TRAITS]; 3];
        }
        const CAP: usize = 6000;
        let step = n.div_ceil(CAP); // 1 si n <= CAP
        let sample: Vec<[f32; N_TRAITS]> = self
            .entities
            .iter()
            .step_by(step)
            .map(|e| e.genome.traits.as_array())
            .collect();
        let m = sample.len();
        let mut out = [[0.0f32; N_TRAITS]; 3];
        let mut col = vec![0.0f32; m];
        for k in 0..N_TRAITS {
            for (i, row) in sample.iter().enumerate() {
                col[i] = row[k];
            }
            col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            for (qi, &q) in [0.1f32, 0.5, 0.9].iter().enumerate() {
                let idx = ((q * (m as f32 - 1.0)).round() as usize).min(m - 1);
                out[qi][k] = col[idx];
            }
        }
        out
    }

    /// Moyenne et ecart-type de la generation genomique sur les entites vivantes.
    /// L'axe « generations ecoulees » du graphe d'evolution.
    pub fn generation_stats(&self) -> (f32, f32) {
        let n = self.entities.len();
        if n == 0 {
            return (0.0, 0.0);
        }
        let mean: f64 =
            self.entities.iter().map(|e| e.genome.generation as f64).sum::<f64>() / n as f64;
        let var: f64 = self
            .entities
            .iter()
            .map(|e| {
                let d = e.genome.generation as f64 - mean;
                d * d
            })
            .sum::<f64>()
            / n as f64;
        (mean as f32, var.sqrt() as f32)
    }

    /// Suivi phylogenetique : nombre de lignees fondatrices encore vivantes, part de la
    /// plus repandue, generation maximale atteinte.
    pub fn lineage_stats(&self) -> (u16, f32, u32) {
        if self.entities.is_empty() {
            return (0, 0.0, 0);
        }
        let mut counts: std::collections::BTreeMap<u16, u32> = std::collections::BTreeMap::new();
        let mut max_gen = 0u32;
        for e in self.entities.iter() {
            *counts.entry(e.genome.lineage).or_insert(0) += 1;
            max_gen = max_gen.max(e.genome.generation);
        }
        let total: u32 = counts.values().sum();
        let top = counts.values().copied().max().unwrap_or(0);
        (counts.len() as u16, top as f32 / total as f32, max_gen)
    }

    /// Signature de l'espece dominante : les traits quantifies (0..3) du genome le plus
    /// repandu dans la population. Sert au brin d'ADN du lecteur.
    pub fn dominant_genome(&self) -> [u8; N_TRAITS] {
        let mut counts: BTreeMap<[u8; N_TRAITS], u32> = BTreeMap::new();
        for e in self.entities.iter() {
            *counts.entry(e.genome.traits.quantized()).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)))
            .map(|(k, _)| k)
            .unwrap_or([0; N_TRAITS])
    }

    /// Energie totale portee par les entites vivantes (biomasse energetique).
    pub fn biomass_energy(&self) -> f64 {
        self.entities.iter().map(|e| e.energy.max(0.0) as f64).sum()
    }

    /// Nombre de cases distinctes occupees par au moins une entite.
    pub fn occupied_cells(&self) -> u32 {
        let mut set: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for e in self.entities.iter() {
            set.insert(self.resources.index(&self.space, e.position));
        }
        set.len() as u32
    }

    /// Etat du milieu : ressource moyenne, fraction de cases epuisees (sous 10 % de leur
    /// plafond de fertilite), tension de surexploitation moyenne.
    pub fn environment_stats(&self, max_per_cell: f32) -> (f32, f32, f32) {
        let n = self.resources.cell.len();
        if n == 0 {
            return (0.0, 0.0, 0.0);
        }
        let mut sum = 0.0f64;
        let mut depleted = 0u32;
        let mut live = 0u32;
        for i in 0..n {
            let f = self.resources.fertility[i];
            let cap = max_per_cell * f;
            sum += self.resources.cell[i] as f64;
            if cap > 0.05 {
                live += 1;
                if self.resources.cell[i] < cap * 0.1 {
                    depleted += 1;
                }
            }
        }
        let strain_sum: f64 = self.resources.strain.iter().map(|&s| s as f64).sum();
        (
            (sum / n as f64) as f32,
            if live > 0 { depleted as f32 / live as f32 } else { 0.0 },
            (strain_sum / n as f64) as f32,
        )
    }

    /// Etat de la matiere structurelle (briques). Renvoie : matiere libre, capacite de
    /// charge (matiere totale / `body_matter`, le plafond de population que la matiere
    /// permet), fraction de la matiere totale immobilisee dans les corps vivants.
    /// Au plateau, la fraction immobilisee approche 1 et la matiere libre est basse.
    pub fn matter_stats(&self, body_matter: f32) -> (f32, u32, f32) {
        let body = body_matter.max(1e-6);
        let locked = self.entities.len() as f32 * body;
        let total = (self.free_matter + locked).max(1e-6);
        (self.free_matter, (total / body) as u32, (locked / total).clamp(0.0, 1.0))
    }

    /// Etat des cellules : cellules vivantes, entites en cellule (somme des effectifs),
    /// taille moyenne. En etape 1 les membres sont encore dans `entities`.
    pub fn cell_stats(&self) -> (u32, u32, f32) {
        if self.cells.is_empty() {
            return (0, 0, 0.0);
        }
        let members: u32 = self.cells.iter().map(|c| c.member_count).sum();
        (self.cells.len() as u32, members, members as f32 / self.cells.len() as f32)
    }
}
