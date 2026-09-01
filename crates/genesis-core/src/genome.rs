//! Genome et traits.
//!
//! Sept traits numeriques normalises dans [0, 1]. Le LLM n'est jamais requis pour calculer
//! la genetique.
//!
//! 0.0.1 est le stade molecule : reproduction asexuee par scission (`divide`). Une
//! molecule accumule de l'energie, se scinde en deux, chaque moitie copie le genome avec
//! micro-mutation. Une mutation peut etre letale : pas d'enfant. La reproduction sexuee
//! (combiner deux genomes) arrive a 0.0.2, et n'est jamais imposee : c'est une strategie
//! heritable qui ne l'emporte que sous une pression environnementale changeante.

use serde::{Deserialize, Serialize};

use crate::config::ReproductionCfg;
use crate::entity::EntityId;
use crate::rng::Rng;

/// Nombre de traits du genome. Un seul point a changer si on en ajoute.
pub const N_TRAITS: usize = 9;

/// Les traits de corps : les `SPECIES_TRAITS` premiers. La signature d'espece
/// (`genome_key`) ne porte que sur eux ; la personnalite (`caution`, `curiosity`) est un
/// calque comportemental, elle ne fait pas d'une population une espece distincte.
pub const SPECIES_TRAITS: usize = 7;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Traits {
    pub metabolism: f32,
    pub speed: f32,
    pub perception: f32,
    pub efficiency: f32,
    pub fertility: f32,
    pub lifespan: f32,
    /// Tendance a s'agreger : haut = l'entite se colle a ses voisins proches et forme des
    /// colonies. Evolue comme les autres traits.
    pub cohesion: f32,
    /// Personnalite (0.0.3, tranche 5), heritee, indice 7. Haut = un agent evite plus fort
    /// ses souvenirs de danger.
    #[serde(default = "half")]
    pub caution: f32,
    /// Personnalite, heritee, indice 8. Haut = un agent est plus attire par ses souvenirs
    /// de lieux d'abondance et explore davantage.
    #[serde(default = "half")]
    pub curiosity: f32,
}

fn half() -> f32 {
    0.5
}

impl Traits {
    pub fn as_array(&self) -> [f32; N_TRAITS] {
        [
            self.metabolism,
            self.speed,
            self.perception,
            self.efficiency,
            self.fertility,
            self.lifespan,
            self.cohesion,
            self.caution,
            self.curiosity,
        ]
    }

    fn from_array(a: [f32; N_TRAITS]) -> Self {
        Traits {
            metabolism: a[0],
            speed: a[1],
            perception: a[2],
            efficiency: a[3],
            fertility: a[4],
            lifespan: a[5],
            cohesion: a[6],
            caution: a[7],
            curiosity: a[8],
        }
    }

    fn random(rng: &mut Rng) -> Self {
        let mut a = [0.0f32; N_TRAITS];
        for v in a.iter_mut() {
            // demarre au milieu de la plage, laisse l'evolution explorer
            *v = 0.35 + rng.next_f32() * 0.3;
        }
        Traits::from_array(a)
    }

    /// Chaque trait quantifie en niveau 0..3. Sert a regrouper les genomes proches
    /// (identification d'espece) et a l'affichage du brin d'ADN.
    pub fn quantized(&self) -> [u8; N_TRAITS] {
        let a = self.as_array();
        let mut q = [0u8; N_TRAITS];
        for i in 0..N_TRAITS {
            q[i] = (a[i] * 4.0).clamp(0.0, 3.0) as u8;
        }
        q
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub traits: Traits,
    pub generation: u32,
    /// Ascendance. En 0.0.1, au plus un parent (scission).
    pub parent: Option<EntityId>,
    /// Lignee : l'indice du fondateur dont cette entite descend. Sert au suivi
    /// phylogenetique (combien de lignees survivent, laquelle domine).
    pub lineage: u16,
}

impl Genome {
    pub fn founder(rng: &mut Rng, lineage: u16) -> Self {
        Genome {
            traits: Traits::random(rng),
            generation: 0,
            parent: None,
            lineage,
        }
    }

    /// Scission asexuee : la copie du genome du parent, trait par trait, avec une chance
    /// de mutation gaussienne par trait. Renvoie `None` si une mutation letale frappe
    /// (membrane instable) : la division a eu lieu, mais il n'y a pas d'enfant viable.
    pub fn divide(
        parent: &Genome,
        parent_id: EntityId,
        cfg: &ReproductionCfg,
        rng: &mut Rng,
    ) -> Option<Genome> {
        let src = parent.traits.as_array();
        let mut child = [0.0f32; N_TRAITS];
        for i in 0..N_TRAITS {
            child[i] = src[i];
            if rng.chance(cfg.mutation_rate) {
                child[i] = (child[i] + rng.gaussian(cfg.mutation_scale)).clamp(0.0, 1.0);
            }
        }
        // Un tirage de letalite par division, apres les mutations.
        if rng.chance(cfg.lethal_mutation_rate) {
            return None;
        }
        Some(Genome {
            traits: Traits::from_array(child),
            generation: parent.generation + 1,
            parent: Some(parent_id),
            lineage: parent.lineage,
        })
    }
}
