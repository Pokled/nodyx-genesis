//! Cognition, 0.0.3 : la marche « individu » de l'escalier des echelles.
//!
//! Tranche 1 (« le premier souvenir ») : une entite qui perçoit assez bien, a vecu assez
//! longtemps et vient de subir un choc s'eveille en Agent. Elle gagne un `Mind` : une
//! memoire episodique spatiale, bornee, qui decroit. Cette memoire biaise son deplacement
//! (elle evite les lieux de peril, elle revient aux lieux d'aubaine). Rien d'autre pour
//! l'instant : pas de besoins, pas de personnalite heritee, pas de LLM (voir
//! `BIBLE/05_COGNITION.md`).
//!
//! Invariant 5 : la memoire subjective ne reecrit jamais l'histoire objective. Un souvenir
//! ancre sur un evenement garde son `event_seq` ; la divergence entre le souvenir et le
//! fait se mesure, elle ne se corrige pas.
//!
//! Invariant 6 : seuls les agents paient le cout cognitif. Le reste de la population ne
//! porte qu'un `Option` a `None` et un petit `last_shock`.

use serde::{Deserialize, Serialize};

use crate::entity::Position;

/// Nature d'un souvenir episodique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    /// Un lieu ou l'agent a failli mourir (sa propre famine). A eviter. Subjectif : pas
    /// d'evenement source (`event_seq` reste `None`).
    Peril,
    /// Un lieu ou l'agent a trouve beaucoup a manger d'un coup. A retrouver.
    Bounty,
    /// Un lieu ou l'agent a vu mourir un des siens (0.0.3, tranche 3). A eviter, comme le
    /// peril. Ancre : `event_seq` pointe l'`EntityDied` correspondant (invariant 5).
    Witnessed,
}

impl MemoryKind {
    /// `true` si ce souvenir repousse l'agent (peril ou mort vue).
    pub fn is_aversive(&self) -> bool {
        matches!(self, MemoryKind::Peril | MemoryKind::Witnessed)
    }
}

/// Un souvenir episodique : un lieu, une valence, une force qui decroit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub formed_tick: u64,
    /// Ou, en coordonnees monde.
    pub place: Position,
    pub kind: MemoryKind,
    /// `seq` de l'evenement objectif a l'origine du souvenir, s'il y en a un. En tranche 1
    /// le peril (sa propre famine) n'a pas d'evenement source : `None`, souvenir purement
    /// subjectif. Les souvenirs ancres (mort d'un proche vue) arrivent en tranche 2.
    #[serde(default)]
    pub event_seq: Option<u64>,
    /// Force dans (0, 1]. Decroit chaque tick, le souvenir s'efface sous `memory_eps`.
    pub strength: f32,
}

/// Jauges internes d'un agent (0.0.3, tranche 4), chacune dans [0, 1]. Montent et descendent
/// selon le vecu, ponderent le comportement : un agent bien nourri explore, un agent effraye
/// evite plus fort ses souvenirs, un agent isole derive vers les siens.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Needs {
    /// Suit vers le haut le manque d'energie, se relache lentement.
    pub hunger: f32,
    /// Monte pres d'un souvenir aversif et apres un choc de peril recent, se relache lentement.
    pub fear: f32,
    /// `1 - support de colonie` : haut = l'agent est loin des siens.
    pub solitude: f32,
}

impl Needs {
    /// Met a jour les jauges. `energy_frac` = energie / seuil de reproduction ;
    /// `near_aversive` dans [0, 1] = proximite ponderee du souvenir aversif le plus proche ;
    /// `shock_peril_recent` = l'agent a frole la famine il y a peu ; `solitude` deja calcule.
    pub fn update(
        &mut self,
        hunger_relief: f32,
        fear_relief: f32,
        energy_frac: f32,
        near_aversive: f32,
        shock_peril_recent: bool,
        solitude: f32,
    ) {
        let raw_hunger = (1.0 - energy_frac).clamp(0.0, 1.0);
        self.hunger = raw_hunger.max(self.hunger * hunger_relief);
        let threat = if shock_peril_recent { 1.0 } else { near_aversive.clamp(0.0, 1.0) };
        self.fear = threat.max(self.fear * fear_relief);
        self.solitude = solitude.clamp(0.0, 1.0);
    }
}

/// Mode de comportement choisi par un agent (0.0.3, tranche 6). A chaque replanification,
/// l'agent evalue chaque option et prend celle de plus grande utilite. Le mode retenu rend
/// la decision lisible : « au tick T elle a choisi de fuir plutot que de manger ».
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorMode {
    /// Aller a la nourriture percue.
    #[default]
    Forage,
    /// S'eloigner du lieu de peril ou de mort vue le plus proche et le plus fort.
    Flee,
    /// Rejoindre le centre de masse des siens.
    Join,
    /// Retourner a un lieu d'abondance memorise.
    SeekBounty,
    /// Petit pas au hasard (rien de mieux a faire).
    Wander,
}

impl BehaviorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BehaviorMode::Forage => "forage",
            BehaviorMode::Flee => "flee",
            BehaviorMode::Join => "join",
            BehaviorMode::SeekBounty => "seek_bounty",
            BehaviorMode::Wander => "wander",
        }
    }
}

/// L'esprit d'un agent. Attache a l'entite quand elle s'eveille, retire si elle retombe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mind {
    /// Tick de l'eveil. Sert au delai de grace et a la retombee.
    pub awoke_tick: u64,
    /// Memoire episodique, bornee a `max_memories` (le plus faible cede la place).
    pub episodic: Vec<Memory>,
    /// Jauges internes (0.0.3, tranche 4).
    #[serde(default)]
    pub needs: Needs,
    /// Dernier mode de comportement choisi (0.0.3, tranche 6).
    #[serde(default)]
    pub mode: BehaviorMode,
}

impl Mind {
    pub fn new(awoke_tick: u64, first: Memory) -> Self {
        Mind {
            awoke_tick,
            episodic: vec![first],
            needs: Needs::default(),
            mode: BehaviorMode::Forage,
        }
    }

    /// Fait decroitre chaque souvenir et retire ceux qui sont passes sous le seuil.
    pub fn decay_and_prune(&mut self, decay: f32, eps: f32) {
        for m in self.episodic.iter_mut() {
            m.strength *= decay;
        }
        self.episodic.retain(|m| m.strength >= eps);
    }

    /// Enregistre un souvenir. Si un souvenir de meme nature existe deja tout pres (moins de
    /// `merge_dist` cases), on le renforce et on le rafraichit au lieu d'en ajouter un. Sinon
    /// on insere ; si la memoire est pleine, le souvenir le plus faible cede la place.
    pub fn record(&mut self, m: Memory, max: usize, merge_dist: f32) {
        let md2 = merge_dist * merge_dist;
        if let Some(existing) = self.episodic.iter_mut().find(|e| {
            e.kind == m.kind && e.place.dist2(&m.place) <= md2
        }) {
            existing.strength = (existing.strength + m.strength).min(1.0);
            existing.formed_tick = m.formed_tick;
            if existing.event_seq.is_none() {
                existing.event_seq = m.event_seq;
            }
            return;
        }
        if self.episodic.len() >= max.max(1) {
            // remplace le plus faible si le nouveau est plus fort, sinon ignore
            if let Some((wi, weakest)) = self
                .episodic
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.strength.partial_cmp(&b.1.strength).unwrap_or(std::cmp::Ordering::Equal))
            {
                if m.strength > weakest.strength {
                    self.episodic[wi] = m;
                }
            }
            return;
        }
        self.episodic.push(m);
    }
}

/// Trace du dernier choc marquant vecu par une entite (agent ou non). Ecrit en phase 5,
/// lu en phase 5c : c'est la graine d'un souvenir, pas encore de la cognition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Shock {
    pub tick: u64,
    pub place: Position,
    /// `true` = famine (peril), `false` = gain de nourriture exceptionnel (aubaine).
    pub peril: bool,
}

impl Shock {
    pub fn kind(&self) -> MemoryKind {
        if self.peril {
            MemoryKind::Peril
        } else {
            MemoryKind::Bounty
        }
    }
}
