//! Entite et position.
//!
//! En 0.0.1, une entite est un organisme sans cognition. L'energie appartient a l'entite,
//! dans le World State. Seul le systeme Metabolisme l'ecrit (voir sim.rs).

use serde::{Deserialize, Serialize};

use crate::cognition::{Mind, Shock};
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

    /// Esprit d'agent (0.0.3, tranche 1). `None` = organisme sans cognition (la quasi
    /// totalite). Attache quand l'entite s'eveille (phase 5c), retire si elle retombe.
    /// `Box` : l'esprit est plus gros que le reste de l'entite et rare, on ne veut pas
    /// alourdir chaque `Entity` du `Vec`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mind: Option<Box<Mind>>,

    /// Dernier choc marquant (famine ou aubaine), ecrit pour toutes les entites en phase 5.
    /// Graine d'un futur souvenir : lu a l'eveil et a chaque tick par un agent.
    #[serde(default)]
    pub last_shock: Option<Shock>,

    /// Sante (0.0.3, tranche 8) : la condition biologique consolidee, dans [0, 1]. `1.0` =
    /// corps intact. Integre lentement les famines repetees et la vieillesse (phase 6), au
    /// lieu que le pipeline rejuge l'energie brute et l'age a chaque tick. C'est la biologie
    /// devenue etat de fond : la cognition (la biographie) lit ce scalaire, pas le detail.
    #[serde(default = "one")]
    pub health: f32,

    /// Tick du dernier appel de nourriture emis par cet agent (Voix tranche 2, schema v17).
    /// Sert a espacer ses appels. `0` = jamais.
    #[serde(default)]
    pub call_born: u64,
}

fn one() -> f32 {
    1.0
}
