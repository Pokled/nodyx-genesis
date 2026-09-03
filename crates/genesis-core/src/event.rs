//! Evenements.
//!
//! Invariant 4 : toute mutation importante devient un evenement, les evenements sont
//! immuables, une correction est un nouvel evenement.
//! Tranchee 17 : le mouvement n'est PAS un evenement. Le journal ne porte que le
//! squelette causal : naissance, repas notable, reproduction, mort, instantane.

use serde::{Deserialize, Serialize};

use crate::entity::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeathCause {
    Starvation,
    Age,
}

impl DeathCause {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeathCause::Starvation => "starvation",
            DeathCause::Age => "age",
        }
    }
}

/// Pourquoi une division n'a pas donne d'enfant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplicationFail {
    /// Mutation letale (membrane instable). Cote genome.
    LethalMutation,
    /// Echec d'origine environnementale, etat sans infrastructure.
    Environment,
    /// Pas assez de briques elementaires pour batir un enfant (0.0.2).
    Materials,
}

impl ReplicationFail {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReplicationFail::LethalMutation => "lethal_mutation",
            ReplicationFail::Environment => "environment",
            ReplicationFail::Materials => "materials",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    WorldCreated,
    EntitySpawned { entity: EntityId },
    EntityAte { entity: EntityId, amount: f32 },
    /// Scission asexuee : `parent` s'est divise, `child` est ne.
    EntityDivided { parent: EntityId, child: EntityId },
    /// Division tentee, sans enfant viable.
    ReplicationFailed { parent: EntityId, reason: ReplicationFail },
    EntityDied { entity: EntityId, cause: DeathCause },
    SnapshotTaken { tick: u64 },

    // -- Evenements de veille : produits par des detecteurs mecanises (sim.rs phase 8b),
    //    jamais par une regle qui les nomme. Ce sont eux qui portent les chapitres.
    /// Une lignee fondatrice n'a plus aucun descendant vivant.
    LineageExtinct { lineage: u16 },
    /// Un groupe genetiquement distinct, assez nombreux et persistant, s'est detache du stock.
    SpeciesEmerged { species: u32, size: u32 },
    /// La population a franchi ce palier pour la premiere fois.
    PopulationMilestone { level: u32 },
    /// Chute rapide et forte de la population.
    PopulationCrash { from: u32, to: u32 },
    /// Un amas coherent de parents est devenu une cellule (0.0.2, tranche 2).
    CellFormed { cell: u32, size: u32 },
    /// Une cellule s'est dissoute : dispersion ou perte de ses membres. Bascule reversible.
    CellDissolved { cell: u32 },
    /// Deux cellules stables aux membranes chevauchantes et aux genomes proches ont fusionne :
    /// `absorbed` disparait dans `cell`, qui garde son identite. `size` = effectif combine,
    /// `at` = position de la cellule survivante (cases), pour que l'overlay pose un effet.
    CellsMerged { cell: u32, absorbed: u32, size: u32, at: [f32; 2] },
    /// Une cellule grande, mure et etiree s'est pincee en deux : `child` s'est detache de
    /// `parent`, avec `size` membres (0.0.4, schema v19). La cellule devient une unite qui se
    /// reproduit. `at` = position de la cellule mere (cases).
    CellDivided { parent: u32, child: u32, size: u32, at: [f32; 2] },
    /// La cle de genome la plus repandue du monde a bascule et s'est tenue : l'evolution a
    /// deplace le centre de la population. `generation` = generation moyenne a ce moment.
    GenomeShift { from: u16, to: u16, generation: u32 },
    /// Une entite s'est eveillee en agent : elle percoit assez, a vecu assez, et vient de
    /// subir un choc. Elle gagne une memoire (0.0.3, tranche 1).
    AgentAwoke { entity: EntityId },
    /// Un agent est retombe entite de fond : plus aucun souvenir depuis longtemps. Bascule
    /// reversible (la cognition n'est pas un aller simple).
    AgentLapsed { entity: EntityId },
}

impl EventKind {
    /// Saillance de base, 0 (bruit) a 255 (fondateur). Le flux et les chapitres filtrent
    /// dessus. La regle est mecanique, jamais un jugement de valeur.
    pub fn base_salience(&self) -> u8 {
        match self {
            EventKind::WorldCreated => 255,
            EventKind::SpeciesEmerged { .. } => 235,
            EventKind::GenomeShift { .. } => 234,
            EventKind::CellsMerged { .. } => 232,
            EventKind::CellDivided { .. } => 231,
            EventKind::CellFormed { .. } => 230,
            EventKind::LineageExtinct { .. } => 225,
            EventKind::AgentAwoke { .. } => 215,
            EventKind::PopulationCrash { .. } => 210,
            EventKind::CellDissolved { .. } => 200,
            EventKind::AgentLapsed { .. } => 195,
            EventKind::PopulationMilestone { .. } => 180,
            EventKind::EntityDied { .. } => 3,
            EventKind::EntitySpawned { .. } => 2,
            EventKind::EntityDivided { .. } => 2,
            EventKind::EntityAte { .. } => 0,
            EventKind::ReplicationFailed { .. } => 0,
            EventKind::SnapshotTaken { .. } => 0,
        }
    }

    /// Un evenement de chapitre : les grands tournants d'un monde, ceux qu'on garde pour
    /// toujours dans la chronique meme apres que le journal detaille a ete compacte. C'est un
    /// sous-ensemble volontairement etroit des evenements saillants : pas les eveils d'agent
    /// ni les formations de cellule (frequents), seulement ce qui fait l'histoire du monde.
    pub fn is_chapter(&self) -> bool {
        matches!(
            self,
            EventKind::WorldCreated
                | EventKind::SpeciesEmerged { .. }
                | EventKind::LineageExtinct { .. }
                | EventKind::PopulationCrash { .. }
                | EventKind::PopulationMilestone { .. }
                | EventKind::CellsMerged { .. }
                | EventKind::CellDivided { .. }
                | EventKind::GenomeShift { .. }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Numero d'ordre, monotone par monde. Attribue a la creation (dans `tick()`), pour que
    /// les evenements de veille puissent pointer les evenements qui les ont causes.
    pub seq: u64,
    pub tick: u64,
    pub kind: EventKind,
    /// Saillance 0..255. Copiee de `kind.base_salience()` a la creation, peut monter ensuite.
    pub salience: u8,
    /// seq des evenements qui ont cause celui-ci. Cable pour `PopulationCrash` (la vague de
    /// morts) et `LineageExtinct` (la derniere mort de la lignee) ; le reste a 0.0.6 (T-15).
    pub causes: Vec<u64>,
    pub cascade_depth: u16,
}

impl Event {
    pub fn now(tick: u64, kind: EventKind) -> Self {
        let salience = kind.base_salience();
        Event { seq: 0, tick, kind, salience, causes: Vec::new(), cascade_depth: 0 }
    }

    pub fn caused_by(mut self, causes: Vec<u64>) -> Self {
        self.causes = causes;
        self
    }
}
