//! La Voix (jalon 0.0.4) : les primitives de signal.
//!
//! Tranche 1 : l'**alarme**. Un agent qui subit le choc d'une famine « crie » a l'endroit ou
//! il se trouve. Les agents proches le percoivent et leur peur monte d'un cran, sans qu'aucun
//! souvenir ne se forme : une contagion breve.
//!
//! Tranche 2 : l'**appel**. Un agent qui tombe sur un repas exceptionnel lance un appel a sa
//! position. Les agents proches en train de decider ou aller inflechissent leur cible vers
//! lui. Deux genres fixes, comme le cri d'alarme et l'appel de nourriture d'un animal.
//!
//! Aucun lexique n'est code. Les genres qui evoluent (chaque lignee sa version), la
//! transmission avec perte, le pari du langage emergent viennent plus tard
//! (`experiments/010_belief_tranche.md`, `06_EMERGENCE.md`).

use serde::{Deserialize, Serialize};

use crate::entity::Position;

/// Le genre d'un signal. Deux entrees fixes en 0.0.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    /// Cri de detresse : un agent frole la mort par famine.
    #[default]
    Alarm,
    /// Appel : un agent a trouve un repas exceptionnel.
    Bounty,
}

/// Un signal emis dans le monde. Vit quelques ticks puis s'efface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// Ou il a ete emis, en coordonnees monde.
    pub pos: Position,
    /// Tick d'emission.
    pub born: u64,
    /// Genre (schema v17). Les signaux d'avant v17 se rechargent en `Alarm`.
    #[serde(default)]
    pub kind: SignalKind,
}
