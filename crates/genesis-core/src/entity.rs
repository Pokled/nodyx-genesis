//! Entite et position.
//!
//! En 0.0.1, une entite est un organisme sans cognition. L'energie appartient a l'entite,
//! dans le World State. Seul le systeme Metabolisme l'ecrit (voir sim.rs).

use serde::{Deserialize, Serialize};

use crate::genome::Genome;

pub type EntityId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn dist2(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Etat visible de l'entite au dernier tick. Sert au rendu, derive de ce qui s'est passe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Forage,
    Eat,
    Divide,
    Dying,
}

impl Default for Action {
    fn default() -> Self {
        Action::Forage
    }
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Forage => "forage",
            Action::Eat => "eat",
            Action::Divide => "divide",
            Action::Dying => "dying",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub genome: Genome,
    pub position: Position,
    pub energy: f32,
    pub age_ticks: u64,

    /// Ticks restants avant que l'entite puisse se reproduire de nouveau (gestation).
    #[serde(default)]
    pub cooldown: u32,

    #[serde(default)]
    pub last_action: Action,

    /// Cible de deplacement courante. Serialisee : elle n'est recalculee que tous les
    /// quelques ticks (replanification decalee), donc le rejeu depuis un instantane doit
    /// la retrouver telle quelle.
    #[serde(default)]
    pub target: Option<Position>,

    /// Support de colonie : somme (plafonnee) de la similarite genetique des voisins
    /// proches. Recalcule a la replanification (tous les quelques ticks), lu en phases 5
    /// et 7. Serialise pour que le rejeu depuis un instantane le retrouve.
    #[serde(default)]
    pub colony_support: f32,

    /// Cellule a laquelle l'entite appartient (0.0.2, tranche 2). `None` = molecule libre.
    /// Un membre partage l'energie du groupe et beneficie d'une reproduction protegee.
    #[serde(default)]
    pub cell_id: Option<u32>,
}
