//! Nodyx Genesis, coeur du moteur.
//!
//! Perimetre 0.0.1 : deux entites, energie, mouvement, reproduction, mutation, mort,
//! persistance, graine deterministe. Pas de memoire, pas de LLM, pas de Nodyx.
//!
//! Invariants respectes (voir BIBLE/02_ARCHITECTURE.md) :
//!  - le coeur ne depend d'aucun rendu (tranchee 3),
//!  - meme graine plus meme config plus meme version = meme monde, tick par tick (tranchee 5),
//!  - toute mutation importante devient un evenement immuable (invariant 4),
//!  - l'etat du RNG fait partie du World State, donc des instantanes (tranchee 5),
//!  - le mouvement n'est pas un evenement, le journal ne porte que le squelette causal (tranchee 17).

pub mod cognition;
pub mod config;
pub mod entity;
pub mod event;
pub mod genome;
pub mod persist;
pub mod rng;
pub mod sim;
pub mod spatial;
pub mod world;

pub use cognition::{BehaviorMode, Memory, MemoryKind, Mind, Needs, Shock, SocialTie};
pub use config::SimConfig;
pub use entity::{Action, Entity, EntityId, Position};
pub use event::{DeathCause, Event, EventKind};
pub use genome::{Genome, Traits};
pub use persist::WorldDir;
pub use rng::Rng;
pub use sim::{profile_dump, tick};
pub use world::{Cell, ResourceField, Space, WorldState};

/// Version du moteur, inscrite dans chaque monde (tranchee 17 : un monde sait qui l'a fait naitre).
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Version du schema de persistance. Change quand la forme du World State change.
/// v2 : genome a 7 traits (ajout de `cohesion`). Les instantanes v1 ne se rechargent plus.
/// v3 : les entites sont un `Vec` trie par id (tableau JSON) au lieu d'un `BTreeMap` (objet).
/// v4 : matiere structurelle (briques), un scalaire `WorldState.free_matter` conserve.
/// v5 : cellules (`WorldState.cells`, `Entity.cell_id`) : la premiere marche de l'escalier.
/// v6 : `seq` d'evenement attribue a la creation (`WorldState.next_event_seq`), tracabilite
///      causale de base (`Watch.deaths_since_check`, `last_death_seq_by_lineage`).
/// v7 : cognition (`Entity.mind`, `Entity.last_shock`) : la marche « individu » de
///      l'escalier. Un agent porte une memoire episodique qui biaise son deplacement.
/// v8 : souvenirs ancres (`MemoryKind::Witnessed`, `Memory.event_seq` peuple pour les morts
///      vues) : la memoire subjective pointe le fait objectif (invariant 5).
/// v9 : besoins (`Mind.needs` : faim, peur, solitude) : l'agent a un etat interne qui
///      pondere ses choix, plus seulement un reflexe.
/// v10 : genome a 9 traits (`caution`, `curiosity`) : la personnalite est heritee et
///       selectionnee, plus derivee des traits de corps.
/// v11 : modele de comportement lisible (`Mind.mode` : l'agent choisit explicitement entre
///       manger, fuir, suivre, chercher une aubaine, errer).
/// v12 : souvenirs sociaux (`Mind.social` : relations vers d'autres agents, familiarite et
///       valence). Premier pas vers les groupes.
pub const SCHEMA_VERSION: u32 = 12;
