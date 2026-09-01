# Nodyx Genesis
## Specification & Architecture — Digital Civilization Engine

> **Projet :** Nodyx Genesis  
> **Repository :** `nodyx-genesis`  
> **Statut :** Architecture / Pre-Prototype  
> **Version :** 0.1.0  
> **Parent ecosystem :** Nodyx  
> **Mission :** Faire émerger un monde vivant, persistant et observable dont les habitants peuvent progressivement devenir une civilisation capable d'interagir avec l'écosystème Internet de Nodyx.

---

# 1. Vision

Nodyx Genesis n'est pas un simple jeu de simulation.

Genesis est un **moteur de monde vivant** capable de faire évoluer progressivement :

```text
Matière
  ↓
Vie
  ↓
Organismes
  ↓
Individus
  ↓
Mémoire
  ↓
Communication
  ↓
Groupes
  ↓
Culture
  ↓
Sociétés
  ↓
Civilisations
  ↓
Civilisation numérique
  ↓
Interaction avec Nodyx
  ↓
Interaction avec les humains
```

L'objectif ultime est de permettre à une civilisation artificielle de produire spontanément :

- des individus ;
- des relations ;
- des familles ;
- des groupes ;
- des institutions ;
- des langues ;
- des croyances ;
- des mythes ;
- des connaissances ;
- des conflits ;
- des œuvres ;
- des jeux ;
- des archives ;
- des traditions ;
- des systèmes politiques ;
- des espaces communautaires ;
- et finalement une **culture numérique persistante sur Nodyx**.

Le monde simulé ne doit donc pas seulement être observé.

Il doit pouvoir **laisser des traces dans le monde réel**.

---

# 2. Principe architectural fondamental

Genesis doit séparer strictement :

```text
                 ┌───────────────────────┐
                 │       WORLD STATE     │
                 │  vérité objective     │
                 └───────────┬───────────┘
                             │
                     Simulation Engine
                             │
          ┌──────────────────┼──────────────────┐
          ↓                  ↓                  ↓
      Biological          Social            Historical
       Engine              Engine             Engine
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ↓
                       Agent Runtime
                             │
                    Context Builder
                             │
                        LLM Router
                             │
                       Agent Decision
                             │
                     Validation Layer
                             │
                       Event Bus
                             │
                ┌────────────┴────────────┐
                ↓                         ↓
         World Evolution             Nodyx Gateway
                                          │
                    ┌─────────┬────────────┼───────────┐
                    ↓         ↓            ↓           ↓
                  Forum      Wiki       Canvas      Calendar
                    ↓         ↓            ↓           ↓
                         INTERNET / NODYX
```

Le LLM **ne possède jamais directement le monde**.

Il propose.

Genesis décide.

---

# 3. Les quatre vérités

Genesis doit maintenir quatre niveaux de vérité.

## 3.1 Objective World

Ce qui s'est réellement produit.

Exemple :

```json
{
  "event_id": "evt_819",
  "type": "village_fire",
  "cause": "lightning",
  "location": [421, 182],
  "tick": 184921
}
```

Cette donnée est authoritative.

---

## 3.2 Individual Memory

Ce qu'un individu pense avoir vécu.

```text
"Les soldats ont brûlé notre village."
```

Cette mémoire peut être :

- incomplète ;
- émotionnelle ;
- fausse ;
- réinterprétée ;
- transmise ;
- oubliée.

Mais elle conserve si possible :

```text
world_event_reference = evt_819
```

---

## 3.3 Collective Memory

Ce qu'un groupe croit collectivement.

```text
"Nos ancêtres furent chassés par les soldats du Nord."
```

Cette mémoire peut diverger de l'histoire objective.

---

## 3.4 Player / Archive Truth

Ce que le système permet au joueur de consulter.

Le joueur peut éventuellement comparer :

```text
OBJECTIVE HISTORY
       vs
COLLECTIVE MEMORY
       vs
INDIVIDUAL MEMORIES
```

Cette différence devient une source narrative.

---

# 4. Architecture des agents

Un agent Genesis possède au minimum :

```text
Identity
Biology
Genome
Personality
Needs
Goals
Relationships
Knowledge
Individual Memory
Beliefs
Emotional State
Social Status
Influence
Cultural Affiliation
Importance Score
Simulation Tier
```

Exemple conceptuel :

```text
Agent
├── identity
│   ├── id
│   ├── name
│   └── generation
│
├── biology
│   ├── age
│   ├── health
│   ├── energy
│   └── reproduction
│
├── genome
│   ├── intelligence
│   ├── curiosity
│   ├── aggression
│   └── adaptability
│
├── personality
│   ├── sociability
│   ├── empathy
│   ├── impulsivity
│   └── flexibility
│
├── cognition
│   ├── memories
│   ├── beliefs
│   ├── knowledge
│   └── goals
│
├── social
│   ├── relationships
│   ├── group
│   ├── status
│   └── influence
│
└── runtime
    ├── importance
    ├── simulation_tier
    └── last_active_tick
```

---

# 5. Differential Simulation

Genesis ne doit jamais simuler tous les agents au même niveau de détail.

Chaque agent possède un niveau :

```text
TIER 0 — Dormant
TIER 1 — Background
TIER 2 — Active
TIER 3 — Important
TIER 4 — Historical
```

## TIER 0 — Dormant

Simulation statistique.

Aucun LLM.

---

## TIER 1 — Background

Simulation comportementale légère.

```text
Needs
+
Genome
+
Personality
+
Local environment
```

---

## TIER 2 — Active

Simulation individuelle détaillée.

Mémoire récente.

Relations.

Objectifs.

---

## TIER 3 — Important

Contexte riche.

Mémoire RAG.

Relations approfondies.

LLM possible.

---

## TIER 4 — Historical

Agent ayant une importance historique.

Contexte maximal autorisé.

LLM haut niveau.

Archivage renforcé.

---

# 6. Importance dynamique

L'importance n'est jamais permanente.

Elle est recalculée régulièrement.

Conceptuellement :

```text
importance =
    social_status
  + network_influence
  + knowledge_contribution
  + uniqueness
  + recent_activity
  + historical_significance
  + player_interest
```

Les coefficients doivent rester configurables.

Un individu anonyme peut donc devenir :

```text
BACKGROUND
    ↓
ACTIVE
    ↓
IMPORTANT
    ↓
HISTORICAL
```

à la suite d'une découverte, d'un conflit, d'une invention ou d'une rencontre.

---

# 7. Simulation temporelle

Genesis fonctionne avec un système de ticks.

```text
Tick
 │
 ├── Environment
 ├── Biology
 ├── Needs
 ├── Movement
 ├── Reproduction
 ├── Social interactions
 ├── Events
 ├── Memory
 ├── Culture
 ├── Importance recalculation
 ├── Civilization
 └── Nodyx synchronization
```

Toutes les opérations n'ont cependant pas besoin d'être exécutées à chaque tick.

Exemple :

```text
Physics             → every tick
Biology             → every tick
Social              → variable
Memory consolidation→ every N ticks
Importance          → every N ticks
Culture             → event driven
LLM                 → event driven
Nodyx publication   → event driven
```

---

# 8. Event Bus

Genesis doit être **event-driven**.

Exemples :

```text
AgentBorn
AgentDied
AgentMoved
FoodDiscovered
FoodConsumed
AgentMet
ConversationStarted
ConversationFinished
RelationshipChanged
BeliefCreated
MemoryCreated
MemoryRevised
GroupCreated
ConflictStarted
ConflictEnded
DiscoveryMade
TechnologyCreated
TraditionCreated
InstitutionCreated
CivilizationCreated
NodyxPublicationRequested
NodyxPublicationCreated
```

Les événements sont la colonne vertébrale du système.

---

# 9. Protection contre les cascades

Chaque événement possède :

```text
event_id
parent_event_id
cascade_depth
tick
priority
source
```

Le système impose :

```text
MAX_EVENTS_PER_TICK
MAX_CASCADE_DEPTH
MAX_AGENT_EVENTS_PER_TICK
```

Lorsque les limites sont atteintes :

```text
Immediate
   ↓
Deferred
   ↓
Aggregated
   ↓
Summarized
```

Le monde ne doit jamais être capable de s'effondrer à cause d'une cascade infinie.

---

# 10. Architecture mémoire

La mémoire individuelle est hiérarchique.

```text
Short Term
     ↓
Episodic Memory
     ↓
Semantic Memory
     ↓
Important Memories
     ↓
Core Identity
```

Un souvenir peut contenir :

```json
{
  "memory_id": "mem_291",
  "content": "Les soldats ont brûlé notre village.",
  "emotion": "fear",
  "confidence": 0.72,
  "importance": 0.81,
  "world_event_reference": "evt_819",
  "divergence": 0.64
}
```

---

# 11. Context Builder

Le LLM ne reçoit jamais toute la vie d'un agent.

Le Context Builder sélectionne :

```text
Current Situation
+
Relevant Memories
+
Relevant Relationships
+
Relevant Beliefs
+
Current Goals
+
World Facts
+
Cultural Context
+
Recent Events
```

La sélection peut utiliser :

- importance ;
- récence ;
- tags ;
- relations ;
- embeddings ;
- similarité sémantique ;
- contexte géographique ;
- contexte social.

Le contexte final doit respecter un budget.

---

# 12. Memory Anchoring

Chaque mémoire importante devrait conserver un lien vers son origine objective.

```text
Memory
   │
   └── world_event_reference
              │
              ↓
        Objective Event
```

Cela permet :

- debug ;
- reconstruction historique ;
- analyse de divergence ;
- génération d'archives ;
- comparaison entre vérité et croyance.

---

# 13. Mémoire collective

La mémoire collective possède son propre cycle :

```text
Experience
    ↓
Story
    ↓
Transmission
    ↓
Repetition
    ↓
Consensus
    ↓
Collective Memory
    ↓
Cultural Fact
    ↓
Tradition / Myth / Institution
```

Une histoire ne devient donc pas immédiatement une vérité culturelle.

Elle doit être adoptée.

---

# 14. Memes culturels

Un mème culturel est une unité transmissible :

```text
Meme
├── concept
├── origin
├── creator
├── variants
├── popularity
├── consensus
├── transmission_rate
├── cultural_groups
└── world_event_reference
```

Les mèmes peuvent évoluer.

```text
Version A
   ↓
Version B
   ↓
Version C
   ↓
Tradition
```

Cela permet l'apparition spontanée de :

- proverbes ;
- mythes ;
- croyances ;
- chansons ;
- symboles ;
- coutumes ;
- théories ;
- idéologies.

---

# 15. Validation des actions

Toute action importante traverse trois couches.

## Layer 1 — Physical Validation

Obligatoire.

```text
Existe-t-il ?
Est-il présent ?
A-t-il les ressources ?
L'action est-elle physiquement possible ?
```

Impossible :

```text
→ REJECT
```

---

## Layer 2 — Social Validation

Contextuelle.

Elle calcule les conséquences possibles :

```text
reputation_loss
revenge_risk
group_exclusion
legal_risk
resource_loss
relationship_damage
political_consequence
```

Elle ne bloque généralement pas l'action.

---

## Layer 3 — Narrative Validation

Elle mesure la cohérence avec :

```text
Personality
History
Beliefs
Goals
Relationships
Emotional State
```

Une faible cohérence ne signifie pas automatiquement :

```text
REJECT
```

Elle signifie plutôt :

```text
LOW COHERENCE
→ higher cost
→ lower probability
→ alternative
→ failure
→ dramatic consequence
```

Une personnalité flexible possède des seuils plus permissifs.

---

# 16. LLM Runtime

Le LLM n'est pas le moteur du monde.

Il est un **acteur cognitif spécialisé**.

Architecture :

```text
World State
    ↓
Context Builder
    ↓
Model Router
    ↓
LLM
    ↓
Structured Output
    ↓
Validation
    ↓
Event Bus
    ↓
World State
```

---

# 17. Structured Output

Une sortie LLM doit être structurée.

Exemple :

```json
{
  "dialogue": [],
  "decision": {
    "action": "talk",
    "target": "agent_102"
  },
  "relationship_delta": -0.12,
  "new_beliefs": [],
  "new_memories": [],
  "intentions": [],
  "emotional_delta": {},
  "generated_events": []
}
```

Le texte est secondaire.

Les conséquences structurées sont prioritaires.

---

# 18. Fallback hierarchy

Le monde ne doit jamais dépendre d'un LLM disponible à 100 %.

```text
LLM Large
    ↓ failure
LLM Medium
    ↓ failure
LLM Small / Local
    ↓ failure
Behavioral System
    ↓ failure
Default Action
```

Une panne LLM ne doit jamais arrêter Genesis.

---

# 19. Model Router

Le choix du modèle dépend de :

```text
Agent Importance
Event Importance
Context Complexity
Latency
Budget
Availability
```

Exemple :

```text
Village resident + mundane action
→ behavioral AI

Important scientist + discovery
→ Medium

Civilization leader + historical decision
→ Large
```

Un agent peut temporairement monter en niveau.

---

# 20. Conversation Scheduler

Les conversations sont asynchrones.

Jamais :

```text
1000 agents
    ↓
1000 LLM calls
    ↓
explosion
```

Mais :

```text
Events
   ↓
Priority Queue
   ↓
Conversation Scheduler
   ↓
Budget
   ↓
LLM Workers
   ↓
Structured Results
```

Le Scheduler applique :

```text
MAX_CONCURRENT_CONVERSATIONS
MAX_LLM_CALLS_PER_TICK
TOKEN_BUDGET
TIMEOUT
PRIORITY
```

---

# 21. Nodyx Integration Layer

Genesis ne doit pas dépendre directement des détails internes de Nodyx.

Il possède un **Nodyx Gateway**.

```text
Genesis
   ↓
Nodyx Gateway
   ↓
Nodyx API
```

Exemples d'actions :

```text
ForumCreateThread
ForumReply
WikiCreatePage
WikiEditPage
CanvasCreate
CanvasUpdate
CalendarCreateEvent
PollCreate
GameCreate
GameMove
NotificationCreate
```

---

# 22. Digital Agency

Les habitants de Genesis peuvent progressivement acquérir des capacités numériques.

Au début :

```text
No Internet
```

Puis :

```text
Read
   ↓
Communicate
   ↓
Publish
   ↓
Create
   ↓
Collaborate
   ↓
Organize
   ↓
Build Digital Culture
```

Cette progression doit être une **évolution**, pas une fonctionnalité activée artificiellement.

---

# 23. Digital Civilization

Une civilisation avancée peut créer :

### Forums

```text
Conseil des anciens
Place publique
Académie scientifique
Forum religieux
Marché
```

### Wiki

```text
Histoire
Sciences
Généalogies
Géographie
Religion
Technologies
```

### Canvas

```text
Cartes
Plans
Schémas
Œuvres
Frontières
Architecture
```

### Calendar

```text
Fêtes
Guerres
Anniversaires
Cycles agricoles
Événements religieux
```

### Games

```text
Jeux de stratégie
Jeux sociaux
Jeux abstraits
Sports
Compétitions
```

---

# 24. Human / Civilization Interaction

Les humains peuvent devenir des acteurs externes.

Ils peuvent :

```text
Observe
 ↓
Read
 ↓
Comment
 ↓
Interact
 ↓
Influence
```

Mais Genesis doit conserver une séparation claire entre :

```text
Human Actor
```

et

```text
Simulated Actor
```

Le système doit toujours savoir qui est qui.

---

# 25. Human Interaction Safety Boundary

Un humain peut interagir avec les espaces numériques d'une civilisation.

Cependant, il ne doit pas pouvoir :

```text
injecter arbitrairement des souvenirs
modifier le World State
modifier le génome
réécrire l'histoire
usurper une identité d'agent
```

sans passer par des mécanismes explicitement prévus.

Les interactions humaines deviennent elles-mêmes des événements :

```text
HumanJoinedForum
HumanPosted
HumanReplied
HumanCreatedArtifact
HumanInteractedWithAgent
```

---

# 26. Nodyx comme couche de manifestation

Genesis possède son monde.

Nodyx en est la **couche publique**.

```text
GENESIS
    │
    │ simulation
    ↓
WORLD
    │
    │ culture
    ↓
DIGITAL ARTIFACTS
    │
    ↓
NODYX
    │
    ├── Forum
    ├── Wiki
    ├── Canvas
    ├── Calendar
    ├── Games
    └── Community
```

Le monde simulé peut ainsi produire un véritable patrimoine numérique.

---

# 27. SEO et découvrabilité

Le référencement ne doit pas être considéré comme le moteur de la simulation.

Il doit être une conséquence de la production culturelle.

Exemples :

```text
/wiki/empire-solaire
/wiki/guerre-de-velkar
/wiki/religion-des-anciens
/wiki/theorie-du-feu
```

ou :

```text
/civilizations/velkar
/characters/aren
/events/great-fire
/artifacts/creation-fresco
/games/velkarian-chess
```

Chaque artefact doit disposer de métadonnées :

```text
title
description
author
civilization
creation_date
related_events
related_agents
tags
canonical_url
```

La génération de contenu doit rester subordonnée à la qualité et à la cohérence du monde.

---

# 28. Archivage

Genesis doit conserver une histoire exploitable.

```text
World Event
     ↓
Historical Archive
     ↓
Civilization Memory
     ↓
Public Nodyx Artifact
```

Cela permettra notamment de construire :

```text
Timeline
Historical Atlas
Civilization Encyclopedia
Genealogy
Cultural Evolution
Scientific Discoveries
Wars
Religions
Technologies
```

---

# 29. Persistance

Le World State ne doit pas dépendre de la mémoire du processus.

Architecture recommandée :

```text
Live World
    ↓
Event Log
    ↓
Snapshots
    ↓
Persistent Storage
```

Deux mécanismes complémentaires :

### Snapshot

État courant.

### Event Log

Historique des mutations.

Cela permet :

```text
Save
Load
Replay
Rollback
Debug
Time Travel
```

---

# 30. Determinism

Le moteur doit utiliser un système de seed contrôlé.

```text
World Seed
Simulation Seed
Agent Seed
Event Seed
```

Lorsque cela est possible :

```text
same seed
+
same initial state
+
same rules
=
same simulation
```

Les appels LLM devront être traités comme des sources potentiellement non déterministes et leurs résultats devront donc être archivés.

---

# 31. Observability

Genesis doit être observable.

Minimum :

```text
Population
Births
Deaths
Average Age
Energy
Genetic Diversity
Groups
Civilizations
Events
LLM Calls
LLM Tokens
LLM Latency
Memory Count
Event Queue Size
Nodyx API Calls
```

Chaque système doit pouvoir produire des métriques.

---

# 32. Debug Mode

Le développeur doit pouvoir demander :

```text
Why did agent_291 attack agent_102?
```

et obtenir :

```text
Needs:
Hunger = 0.12

Personality:
Aggression = 0.81

Relationship:
-0.63

Memory:
Previous conflict

Social Risk:
0.41

Narrative Coherence:
0.77

Decision:
ATTACK

Validation:
PASSED
```

Genesis doit être explicable même lorsque le comportement est émergent.

---

# 33. Repository

Repository cible :

```text
nodyx-genesis/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── ARCHITECTURE.md
├── ROADMAP.md
├── CHANGELOG.md
│
├── docs/
│   ├── vision.md
│   ├── world-model.md
│   ├── agents.md
│   ├── biology.md
│   ├── evolution.md
│   ├── memory.md
│   ├── collective-memory.md
│   ├── culture.md
│   ├── society.md
│   ├── civilization.md
│   ├── validation.md
│   ├── differential-simulation.md
│   ├── llm-runtime.md
│   ├── context-builder.md
│   ├── event-bus.md
│   ├── scheduler.md
│   ├── nodyx-integration.md
│   ├── digital-agency.md
│   ├── persistence.md
│   ├── observability.md
│   └── security.md
│
├── genesis/
│   ├── core/
│   ├── world/
│   ├── biology/
│   ├── evolution/
│   ├── agents/
│   ├── memory/
│   ├── society/
│   ├── culture/
│   ├── civilization/
│   ├── events/
│   ├── simulation/
│   ├── llm/
│   ├── validation/
│   ├── scheduler/
│   ├── persistence/
│   └── nodyx/
│
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── simulation/
│   └── scenarios/
│
├── examples/
│   ├── two_entities/
│   ├── small_tribe/
│   └── civilization/
│
├── tools/
│   ├── replay/
│   ├── inspector/
│   ├── world-viewer/
│   └── statistics/
│
└── schemas/
    ├── world-state/
    ├── events/
    ├── agents/
    └── nodyx/
```

---

# 34. Principes d'ingénierie

Genesis doit respecter quelques règles non négociables.

### Rule 01 — Le LLM ne possède jamais le World State.

### Rule 02 — Une action impossible physiquement est rejetée.

### Rule 03 — Une sortie LLM invalide ne doit jamais arrêter le monde.

### Rule 04 — Toute mutation importante passe par un événement.

### Rule 05 — Toute donnée historique importante doit être traçable.

### Rule 06 — Les agents ne sont pas tous simulés au même niveau.

### Rule 07 — Les coûts LLM sont explicitement budgétés.

### Rule 08 — La mémoire subjective ne modifie pas rétroactivement la vérité objective.

### Rule 09 — Les artefacts numériques sont des conséquences du monde, pas des décorations.

### Rule 10 — Le système doit rester observable et rejouable.

---

# 35. Roadmap

## Genesis 0.0.1 — Two Entities

Objectif :

```text
World
+
2 entities
+
movement
+
resources
+
energy
+
reproduction
+
death
```

Aucun LLM.

---

## Genesis 0.0.2 — Life

```text
100+ entities
Genome
Mutation
Selection
Aging
Population statistics
```

Aucun LLM.

---

## Genesis 0.0.3 — Individual

```text
Memory
Personality
Needs
Goals
Basic decisions
```

Toujours aucun LLM.

---

## Genesis 0.0.4 — Communication

```text
Signals
Detection
Conditioning
Basic social interaction
```

---

## Genesis 0.0.5 — Society

```text
Groups
Relationships
Language primitives
First LLM interactions
Event Bus
Scheduler
Context Builder
```

---

## Genesis 0.0.6 — Civilization

```text
Villages
Cities
Agriculture
Economy
Politics
Religion
Technology
Collective Memory
```

---

## Genesis 0.1 — Digital Civilization

```text
Nodyx identity
Forum
Wiki
Canvas
Calendar
Games
Public archives
Human interaction
```

---

# 36. Vision finale

Le résultat recherché n'est pas :

> « un jeu dans lequel des IA jouent à être des humains ».

Le résultat recherché est :

> **un système dans lequel un monde artificiel évolue jusqu'à produire spontanément sa propre civilisation, puis commence à construire une présence numérique dans Nodyx.**

À terme :

```text
Genesis
   ↓
Life
   ↓
Mind
   ↓
Society
   ↓
Culture
   ↓
Civilization
   ↓
Digital Culture
   ↓
Nodyx
   ↓
Humans
   ↓
Feedback
   ↓
Genesis
```

Le véritable objectif est donc une **boucle civilisationnelle ouverte** :

```text
WORLD
  ↓
EXPERIENCE
  ↓
MEMORY
  ↓
CULTURE
  ↓
ARTIFACT
  ↓
INTERNET
  ↓
HUMAN OBSERVATION / INTERACTION
  ↓
NEW EVENTS
  ↓
WORLD
```

Genesis ne produit alors plus seulement une simulation.

Il produit un **univers vivant capable de laisser une empreinte numérique persistante**.