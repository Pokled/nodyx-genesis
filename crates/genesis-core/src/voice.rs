//! La Voix (jalon 0.0.4) : la premiere primitive de signal.
//!
//! En 0.0.4 tranche 1, tous les signaux sont des **alarmes** : un agent qui subit le choc
//! d'une famine « crie » a l'endroit ou il se trouve. Les agents proches le percoivent et
//! leur peur monte d'un cran, sans qu'aucun souvenir ne se forme : une contagion breve.
//!
//! Aucun lexique n'est code. Les autres genres de signal (appel, ralliement, marquage) et
//! le pari du langage emergent viennent plus tard (`06_EMERGENCE.md`).

use serde::{Deserialize, Serialize};

use crate::entity::Position;

/// Un signal emis dans le monde. Vit quelques ticks puis s'efface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// Ou il a ete emis, en coordonnees monde.
    pub pos: Position,
    /// Tick d'emission.
    pub born: u64,
}
