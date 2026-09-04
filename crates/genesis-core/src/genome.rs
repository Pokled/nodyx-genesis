//! Genome et traits.
//!
//! Dix traits numeriques normalises dans [0, 1]. Le LLM n'est jamais requis pour calculer
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
pub const N_TRAITS: usize = 10;

/// Les traits de corps : les `SPECIES_TRAITS` premiers. La signature d'espece
/// (`genome_key`) ne porte que sur eux ; la personnalite (`caution`, `curiosity`) et la
/// tolerance a la chaleur (`heat_tol`, un ecotype pas encore une espece) sont des calques,
/// elles ne font pas d'une population une espece distincte.
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
    /// Tolerance a la chaleur (0.0.4, saisons thermiques, schema v18), heritee, indice 9.
    /// Deplace la temperature a laquelle le metabolisme de l'entite est le moins cher :
    /// `0` = adapte au froid (`temp_optimal_c - span/2`), `1` = adapte au chaud (`+ span/2`).
    /// Premier axe genetique qui repond au climat : sous une saison thermique il derive vers
    /// l'adaptation a la saison la plus dure et garde de l'etalement (voir `experiments/011`).
    #[serde(default = "half")]
    pub heat_tol: f32,
}

fn half() -> f32 {
    0.5
}

/// Genome structurel (0.0.2, piste D, distinct du genome de traits ci-dessus). Chaque gene n'est
/// tire/mute du RNG que si SON levier est actif (`[cells]`) -- sinon il reste fige au neutre
/// `0,5`, SANS consommer de tirage : `false` doit laisser la trajectoire du monde strictement
/// inchangee, comme tout autre levier de cette base (voir `experiments/018`, le decalage de
/// flux RNG inconditionnel avait casse cinq tests sans rapport avant d'etre corrige).
///
/// - `adhesion` : la tolerance PERSONNELLE d'une cellule a la distance de parente mesuree par
///   `trait_l1` -- pas une dimension de plus dans cette mesure (qui casserait l'echelle deja
///   calibree de `fuse_kin`/`tissue_kin`/`kin_dist`), mais un multiplicateur heritable du seuil
///   d'adhesion de tissu (`[cells] adhesion_gene`).
/// - `germinal_bias` : le seuil PERSONNEL (pas moyenne par cellule, contrairement a `adhesion` --
///   `experiments/018` a montre que moyenner par cellule dilue la selection) d'entassement
///   (`Cell.tissue_bonds`, deja mesure par `tissue_shelter`) a partir duquel une entite se
///   compte germinale plutot que somatique (`[cells] role_gene`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StructuralGenome {
    pub adhesion: f32,
    #[serde(default = "half")]
    pub germinal_bias: f32,
}

impl Default for StructuralGenome {
    fn default() -> Self {
        StructuralGenome { adhesion: 0.5, germinal_bias: 0.5 }
    }
}

impl StructuralGenome {
    fn random(rng: &mut Rng, cc: &crate::config::CellsCfg) -> Self {
        StructuralGenome {
            adhesion: if cc.adhesion_gene { 0.35 + rng.next_f32() * 0.3 } else { 0.5 },
            germinal_bias: if cc.role_gene { 0.35 + rng.next_f32() * 0.3 } else { 0.5 },
        }
    }

    fn divide(
        parent: &StructuralGenome,
        cc: &crate::config::CellsCfg,
        repro: &ReproductionCfg,
        rng: &mut Rng,
    ) -> Self {
        let mut adhesion = parent.adhesion;
        if cc.adhesion_gene && rng.chance(repro.mutation_rate) {
            adhesion = (adhesion + rng.gaussian(repro.mutation_scale)).clamp(0.0, 1.0);
        }
        let mut germinal_bias = parent.germinal_bias;
        if cc.role_gene && rng.chance(repro.mutation_rate) {
            germinal_bias = (germinal_bias + rng.gaussian(repro.mutation_scale)).clamp(0.0, 1.0);
        }
        StructuralGenome { adhesion, germinal_bias }
    }
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
            self.heat_tol,
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
            heat_tol: a[9],
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
    /// Genome structurel (0.0.2, piste D etape 1). Distinct du genome de traits, mute
    /// separement. `#[serde(default)]` : absent des sauvegardes d'avant ce champ.
    #[serde(default)]
    pub structural: StructuralGenome,
    pub generation: u32,
    /// Ascendance. En 0.0.1, au plus un parent (scission).
    pub parent: Option<EntityId>,
    /// Lignee : l'indice du fondateur dont cette entite descend. Sert au suivi
    /// phylogenetique (combien de lignees survivent, laquelle domine).
    pub lineage: u16,
}

impl Genome {
    /// `cc` : chaque gene structurel n'est tire du RNG que si SON levier (`[cells]`) est actif
    /// -- sinon fige au neutre, SANS consommer de tirage. Comme pour tout autre levier de
    /// cette base : `false` doit laisser la trajectoire du monde strictement inchangee, pas
    /// seulement l'effet du gene.
    pub fn founder(rng: &mut Rng, lineage: u16, cc: &crate::config::CellsCfg) -> Self {
        Genome {
            traits: Traits::random(rng),
            structural: StructuralGenome::random(rng, cc),
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
        cc: &crate::config::CellsCfg,
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
        // Meme regle que `founder` : chaque gene structurel ne tire du RNG que si son propre
        // levier est actif (`false` = trajectoire strictement inchangee).
        let structural = StructuralGenome::divide(&parent.structural, cc, cfg, rng);
        // Un tirage de letalite par division, apres les mutations.
        if rng.chance(cfg.lethal_mutation_rate) {
            return None;
        }
        Some(Genome {
            traits: Traits::from_array(child),
            structural,
            generation: parent.generation + 1,
            parent: Some(parent_id),
            lineage: parent.lineage,
        })
    }
}
