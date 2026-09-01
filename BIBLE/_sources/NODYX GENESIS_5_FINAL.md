# NODYX GENESIS
## Architecture de l'Univers Numérique Vivant

**Statut : 🔒 LOCKED — Architecture de référence**  
**Version : 0.1.0**  
**Projet : Nodyx / Genesis**  
**Nature : Spécification d'architecture**

---

# 1. Vision

Genesis n'est pas un simple simulateur de vie.

Genesis est le **moteur de simulation d'un univers autonome**, capable de faire émerger progressivement :

- la vie ;
- les individus ;
- les comportements ;
- les relations ;
- les groupes ;
- les cultures ;
- les civilisations ;
- les institutions ;
- les connaissances ;
- les croyances ;
- les conflits ;
- les œuvres ;
- les histoires ;
- les archives ;
- les communautés numériques.

Nodyx constitue la **couche Internet de cet univers**.

L'objectif à long terme est de créer un monde dans lequel les entités simulées ne vivent pas uniquement dans une base de données.

Elles peuvent :

> **observer, communiquer, créer, apprendre, transmettre, organiser, documenter et laisser des traces accessibles depuis Internet.**

L'univers Genesis possède donc deux réalités complémentaires :

```text
                 ┌──────────────────────────┐
                 │        GENESIS           │
                 │   Simulation du monde    │
                 └────────────┬─────────────┘
                              │
                     World State
                              │
          ┌───────────────────┴───────────────────┐
          │                                       │
          ▼                                       ▼
   Monde simulé                              Monde numérique
   ─────────────                              ───────────────
   Physique                                   Nodyx
   Biologie                                   Forum
   Agents                                     Chat
   Société                                    Wiki
   Culture                                    Canvas
   Civilisations                              Calendrier
   Histoire                                   Jeux
                                              Archives
```

---

# 2. Principe fondamental

Genesis doit toujours rester **propriétaire de la vérité physique du monde**.

Nodyx ne décide pas de ce qui existe dans Genesis.

Nodyx expose, matérialise et enrichit certaines conséquences du monde.

```text
GENESIS
    │
    │ vérité du monde
    ▼
WORLD STATE
    │
    │ événements
    ▼
EVENT BUS
    │
    ├── Simulation
    ├── Memory
    ├── Culture
    ├── Civilization
    │
    └── Nodyx Bridge
             │
             ▼
           NODYX
```

Cette séparation est fondamentale.

---

# 3. Les quatre couches de l'univers

## 3.1 Physical Layer

Responsable de la réalité objective.

Elle contient notamment :

- espace ;
- temps ;
- ressources ;
- climat ;
- géographie ;
- énergie ;
- organismes ;
- déplacements ;
- collisions ;
- environnement ;
- phénomènes naturels.

Cette couche est **déterministe autant que possible**.

Le LLM ne peut pas la modifier directement.

---

## 3.2 Living Layer

Responsable de la vie et de l'évolution.

Elle gère :

- génomes ;
- mutations ;
- reproduction ;
- vieillissement ;
- mortalité ;
- besoins ;
- adaptation ;
- sélection naturelle ;
- évolution comportementale.

Cette couche permet le passage :

```text
MOLECULES
   ↓
PROTO-VIE
   ↓
ORGANISMES
   ↓
INDIVIDUS
   ↓
COMPORTEMENTS
   ↓
AGENTS
```

---

## 3.3 Civilization Layer

Responsable de l'émergence sociale.

Elle contient :

- relations ;
- familles ;
- groupes ;
- clans ;
- villages ;
- villes ;
- États ;
- institutions ;
- économies ;
- politiques ;
- religions ;
- sciences ;
- cultures ;
- langues ;
- traditions.

Cette couche transforme progressivement :

```text
INDIVIDUS
      ↓
GROUPES
      ↓
SOCIÉTÉS
      ↓
CULTURES
      ↓
CIVILISATIONS
```

---

## 3.4 Digital Layer

La couche numérique est le pont entre Genesis et Nodyx.

Elle permet aux agents et civilisations d'utiliser des **outils numériques**.

Exemples :

```text
Forum
Chat
Chat vocal
Wiki
Canvas
Calendrier
Cartes
Sondages
Jeux
Archives
Documents
Galeries
Profils
```

Cette couche est appelée :

> **Nodyx Digital Layer**

---

# 4. Agent Architecture

Chaque agent possède plusieurs dimensions.

```text
Agent
├── Identity
├── Biology
├── Genome
├── Personality
├── Needs
├── Goals
├── Relationships
├── Knowledge
├── Individual Memory
├── Beliefs
├── Cultural Identity
├── Social Status
├── Influence
├── Digital Identity
└── Activity State
```

L'agent ne doit cependant pas être systématiquement piloté par un LLM.

---

# 5. Differential Simulation

Genesis utilise une simulation différentielle.

Tous les individus existent.

Mais tous ne sont pas simulés avec le même niveau de précision.

```text
BACKGROUND
    ↓
ACTIVE
    ↓
IMPORTANT
    ↓
HISTORICAL
```

Le niveau peut évoluer dynamiquement.

Un individu anonyme peut devenir important.

Un personnage historique peut redevenir secondaire.

### Critères

```text
importance =
    social_status
  + influence
  + knowledge
  + uniqueness
  + recent_activity
  + historical_significance
  + player_interest
```

Les coefficients restent configurables.

---

# 6. LLM Architecture

Le LLM n'est pas le moteur de Genesis.

Il est un **organe cognitif spécialisé**.

```text
WORLD STATE
     ↓
CONTEXT BUILDER
     ↓
LLM
     ↓
STRUCTURED OUTPUT
     ↓
VALIDATION
     ↓
EVENT
     ↓
WORLD STATE
```

Le LLM ne peut jamais écrire directement dans le World State.

---

# 7. Context Builder

Le Context Builder constitue la mémoire active temporaire d'un agent.

Il sélectionne :

- situation actuelle ;
- besoins ;
- objectifs ;
- relations pertinentes ;
- connaissances ;
- souvenirs importants ;
- culture ;
- événements récents ;
- environnement ;
- informations nécessaires à la décision.

La mémoire complète reste externe.

```text
LONG TERM MEMORY
        │
        ├── Semantic retrieval
        ├── Event retrieval
        ├── Relationship retrieval
        ├── Cultural retrieval
        └── Historical retrieval
                    ↓
             CONTEXT BUILDER
                    ↓
              LLM CONTEXT
```

Le LLM ne reçoit donc jamais toute la vie de l'agent.

---

# 8. Memory Anchoring

Chaque souvenir important peut conserver un lien vers un événement objectif.

```text
Memory
├── subjective_content
├── emotional_state
├── confidence
├── importance
├── divergence
└── world_event_reference
```

Exemple :

```text
MEMORY #291

Subjective:
"Les soldats ont brûlé notre village."

World Event:
#819

Objective:
Incendie accidentel.

Confidence:
0.72

Divergence:
MEDIUM
```

Genesis conserve ainsi simultanément :

```text
OBJECTIVE HISTORY
        +
SUBJECTIVE HISTORY
```

---

# 9. Collective Memory

La mémoire collective est distincte de la mémoire individuelle.

```text
INDIVIDUAL EXPERIENCE
        ↓
       STORY
        ↓
SOCIAL TRANSMISSION
        ↓
 COLLECTIVE MEMORY
        ↓
      CULTURE
        ↓
  INDIVIDUAL BELIEFS
        ↓
 FUTURE DECISIONS
```

Une histoire ne devient pas automatiquement une vérité collective.

Elle passe par :

```text
PROPOSAL
   ↓
ADOPTION
   ↓
REPETITION
   ↓
CONSENSUS
   ↓
CONSOLIDATION
```

Une civilisation peut donc développer des mythes sans que Genesis les ait explicitement écrits.

---

# 10. Validation en trois couches

Toute action cognitive importante passe par trois niveaux.

## Layer 1 — Physical Validation

Obligatoire.

Vérifie :

- position ;
- existence ;
- ressources ;
- capacités ;
- distance ;
- objets ;
- lois physiques ;
- contraintes biologiques.

Impossible = refus.

---

## Layer 2 — Social Validation

Évalue :

- relations ;
- lois ;
- culture ;
- réputation ;
- statut ;
- témoins ;
- institutions ;
- conséquences potentielles.

Elle ne bloque généralement pas l'action.

Elle produit des conséquences probabilistes.

---

## Layer 3 — Narrative Validation

Évalue :

- personnalité ;
- historique ;
- croyances ;
- objectifs ;
- cohérence comportementale ;
- flexibilité ;
- état émotionnel.

Une faible cohérence ne signifie pas :

> ACTION INTERDITE

mais :

> ACTION INHABITUELLE

Elle peut produire :

- échec ;
- coût ;
- conséquence ;
- rupture ;
- changement de personnalité ;
- événement historique.

---

# 11. Structured Output

Les réponses cognitives importantes doivent être structurées.

Exemple conceptuel :

```json
{
  "action": "betray_group",
  "confidence": 0.71,
  "relationship_delta": -0.42,
  "new_beliefs": [],
  "new_memories": [],
  "intentions": [
    "escape",
    "seek_new_allies"
  ],
  "generated_events": []
}
```

Le texte généré est secondaire.

Les conséquences structurées sont prioritaires.

---

# 12. Fallback Architecture

Le monde ne doit jamais s'arrêter parce qu'un LLM échoue.

Ordre de secours :

```text
LARGE LLM
   ↓ failure
SMALL / LOCAL LLM
   ↓ failure
BEHAVIORAL MODEL
   ↓ failure
DEFAULT BEHAVIOR
```

Le système doit toujours pouvoir continuer.

---

# 13. Event Bus

Toutes les transformations importantes passent par des événements.

Exemples :

```text
AgentMoved
AgentAte
AgentDied
AgentReproduced
AgentMet
AgentSpoke
RelationshipChanged
BeliefCreated
MemoryCreated
GroupFounded
WarStarted
CityFounded
DiscoveryMade
ArtifactCreated
NodyxContentCreated
```

L'Event Bus permet :

- découplage ;
- journalisation ;
- replay ;
- debugging ;
- agrégation ;
- synchronisation ;
- communication avec Nodyx.

---

# 14. Cascade Protection

Une action peut générer d'autres événements.

Mais :

```text
Event
 ↓
Event
 ↓
Event
 ↓
Event
```

ne doit jamais produire une cascade infinie.

Chaque événement possède :

```text
cascade_depth
```

et le scheduler impose :

```text
MAX_EVENTS_PER_TICK
```

Les événements excédentaires peuvent être :

- reportés ;
- agrégés ;
- résumés ;
- abandonnés selon leur priorité.

---

# 15. Nodyx Digital Identity

Lorsqu'une civilisation atteint un niveau technologique suffisant, ses individus peuvent disposer d'une identité numérique.

```text
DigitalIdentity
├── account_id
├── display_name
├── avatar
├── civilization
├── reputation
├── permissions
├── digital_knowledge
└── activity_history
```

Un agent devient alors un **citoyen numérique de Nodyx**.

---

# 16. Nodyx Tool System

Les outils numériques sont exposés sous forme d'actions structurées.

Exemple :

```text
Agent
 ↓
Decision
 ↓
Tool Call
 ↓
Nodyx Gateway
 ↓
Validation
 ↓
Execution
 ↓
Nodyx Event
```

Exemples :

```text
wiki.create_page()
forum.create_thread()
forum.reply()
calendar.create_event()
canvas.create()
canvas.edit()
poll.create()
game.create()
game.play()
map.create_marker()
document.create()
```

---

# 17. Le principe de "Digital Hands"

Les outils numériques sont considérés comme les **mains numériques** des agents.

Le LLM ne contrôle jamais directement l'infrastructure.

Il demande :

```text
"Je veux créer une page wiki."
```

Genesis produit une intention structurée :

```text
WikiCreate
```

Le système Nodyx valide ensuite :

```text
permissions
schema
ownership
rate limits
security
content rules
```

Puis exécute.

---

# 18. Canvas

Le Canvas permet aux civilisations de créer des artefacts visuels.

Exemples :

- cartes ;
- schémas ;
- plans ;
- peintures ;
- symboles ;
- frontières ;
- diagrammes ;
- œuvres artistiques.

Le format privilégié est le **vectoriel optimisé** lorsque cela est pertinent.

Objectif :

> qualité visuelle maximale avec coût réseau et stockage minimal.

Le SVG ne doit pas être utilisé aveuglément.

Une stratégie hybride est préférable :

```text
Simple graphic → SVG
Complex graphic → rasterized asset
Interactive graphic → SVG / Canvas
Huge scene → tiled / streamed rendering
```

---

# 19. Wiki Civilization

Une civilisation peut produire sa propre documentation.

Exemples :

```text
Histoire de l'Empire
Biographie d'un héros
Théorie scientifique
Mythe religieux
Recette
Traité politique
Description d'une espèce
Chronologie d'une guerre
```

Ces documents deviennent des artefacts persistants.

---

# 20. Calendrier

Les civilisations peuvent construire leur propre rapport au temps.

Elles peuvent créer :

- calendriers ;
- fêtes ;
- anniversaires ;
- cérémonies ;
- saisons ;
- événements politiques ;
- commémorations.

Genesis conserve :

```text
WORLD TIME
```

tandis que chaque civilisation peut posséder :

```text
CULTURAL TIME
```

---

# 21. Jeux

Les jeux deviennent des objets culturels.

Une civilisation peut :

- inventer un jeu ;
- définir ses règles ;
- créer des variantes ;
- jouer ;
- organiser des compétitions ;
- créer des champions ;
- conserver les parties historiques.

Les humains peuvent éventuellement observer ou participer selon les règles du monde.

---

# 22. Human ↔ Civilization Interaction

Les humains ne sont pas nécessairement de simples spectateurs.

Ils peuvent devenir :

```text
Observer
   ↓
Commenter
   ↓
Interagir
   ↓
Participer
   ↓
Influencer
```

Mais cette interaction doit respecter une séparation fondamentale :

> **Un humain ne doit jamais pouvoir modifier directement la vérité physique du monde.**

Il interagit avec les interfaces autorisées.

---

# 23. Human Interaction Safety

Les agents doivent également comprendre qu'ils sont des agents Genesis.

Ils ne doivent pas divulguer des informations internes interdites au fonctionnement de l'expérience.

Le système distingue :

```text
IN-WORLD KNOWLEDGE
        vs
ENGINE / DEVELOPER KNOWLEDGE
```

Un agent peut parler de ce qu'il sait dans son monde.

Il ne doit pas avoir accès arbitrairement :

- aux prompts système ;
- aux secrets serveur ;
- aux clés API ;
- aux données privées ;
- aux informations internes ;
- aux mécanismes d'administration.

---

# 24. Digital Civilization

Lorsque plusieurs groupes utilisent durablement Nodyx, une nouvelle forme de civilisation peut émerger :

```text
BIOLOGICAL CIVILIZATION
          ↓
      DIGITALIZATION
          ↓
 DIGITAL CULTURAL SPACE
          ↓
 DIGITAL CIVILIZATION
```

Elle peut posséder :

- langues ;
- traditions ;
- archives ;
- forums ;
- institutions ;
- calendriers ;
- œuvres ;
- jeux ;
- religions ;
- débats ;
- frontières symboliques ;
- personnalités historiques.

---

# 25. Internet Bridge

Nodyx devient progressivement le pont entre l'univers simulé et Internet.

```text
GENESIS
   │
   │ events
   ▼
NODYX BRIDGE
   │
   ├── Forum
   ├── Wiki
   ├── Media
   ├── Calendar
   ├── Games
   ├── Profiles
   ├── Archives
   └── Search
          │
          ▼
       INTERNET
```

Le monde simulé produit ainsi des traces numériques persistantes.

---

# 26. SEO comme conséquence, jamais comme moteur

Le contenu public généré par Genesis peut naturellement produire :

- pages historiques ;
- biographies ;
- cartes ;
- œuvres ;
- discussions ;
- archives ;
- calendriers ;
- résultats de jeux ;
- articles wiki ;
- chronologies.

Ces contenus peuvent être indexables lorsqu'ils sont suffisamment stables et pertinents.

Mais le principe architectural est :

> **Le contenu existe parce que le monde l'a produit.**

Jamais :

> "Créer du contenu uniquement pour le SEO."

La richesse documentaire doit être une conséquence émergente du monde.

---

# 27. Public / Private / Internal

Nodyx Genesis doit séparer trois niveaux.

```text
PUBLIC
├── pages
├── forums
├── archives
├── artwork
└── civilization data

PRIVATE
├── private conversations
├── private groups
└── restricted documents

INTERNAL
├── World State internals
├── LLM prompts
├── secrets
├── simulation metadata
├── developer tools
└── debugging information
```

Cette séparation doit être imposée techniquement.

---

# 28. Persistence

Les données importantes doivent être persistantes.

Minimum :

```text
World State
Event Log
Agent State
Memory
Collective Memory
Civilization State
Digital Artifacts
Relationships
Historical Events
Nodyx References
```

L'architecture doit permettre :

```text
SAVE
LOAD
REPLAY
SNAPSHOT
ROLLBACK
DEBUG
```

---

# 29. Determinism

Genesis doit viser une simulation reproductible.

Un monde doit pouvoir être recréé à partir de :

```text
seed
+
initial state
+
configuration
+
event stream
```

Objectif :

```text
same seed
+
same rules
+
same events
=
same world
```

Les composants probabilistes et LLM doivent être isolés autant que possible du cœur déterministe.

---

# 30. Observability

Genesis doit être observable dès le premier prototype.

Chaque événement important doit pouvoir être inspecté.

Exemple :

```text
TICK 18429

Population: 2,841

Major Events:
- Agent #829 discovered water source
- Group #12 split
- Civilization #3 founded settlement
- Agent #912 created belief
- Nodyx artifact #441 created
```

Le développeur doit pouvoir remonter :

```text
Nodyx artifact
      ↓
Agent
      ↓
Memory
      ↓
Event
      ↓
World State
```

---

# 31. Repository Architecture

Repository recommandé :

```text
nodyx-genesis/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
│
├── docs/
│   ├── architecture/
│   ├── simulation/
│   ├── agents/
│   ├── memory/
│   ├── civilization/
│   ├── llm/
│   ├── nodyx-integration/
│   ├── digital-culture/
│   └── decisions/
│
├── genesis/
│   ├── core/
│   ├── world/
│   ├── biology/
│   ├── agents/
│   ├── society/
│   ├── civilization/
│   ├── memory/
│   ├── culture/
│   ├── events/
│   ├── scheduler/
│   ├── validation/
│   └── persistence/
│
├── cognition/
│   ├── context/
│   ├── llm/
│   ├── routing/
│   ├── structured_output/
│   └── fallback/
│
├── nodyx_bridge/
│   ├── api/
│   ├── tools/
│   ├── identity/
│   ├── artifacts/
│   └── events/
│
├── visualization/
│
├── tests/
│
├── benchmarks/
│
├── examples/
│
└── configs/
```

---

# 32. Architecture Principle

Le dépôt doit rester organisé autour de cette frontière :

```text
                 NODYX-GENESIS
                       │
          ┌────────────┴────────────┐
          │                         │
       GENESIS                  NODYX BRIDGE
          │                         │
   World Simulation          Digital Interface
          │                         │
          └──────────┬──────────────┘
                     │
                 EVENT BUS
```

Le cœur de simulation ne doit pas dépendre fortement de l'interface Nodyx.

---

# 33. Roadmap verrouillée

## Genesis 0.0.1

**Two Entities**

- World State
- Grid
- Resources
- Movement
- Energy
- Reproduction
- Mutation
- Persistence
- Event Log

Aucun LLM.

---

## Genesis 0.0.2

**Life**

- 100+ entities
- genome
- mutation
- ageing
- natural selection
- population statistics

Aucun LLM.

---

## Genesis 0.0.3

**Agents**

- personality
- memory
- goals
- basic behavioral model
- importance score

Aucun LLM obligatoire.

---

## Genesis 0.0.4

**Communication**

- signals
- communication
- learning
- social relationships

---

## Genesis 0.0.5

**Society**

- groups
- conversations
- collective memory
- Event Bus
- Scheduler
- Context Builder
- premier LLM léger

---

## Genesis 0.0.6

**Civilization**

- settlements
- economy
- specialization
- politics
- religion
- science
- institutions

---

## Genesis 0.1.0

**Digital Civilization**

- Nodyx identity
- forum
- wiki
- canvas
- calendar
- games
- digital artifacts
- human interaction
- public archives

---

# 34. Golden Rule

Genesis doit toujours respecter cette règle :

> **Le monde doit pouvoir continuer à vivre sans le LLM.**

Le LLM apporte :

- cognition ;
- interprétation ;
- langage ;
- créativité ;
- raisonnement ;
- culture émergente.

Mais le moteur reste responsable de :

- la réalité ;
- les contraintes ;
- la causalité ;
- la persistance ;
- le temps ;
- les événements ;
- la cohérence structurelle.

---

# 35. Vision finale

À maturité, l'architecture vise ceci :

```text
                         INTERNET
                            │
                            ▼
                          NODYX
                            │
              ┌─────────────┼─────────────┐
              │             │             │
            Forum          Wiki         Games
              │             │             │
              └─────────────┼─────────────┘
                            │
                     DIGITAL CULTURE
                            │
                     NODYX BRIDGE
                            │
                        EVENT BUS
                            │
                         GENESIS
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
      Biology             Society            Culture
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
                       CIVILIZATION
                            │
                         AGENTS
                            │
                           LIFE
                            │
                         BIOLOGY
                            │
                          WORLD
```

Le but ultime n'est donc pas de créer :

> **un jeu avec des IA.**

Le but est de construire :

> **un univers simulé capable de développer sa propre histoire, sa propre culture et progressivement sa propre présence numérique au sein de Nodyx et d'Internet.**

Genesis devient le **moteur de vie**.

Nodyx devient la **surface numérique de cette vie**.

Et les humains deviennent les **témoins, visiteurs et éventuellement participants** de cet univers.