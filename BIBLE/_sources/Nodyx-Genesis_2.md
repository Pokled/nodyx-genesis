# Nodyx-Genesis
## Core Data Model Specification

**Document:** `CORE_DATA_MODEL.md`  
**Status:** Draft — Architecture Baseline  
**Version:** 0.1.0  
**Scope:** Genesis Core  
**Audience:** Engine developers, simulation developers, AI/LLM developers, Nodyx integration developers

---

# 1. Objectif

Ce document définit le **modèle de données canonique** de Nodyx-Genesis.

Il constitue le contrat architectural entre :

- le moteur de simulation ;
- la biologie ;
- l'évolution ;
- les agents ;
- la mémoire ;
- les relations sociales ;
- la culture ;
- les civilisations ;
- l'historique ;
- le système événementiel ;
- le LLM ;
- la validation ;
- l'intégration Nodyx ;
- la persistance.

L'objectif principal est d'éviter une situation dans laquelle chaque sous-système développe son propre modèle de vérité.

Genesis doit avoir **une seule représentation canonique du monde**.

---

# 2. Principe fondamental : une seule source de vérité

Genesis distingue trois catégories fondamentales.

```text
                    WORLD STATE
                        │
          ┌─────────────┼─────────────┐
          │             │             │
       REALITY       HISTORY       DERIVED
          │             │             │
          ▼             ▼             ▼
       Entities       Events      Statistics
       Resources      Timeline    Importance
       Geography      Provenance  Aggregates
       State          Causes      Indexes
```

## 2.1. Reality

La réalité actuelle du monde.

Exemples :

- position d'un agent ;
- énergie ;
- population ;
- ressources ;
- relations actuelles ;
- appartenance à un groupe ;
- état politique.

## 2.2. History

Les événements ayant produit cette réalité.

L'historique est append-only.

## 2.3. Derived State

Informations calculées à partir de Reality + History.

Exemples :

- importance d'un agent ;
- statistiques ;
- classement ;
- agrégats démographiques ;
- embeddings ;
- index de recherche.

Un état dérivé peut être supprimé et recalculé.

**Il ne constitue jamais la vérité primaire.**

---

# 3. Invariants absolus

Les règles suivantes sont non négociables.

### INV-001 — Identité

Tout objet persistant possède un identifiant unique.

### INV-002 — Références

Une référence vers une entité inexistante est invalide.

### INV-003 — Temps

Tout événement possède un timestamp logique Genesis.

### INV-004 — Causalité

Un événement ne peut référencer comme cause un événement situé dans le futur.

### INV-005 — Immutabilité historique

Un événement historique validé ne doit jamais être modifié silencieusement.

Une correction produit un nouvel événement.

### INV-006 — Validation

Le LLM ne modifie jamais directement le World State.

### INV-007 — Déterminisme du moteur

À seed + état initial + configuration identiques, la simulation déterministe doit produire le même résultat.

### INV-008 — Séparation subjective/objective

Une croyance ou un souvenir subjectif ne modifie jamais rétroactivement l'Objective History.

### INV-009 — Nodyx externe

Un artefact Nodyx ne devient une partie du monde Genesis qu'après confirmation de son résultat par l'intégration.

### INV-010 — Échec gracieux

Une panne du LLM, du réseau ou de Nodyx ne doit jamais corrompre le World State.

---

# 4. Identifiants

Tous les objets persistants utilisent un identifiant stable.

Format recommandé :

```text
UUIDv7
```

Avantages :

- unicité distribuée ;
- tri temporel approximatif ;
- génération locale ;
- compatibilité avec PostgreSQL ;
- absence de dépendance à un compteur global.

Exemples :

```text
agent_id
event_id
memory_id
civilization_id
artifact_id
conversation_id
```

Les IDs ne doivent jamais être réutilisés.

---

# 5. Temps

Genesis utilise un temps logique interne.

```text
GenesisTime {
    tick: u64,
}
```

Le `tick` constitue l'unité fondamentale de simulation.

Un tick peut représenter une durée abstraite dépendant de la phase du monde.

Exemple :

```text
1 tick = 1 seconde simulée
```

ou, plus tard :

```text
1 tick = 1 minute
1 tick = 1 heure
```

Le moteur ne doit pas dépendre directement du temps réel.

---

# 6. WorldState

`WorldState` représente l'état complet observable du monde à un instant donné.

Conceptuellement :

```rust
struct WorldState {
    world_id: WorldId,
    tick: u64,
    seed: u64,

    geography: Geography,
    environment: Environment,

    entities: EntityStore,
    groups: GroupStore,
    civilizations: CivilizationStore,

    relationships: RelationshipStore,

    culture: CultureStore,
    collective_memory: CollectiveMemoryStore,

    resources: ResourceStore,

    event_log: EventLog,

    scheduler: SchedulerState,
}
```

Le WorldState est la racine logique de Genesis.

---

# 7. Entity

Toutes les formes de vie utilisent une identité d'entité commune.

```rust
struct Entity {
    id: EntityId,
    kind: EntityKind,
    state: EntityState,
}
```

Exemple :

```rust
enum EntityKind {
    Organism,
    Agent,
    Animal,
    Plant,
    Other,
}
```

Un organisme peut donc commencer comme une entité biologique simple puis acquérir progressivement davantage de capacités.

---

# 8. Agent

Un `Agent` représente une entité capable de comportement autonome.

```rust
struct Agent {
    id: AgentId,

    biology: BiologyState,
    position: Position,

    needs: Needs,
    personality: Personality,

    cognition: CognitionState,

    memory: MemoryStore,

    relationships: RelationshipIndex,

    social: SocialState,

    culture: CulturalState,

    possessions: PossessionStore,

    importance: ImportanceState,

    lifecycle: LifecycleState,
}
```

---

# 9. BiologyState

La biologie doit être séparée de la cognition.

```rust
struct BiologyState {
    age: f32,
    health: f32,
    energy: f32,

    reproductive_state: ReproductiveState,

    genome_id: GenomeId,

    species_id: SpeciesId,
}
```

Une entité peut donc être simulée biologiquement sans posséder de LLM.

---

# 10. Genome

Le génome définit les caractéristiques héritables.

```rust
struct Genome {
    id: GenomeId,

    traits: GenomeTraits,

    generation: u64,

    parent_a: Option<AgentId>,
    parent_b: Option<AgentId>,
}
```

Exemple :

```rust
struct GenomeTraits {
    metabolism: f32,
    speed: f32,
    perception: f32,
    fertility: f32,
    longevity: f32,

    curiosity: f32,
    sociability: f32,
    aggression: f32,
}
```

Les traits doivent rester numériques et exploitables par le moteur.

Le LLM ne doit pas être nécessaire pour calculer la génétique.

---

# 11. Needs

Les besoins sont des variables d'état.

```rust
struct Needs {
    hunger: f32,
    thirst: f32,
    fatigue: f32,

    safety: f32,
    social: f32,

    reproduction: f32,
}
```

Les besoins influencent le comportement mais ne constituent pas directement une décision.

---

# 12. Personality

La personnalité est relativement stable mais évolutive.

```rust
struct Personality {
    curiosity: f32,
    sociability: f32,
    aggression: f32,
    empathy: f32,

    risk_tolerance: f32,
    impulsivity: f32,

    conformity: f32,
    openness: f32,

    behavioral_flexibility: f32,
}
```

Toutes les valeurs sont normalisées :

```text
0.0 → 1.0
```

La personnalité peut évoluer à la suite d'expériences significatives.

---

# 13. CognitionState

La cognition représente l'état mental courant.

```rust
struct CognitionState {
    current_goal: Option<GoalId>,
    current_intention: Option<IntentId>,

    emotional_state: EmotionalState,

    attention: AttentionState,

    cognitive_load: f32,
}
```

Cette couche est volontairement distincte de la mémoire.

---

# 14. EmotionalState

```rust
struct EmotionalState {
    valence: f32,
    arousal: f32,

    fear: f32,
    anger: f32,
    joy: f32,
    sadness: f32,
    trust: f32,
}
```

Les émotions sont des variables d'état.

Elles ne doivent pas être générées exclusivement par le LLM.

---

# 15. Memory

La mémoire individuelle est une collection de souvenirs.

```rust
struct Memory {
    id: MemoryId,

    agent_id: AgentId,

    memory_type: MemoryType,

    content: MemoryContent,

    world_event_reference: Option<EventId>,

    confidence: f32,
    emotional_weight: f32,

    divergence_score: f32,

    created_at: GenesisTime,
    last_reinforced_at: GenesisTime,

    importance: f32,
}
```

---

# 16. Types de mémoire

```rust
enum MemoryType {
    Episodic,
    Semantic,
    Procedural,
    Social,
    Autobiographical,
    Cultural,
}
```

Exemples :

```text
Episodic:
"J'ai vu le village brûler."

Semantic:
"Le feu détruit le bois."

Social:
"Aren est dangereux."

Procedural:
"Comment fabriquer une lance."

Cultural:
"Notre peuple célèbre le solstice."
```

---

# 17. Memory Anchoring

Un souvenir peut être subjectif tout en possédant une référence objective.

```text
Memory
   │
   ├── subjective_content
   │
   ├── confidence
   │
   └── world_event_reference
             │
             ▼
          Event #819
```

L'ancrage permet :

- debug ;
- comparaison historique ;
- mesure de divergence ;
- reconstruction de contexte ;
- analyse narrative.

---

# 18. Divergence

```text
divergence_score ∈ [0.0, 1.0]
```

Interprétation indicative :

```text
0.00 → souvenir très proche du fait
0.25 → légère interprétation
0.50 → divergence importante
0.75 → forte reconstruction subjective
1.00 → souvenir pratiquement détaché du fait
```

Ces seuils sont configurables.

---

# 19. Relationship

Les relations sociales sont des objets à part entière.

```rust
struct Relationship {
    id: RelationshipId,

    source: AgentId,
    target: AgentId,

    affinity: f32,
    trust: f32,
    respect: f32,
    fear: f32,

    familiarity: f32,

    relationship_type: RelationshipType,

    history: RelationshipHistory,
}
```

Exemples :

```text
Friend
Parent
Child
Partner
Rival
Enemy
Leader
Follower
Teacher
Student
Stranger
```

Les relations sont directionnelles.

A peut faire confiance à B sans que B fasse confiance à A.

---

# 20. Group

Un groupe représente une organisation sociale.

```rust
struct Group {
    id: GroupId,

    name: String,

    members: Vec<AgentId>,

    leader: Option<AgentId>,

    values: ValueSet,

    norms: NormSet,

    resources: ResourcePool,
}
```

Les groupes peuvent évoluer en :

```text
Family
Band
Tribe
Clan
Guild
Community
PoliticalGroup
ReligiousGroup
```

---

# 21. Civilization

Une civilisation est une structure sociale de niveau supérieur.

```rust
struct Civilization {
    id: CivilizationId,

    name: String,

    territory: Territory,

    population: PopulationState,

    government: GovernmentState,

    economy: EconomyState,

    technology: TechnologyState,

    culture_id: CultureId,

    collective_memory_id: CollectiveMemoryId,

    institutions: InstitutionStore,

    calendar: CalendarState,

    historical_importance: f32,
}
```

---

# 22. Culture

La culture ne doit pas être un simple texte.

Elle doit être un ensemble structuré de connaissances et de pratiques.

```rust
struct Culture {
    id: CultureId,

    values: Vec<Value>,
    norms: Vec<Norm>,
    traditions: Vec<Tradition>,

    beliefs: Vec<CulturalBelief>,

    language: LanguageState,

    memes: MemeStore,

    symbols: SymbolStore,
}
```

---

# 23. Meme

Un mème est une unité transmissible de culture.

```rust
struct Meme {
    id: MemeId,

    origin_event: Option<EventId>,

    content: MemeContent,

    adoption: f32,
    consensus: f32,

    transmission_count: u64,

    mutation_rate: f32,

    cultural_status: CulturalStatus,
}
```

Statuts :

```text
Rumor
Story
Tradition
Legend
Myth
CulturalFact
```

La culture peut donc évoluer sans être directement codée par le développeur.

---

# 24. CollectiveMemory

La mémoire collective appartient à un groupe ou une civilisation.

```rust
struct CollectiveMemory {
    id: CollectiveMemoryId,

    owner_id: CivilizationId,

    memories: Vec<CollectiveMemoryEntry>,
}
```

Une entrée :

```rust
struct CollectiveMemoryEntry {
    id: CollectiveMemoryEntryId,

    meme_id: MemeId,

    consensus: f32,

    confidence: f32,

    divergence_from_objective: f32,

    transmission_rate: f32,

    institutional_support: f32,
}
```

---

# 25. Event

L'événement est le mécanisme central de Genesis.

```rust
struct Event {
    id: EventId,

    tick: u64,

    event_type: EventType,

    actors: Vec<EntityId>,

    location: Option<LocationId>,

    payload: EventPayload,

    causes: Vec<EventId>,

    cascade_depth: u16,

    importance: f32,

    provenance: Provenance,
}
```

---

# 26. Event immutabilité

Un événement validé est immuable.

Incorrect :

```text
Event #500
old_payload → modified_payload
```

Correct :

```text
Event #500
    ↓
CorrectionEvent #721
```

L'historique devient ainsi traçable.

---

# 27. Event Bus

Les événements sont distribués aux systèmes intéressés.

```text
                 Event Bus
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
   Biology       Society      Memory
       │            │            │
       ▼            ▼            ▼
   Evolution      Culture    Civilization
```

Les systèmes doivent être aussi indépendants que possible.

---

# 28. Intent

Une intention représente ce qu'un agent souhaite faire.

```rust
struct Intent {
    id: IntentId,

    agent_id: AgentId,

    action: Action,

    motivation: Motivation,

    confidence: f32,

    generated_by: DecisionSource,
}
```

Sources possibles :

```text
RuleBased
BehaviorTree
LLM
HumanInteraction
ExternalEvent
```

---

# 29. Action

Une action est une opération que Genesis peut tenter d'exécuter.

```rust
enum Action {
    Move,
    Eat,
    Drink,
    Reproduce,

    Talk,
    Trade,
    Fight,

    JoinGroup,
    LeaveGroup,

    CreateArtifact,
    UseNodyxTool,

    Learn,
    Teach,

    CreateInstitution,
}
```

---

# 30. Validation

Toute intention importante passe par les validateurs.

```text
Intent
  │
  ▼
PhysicalValidator
  │
  ▼
SocialValidator
  │
  ▼
NarrativeValidator
  │
  ▼
Execution
```

Résultat :

```rust
struct ValidationResult {
    allowed: bool,

    physical_score: f32,
    social_score: f32,
    narrative_score: f32,

    consequences: Vec<Consequence>,

    modifications: Vec<ActionModification>,

    reason_codes: Vec<String>,
}
```

---

# 31. Physical Validation

Cette couche vérifie les contraintes objectives.

Exemples :

```text
Existe-t-il ?
Est-il vivant ?
Est-il à portée ?
Possède-t-il la ressource ?
L'action est-elle physiquement possible ?
```

Cette validation est obligatoire.

---

# 32. Social Validation

Cette couche calcule les conséquences sociales.

Exemple :

```text
Action:
Trahir un allié

Possible consequences:

reputation_loss = 0.85
group_exclusion = 0.70
revenge_risk = 0.60
resource_loss = 0.50
conflict_risk = 0.30
```

Elle ne décide pas nécessairement si l'action est moralement "bonne".

Elle décrit les conséquences plausibles.

---

# 33. Narrative Validation

La cohérence narrative est un signal.

```text
HIGH
MEDIUM
LOW
```

Elle dépend notamment de :

- personnalité ;
- historique ;
- objectifs ;
- croyances ;
- relations ;
- état émotionnel ;
- stress ;
- flexibilité comportementale.

Une faible cohérence ne signifie pas nécessairement :

```text
REJECT
```

Elle peut signifier :

```text
HIGH COST
LOW PROBABILITY
UNEXPECTED CONSEQUENCE
```

---

# 34. Importance

L'importance est une donnée dérivée.

```rust
struct ImportanceState {
    score: f32,

    social_status: f32,
    influence: f32,
    knowledge: f32,
    uniqueness: f32,

    recent_activity: f32,
    historical_significance: f32,

    player_interest: f32,

    simulation_level: SimulationLevel,
}
```

Niveaux :

```text
Background
Active
Important
Historical
```

---

# 35. Differential Simulation

Le niveau de simulation dépend de l'importance.

```text
Background
    ↓
Statistical simulation

Active
    ↓
Behavioral simulation

Important
    ↓
Detailed simulation

Historical
    ↓
Full cognitive simulation
```

Un agent peut changer de niveau à tout moment.

---

# 36. LLM Context

Le LLM ne reçoit jamais automatiquement toute la vie d'un agent.

Le `ContextBuilder` sélectionne :

```text
Current World State
        +
Relevant Memories
        +
Relevant Relationships
        +
Current Goals
        +
Cultural Context
        +
Recent Events
        +
Necessary History
```

Le contexte est limité par un budget.

---

# 37. ContextItem

```rust
struct ContextItem {
    source_id: String,

    relevance: f32,

    recency: f32,

    importance: f32,

    emotional_weight: f32,

    token_cost: u32,
}
```

Le Context Builder doit optimiser :

```text
semantic value / token cost
```

---

# 38. Structured Output

Les sorties LLM doivent être validées par schéma.

Exemple conceptuel :

```json
{
  "action": "talk",
  "dialogue": [],
  "relationship_delta": 0.12,
  "new_beliefs": [],
  "new_memories": [],
  "intentions": [],
  "emotional_change": {}
}
```

Le texte conversationnel est secondaire.

Les métadonnées structurées constituent la sortie exploitable par Genesis.

---

# 39. Fallback LLM

Hiérarchie :

```text
LLM principal
    ↓ failure
LLM léger
    ↓ failure
Behavior Tree
    ↓ failure
Rule-based behavior
    ↓ failure
Default action
```

Le moteur doit toujours pouvoir continuer.

---

# 40. Artifact

Tout contenu culturel persistant possède une identité Genesis.

```rust
struct Artifact {
    id: ArtifactId,

    creator: EntityId,

    civilization_id: Option<CivilizationId>,

    artifact_type: ArtifactType,

    created_at: GenesisTime,

    origin_event: EventId,

    nodyx_reference: Option<NodyxArtifactReference>,

    cultural_significance: f32,
}
```

---

# 41. NodyxArtifactReference

Genesis ne possède pas nécessairement le contenu technique de Nodyx.

Il conserve une référence.

```rust
struct NodyxArtifactReference {
    platform: String,

    external_id: String,

    artifact_type: String,

    created_at: Timestamp,

    integrity_hash: Option<String>,
}
```

Ainsi, Genesis peut rester indépendant de l'implémentation interne de Nodyx.

---

# 42. Nodyx Tool Call

```rust
struct NodyxToolCall {
    id: ToolCallId,

    agent_id: AgentId,

    tool: NodyxTool,

    parameters: ToolParameters,

    requested_at: GenesisTime,

    validation: ValidationResult,

    execution_status: ExecutionStatus,
}
```

Exemples :

```text
CreateWikiPage
CreateCanvas
CreateCalendarEvent
CreateForumPost
SendMessage
CreateGame
Vote
```

---

# 43. Transaction Boundary

Une action externe doit suivre :

```text
Intent
  ↓
Validation
  ↓
Event Creation
  ↓
Nodyx Request
  ↓
External Result
  ↓
Genesis Confirmation
  ↓
World State Update
```

Une opération Nodyx ne doit jamais modifier directement les structures internes.

---

# 44. Provenance

Chaque donnée importante doit pouvoir répondre à :

> "Pourquoi cette donnée existe-t-elle ?"

```rust
struct Provenance {
    source_event: Option<EventId>,

    source_agent: Option<AgentId>,

    source_system: ProvenanceSource,

    created_at: GenesisTime,
}
```

Sources :

```text
Simulation
Agent
LLM
Human
Nodyx
Migration
System
```

---

# 45. Persistence

Le World State doit être persistant.

Architecture recommandée :

```text
             World State
                 │
        ┌────────┴────────┐
        ▼                 ▼
     Snapshot          Event Log
        │                 │
        └────────┬────────┘
                 ▼
             Recovery
```

Le snapshot accélère le chargement.

L'Event Log permet la reconstruction et l'audit.

---

# 46. Snapshot

Un snapshot contient :

```text
WorldState
tick
configuration_version
simulation_version
seed
schema_version
```

Il doit être versionné.

---

# 47. Event Sourcing partiel

Genesis ne doit pas nécessairement être un Event Sourcing pur.

Approche recommandée :

```text
Mutable World State
+
Immutable Event Log
+
Periodic Snapshots
```

Cela permet :

- performance ;
- replay ;
- debugging ;
- rollback ;
- analyse historique.

---

# 48. Configuration

Les règles du monde ne doivent pas être hardcodées partout.

```rust
struct SimulationConfig {
    biology: BiologyConfig,
    evolution: EvolutionConfig,

    social: SocialConfig,
    culture: CultureConfig,

    llm: LlmConfig,
    validation: ValidationConfig,

    scheduler: SchedulerConfig,
}
```

Deux mondes peuvent ainsi posséder des paramètres différents.

---

# 49. Seed

Toute simulation possède une seed.

```text
world.seed
```

Elle permet de reproduire les phénomènes pseudo-aléatoires.

Important :

Les appels LLM externes ne sont pas intrinsèquement déterministes.

Ils doivent donc être enregistrés lorsqu'ils influencent le monde.

---

# 50. LLM Replay

Lorsqu'une sortie LLM provoque une modification significative :

```text
LLM Request
+
Model
+
Model Version
+
Parameters
+
Prompt Hash
+
Context Hash
+
Structured Output
```

doivent pouvoir être associés à l'événement produit.

Cela permet de comprendre :

> pourquoi l'agent a pris cette décision ?

---

# 51. Séparation des responsabilités

Genesis doit respecter cette architecture :

```text
BIOLOGY
  → corps

BEHAVIOR
  → décisions bas niveau

COGNITION
  → objectifs / intentions

LLM
  → raisonnement complexe / langage

VALIDATION
  → plausibilité

EVENT SYSTEM
  → causalité

WORLD STATE
  → vérité actuelle

HISTORY
  → vérité passée

CULTURE
  → connaissances collectives

NODYX
  → environnement numérique externe
```

Aucun système ne doit absorber toutes les responsabilités.

---

# 52. Dépendances autorisées

Règle générale :

```text
World
  ↓
Biology
  ↓
Behavior
  ↓
Cognition
  ↓
LLM
```

Mais :

```text
LLM ─X→ WorldState
```

Le LLM ne possède jamais d'autorité directe.

De même :

```text
Nodyx ─X→ WorldState
```

Nodyx retourne un résultat.

Genesis décide de son intégration.

---

# 53. Architecture globale

```text
                           NODYX
                             ▲
                             │
                       Integration Layer
                             ▲
                             │
                        Event / API
                             ▲
                             │
┌────────────────────────────────────────────────────┐
│                  GENESIS ENGINE                    │
│                                                    │
│  ┌──────────┐     ┌──────────────┐                │
│  │ Biology  │────►│ World State  │                │
│  └──────────┘     └──────┬───────┘                │
│                           │                        │
│  ┌──────────┐             ▼                        │
│  │Behavior  │──────► Decision Engine              │
│  └──────────┘             │                        │
│                           ▼                        │
│                      Validation                    │
│                           │                        │
│                           ▼                        │
│                       Event Bus                    │
│                           │                        │
│          ┌────────────────┼───────────────┐        │
│          ▼                ▼               ▼        │
│       Memory           Culture        Civilization │
│          │                │               │        │
│          └────────────────┼───────────────┘        │
│                           ▼                        │
│                     History / Log                  │
│                                                    │
└────────────────────────────────────────────────────┘
                             │
                             ▼
                           LLM
```

---

# 54. Règle d'or architecturale

Genesis doit pouvoir fonctionner dans cet état :

```text
LLM = OFF
NODYX = OFF
NETWORK = OFF
```

avec :

```text
Biology = ON
Evolution = ON
World = ON
Events = ON
Persistence = ON
```

Si ce n'est pas possible, l'architecture est trop dépendante des services externes.

---

# 55. Genesis 0.0.1 — Modèle minimal

Le premier prototype n'implémente qu'une fraction de ce modèle.

Minimum :

```text
WorldState
├── Cell
├── Entity
│   ├── BiologyState
│   ├── Genome
│   └── Position
│
├── Resource
│
└── EventLog
```

Aucune :

```text
LLM
Memory
Culture
Civilization
Nodyx
```

---

# 56. Évolution du modèle

### 0.0.1

```text
World
Biology
Evolution
Events
Persistence
```

### 0.0.2

```text
+
Needs
Personality
Behavior
```

### 0.0.3

```text
+
Memory
Relationships
```

### 0.0.4

```text
+
Communication
Social Groups
```

### 0.0.5

```text
+
Culture
Collective Memory
LLM
```

### 0.0.6+

```text
+
Civilization
Institutions
Nodyx
Artifacts
Human Interaction
```

---

# 57. Tests architecturaux

Chaque invariant important doit avoir un test.

Exemples :

```text
test_event_ids_are_unique
test_event_history_is_immutable
test_invalid_move_is_rejected
test_dead_agent_cannot_reproduce
test_llm_cannot_modify_world_directly
test_nodyx_failure_does_not_corrupt_world
test_memory_anchor_points_to_valid_event
test_future_event_cannot_be_used_as_cause
test_snapshot_can_restore_world
test_same_seed_produces_same_deterministic_state
```

---

# 58. Critère de maturité

Le modèle de données sera considéré comme suffisamment solide lorsque :

1. un monde peut être sauvegardé ;
2. un snapshot peut être restauré ;
3. les événements peuvent être rejoués ;
4. les agents peuvent évoluer sans LLM ;
5. les systèmes peuvent évoluer indépendamment ;
6. les références restent cohérentes ;
7. les décisions LLM sont traçables ;
8. les erreurs externes n'endommagent pas le monde ;
9. une histoire objective peut être reconstruite ;
10. une histoire subjective peut diverger sans modifier l'histoire objective.

---

# 59. Décision architecturale finale

Le principe directeur de Nodyx-Genesis est :

> **Le World State représente ce qui existe.**
>
> **L'Event Log représente ce qui s'est produit.**
>
> **La mémoire représente ce qu'un individu croit avoir vécu.**
>
> **La mémoire collective représente ce qu'une société croit avoir vécu.**
>
> **La culture représente ce qu'une société transmet.**
>
> **Le LLM représente une capacité cognitive, pas une autorité.**
>
> **Nodyx représente l'environnement numérique avec lequel le monde peut interagir.**

Cette séparation constitue le socle de Genesis.

Toute évolution future de l'architecture doit préserver ces frontières.