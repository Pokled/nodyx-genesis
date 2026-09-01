# GENESIS — Technical Foundation & Implementation Plan

> **Projet : Genesis**
>
> Simulation évolutive d'un monde artificiel allant de la matière primitive à la civilisation, puis potentiellement au-delà.
>
> Genesis simule un monde autonome dans lequel les événements émergent des interactions entre biologie, environnement, individus, sociétés, cultures, technologies et institutions.
>
> Le joueur n'est pas un personnage.
>
> **Le joueur est une entité extérieure au monde : son Dieu, son observateur et parfois son perturbateur.**

---

# 1. Vision

Genesis doit simuler une évolution extrêmement longue :

```text
Matière
   ↓
Chimie
   ↓
Molécules complexes
   ↓
Auto-réplication
   ↓
Vie primitive
   ↓
Organismes
   ↓
Espèces
   ↓
Intelligence
   ↓
Communication
   ↓
Individus
   ↓
Groupes
   ↓
Culture
   ↓
Tribus
   ↓
Villages
   ↓
Villes
   ↓
Civilisations
   ↓
Religion
   ↓
Économie
   ↓
Politique
   ↓
Science
   ↓
Technologie
   ↓
Civilisations avancées
   ↓
Exploration spatiale
   ↓
Entités extérieures potentielles
```

Il ne s'agit pas d'écrire cette histoire à l'avance.

Le système doit créer les conditions permettant à cette histoire d'émerger.

---

# 2. Architecture générale

```text
                         ┌──────────────────────┐
                         │       PLAYER         │
                         │   Observateur / Dieu │
                         └──────────┬───────────┘
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │       GODOT          │
                         │ Visualization / UI   │
                         └──────────┬───────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────┐
│                       GENESIS                            │
│                                                         │
│  ┌─────────────┐     ┌──────────────┐                  │
│  │ World State │◄───►│ Event System  │                  │
│  └──────┬──────┘     └──────┬───────┘                  │
│         │                    │                          │
│         ▼                    ▼                          │
│  ┌─────────────┐     ┌──────────────┐                  │
│  │ Simulation  │     │   Scheduler  │                  │
│  └──────┬──────┘     └──────┬───────┘                  │
│         │                    │                          │
│         ▼                    ▼                          │
│  ┌─────────────┐     ┌──────────────┐                  │
│  │   Agents    │◄───►│ Memory/RAG   │                  │
│  └──────┬──────┘     └──────────────┘                  │
│         │                                                │
│         ▼                                                │
│  ┌────────────────────────────────────┐                 │
│  │ Society / Culture / Economy / Law │                 │
│  └────────────────┬───────────────────┘                 │
│                   │                                     │
│                   ▼                                     │
│          ┌──────────────────┐                           │
│          │ Civilization     │                           │
│          └──────────────────┘                           │
│                                                         │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
                ┌───────────────┐
                │      Nodyx    │
                │ Community Hub │
                └───────────────┘
```

---

# 3. Séparation fondamentale des responsabilités

## Genesis

Genesis est la **vérité mécanique du monde**.

Il décide :

- ce qui existe ;
- où se trouve une entité ;
- quelles ressources sont disponibles ;
- qui est vivant ;
- quelles actions sont physiquement possibles ;
- quelles conséquences objectives ont eu lieu ;
- comment le monde évolue.

Genesis ne doit jamais dépendre du texte produit par un LLM pour maintenir la cohérence physique.

---

# 4. Godot

Godot est la **fenêtre vers le monde**.

Il doit permettre :

- vue globale de la planète ;
- zoom ;
- observation des individus ;
- observation des villes ;
- cartes politiques ;
- cartes religieuses ;
- cartes économiques ;
- arbres technologiques ;
- historique ;
- événements majeurs ;
- statistiques ;
- chronologie ;
- observation d'une civilisation ;
- observation d'un individu ;
- visualisation des relations ;
- visualisation des migrations ;
- visualisation des guerres ;
- visualisation de l'évolution.

Godot ne doit pas être responsable de la simulation.

```text
Genesis → API / State Stream → Godot
```

Cela permet éventuellement de faire tourner Genesis sans interface.

---

# 5. Nodyx

Nodyx devient la **couche communautaire du monde**.

Une civilisation pourra potentiellement disposer :

- d'un compte ;
- de forums ;
- de discussions ;
- de groupes ;
- de messages ;
- de profils ;
- d'archives ;
- d'espaces communautaires ;
- de discussions vocales ;
- de contenus générés par les agents.

Le monde Genesis devient ainsi observable depuis un environnement communautaire réel.

---

# 6. Principe fondamental : deux réalités

Genesis doit maintenir deux niveaux de vérité.

## Objective History

Ce qui s'est réellement passé.

```text
EVENT #819

Year: 142
Location: Village A

Cause:
Natural wildfire

Deaths:
17

Buildings destroyed:
31
```

Cette histoire appartient à Genesis.

Elle ne doit pas être modifiée par les croyances des agents.

---

## Subjective History

Ce que les agents pensent qu'il s'est passé.

Exemple :

```text
MEMORY

"The gods burned our village."

Confidence: 0.91
Origin: Event #819
Divergence: HIGH
```

Les deux peuvent coexister.

---

# 7. Les agents

Un agent possède plusieurs couches.

```text
Agent
│
├── Physical State
│
├── Biological State
│
├── Genome
│
├── Personality
│
├── Needs
│
├── Emotions
│
├── Knowledge
│
├── Individual Memory
│
├── Relationships
│
├── Beliefs
│
├── Social Status
│
└── Importance
```

Tous les agents ne sont pas égaux en coût de simulation.

---

# 8. Differential Simulation

Genesis ne doit jamais traiter 10 000 agents comme 10 000 personnages principaux.

Chaque agent possède un niveau de simulation.

```text
BACKGROUND
    ↓
ACTIVE
    ↓
IMPORTANT
    ↓
HISTORICAL
```

## Background

Simulation statistique.

Pas de LLM.

---

## Active

Simulation comportementale.

Mémoire réduite.

Pas nécessairement de LLM.

---

## Important

Mémoire détaillée.

Relations détaillées.

LLM occasionnel.

---

## Historical

Simulation maximale.

Contexte riche.

LLM puissant si nécessaire.

Archivage important.

---

# 9. Importance dynamique

L'importance d'un agent est recalculée périodiquement.

Variables possibles :

```text
social_status
influence_network
knowledge
relationships
uniqueness
recent_activity
historical_significance
player_interest
```

Formule initiale :

```text
importance =
    0.25 * social_status
  + 0.20 * influence_network
  + 0.15 * knowledge
  + 0.15 * uniqueness
  + 0.10 * recent_activity
  + 0.10 * historical_significance
  + 0.05 * player_interest
```

Cette formule doit rester configurable.

---

# 10. Validation des actions

Toute action passe par trois niveaux.

```text
Action
  │
  ▼
Physical Validation
  │
  ▼
Social Validation
  │
  ▼
Narrative Validation
  │
  ▼
Consequences
```

---

## 10.1 Physical Validation

Obligatoire.

Elle vérifie :

- existence ;
- position ;
- distance ;
- ressources ;
- capacités ;
- outils ;
- énergie ;
- environnement ;
- lois physiques.

Une action impossible est rejetée.

---

## 10.2 Social Validation

Elle évalue :

- relations ;
- lois ;
- culture ;
- réputation ;
- statut ;
- témoins ;
- obligations ;
- institutions.

Elle produit des conséquences probabilistes.

Exemple :

```text
Action:
Trahir un allié

Consequences:

reputation_loss: 0.85
exclusion:       0.70
revenge:         0.60
conflict:        0.30
resource_loss:   0.50
```

---

## 10.3 Narrative Validation

Elle mesure la cohérence avec :

- personnalité ;
- mémoire ;
- croyances ;
- objectifs ;
- historique ;
- état émotionnel.

Elle ne doit pas devenir une prison.

Une action incohérente peut toujours arriver.

Elle devient simplement plus coûteuse, improbable ou surprenante.

---

# 11. Mémoire

La mémoire est subjective.

Chaque souvenir peut contenir :

```text
memory_id
agent_id
content
emotion
confidence
importance
created_at
last_recalled
world_event_reference
divergence
tags
```

Exemple :

```text
Memory #291

Content:
"Les soldats ont brûlé notre village."

World Event:
#819

Confidence:
0.72

Divergence:
MEDIUM

Tags:
war
village
soldiers
fire
trauma
```

---

# 12. Memory Anchoring

Un souvenir peut rester lié à l'événement objectif qui l'a provoqué.

```text
Subjective Memory
       │
       └──────► World Event
```

Cela permet :

- le debug ;
- la comparaison vérité/perception ;
- l'analyse historique ;
- la génération d'archives ;
- la détection de mythes ;
- l'étude des déformations culturelles.

---

# 13. RAG / Context Builder

Le LLM ne reçoit jamais toute la vie d'un agent.

Le Context Builder sélectionne les informations pertinentes.

```text
Current Event
      │
      ▼
Context Builder
      │
      ├── Recent Memory
      ├── Important Memory
      ├── Relationships
      ├── Beliefs
      ├── Goals
      ├── Collective Culture
      └── Relevant History
              │
              ▼
             LLM
```

Les souvenirs doivent être indexables par :

- personne ;
- lieu ;
- événement ;
- émotion ;
- thème ;
- époque ;
- importance ;
- relation.

---

# 14. Mémoire collective

La mémoire collective est distincte de la mémoire individuelle.

```text
Individual Experience
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
Culture
        ↓
Beliefs
        ↓
Future Decisions
```

---

# 15. Émergence des mythes

Une histoire ne devient pas automatiquement une vérité culturelle.

Elle passe par plusieurs étapes.

```text
Experience
   ↓
Rumor
   ↓
Story
   ↓
Repeated Story
   ↓
Collective Memory
   ↓
Legend
   ↓
Myth / Tradition
```

Le système peut utiliser un score de consensus.

```text
consensus = 0.0 → anecdote
consensus = 0.3 → rumeur
consensus = 0.6 → récit connu
consensus = 0.8 → tradition
consensus = 0.95 → fait culturel
```

Les seuils restent configurables.

---

# 16. Transmission culturelle

Les informations peuvent être transmises par :

- parents ;
- amis ;
- conversations ;
- cérémonies ;
- religion ;
- écoles ;
- institutions ;
- dirigeants ;
- livres ;
- médias ;
- monuments ;
- propagande.

Les institutions pourront ensuite devenir des multiplicateurs de diffusion.

---

# 17. Event Bus

Le monde fonctionne autour d'événements.

Exemples :

```text
ENTITY_BORN
ENTITY_DIED
ENTITY_MET
RESOURCE_FOUND
RESOURCE_DEPLETED
CONFLICT_STARTED
CONFLICT_ENDED
DISCOVERY_MADE
RELATION_CHANGED
BELIEF_CHANGED
GROUP_CREATED
GROUP_DISSOLVED
CITY_FOUNDED
WAR_STARTED
TREATY_SIGNED
RELIGION_CREATED
TECHNOLOGY_DISCOVERED
```

Les événements sont la colonne vertébrale de Genesis.

---

# 18. Scheduler

Un événement ne doit pas nécessairement être traité immédiatement.

Le Scheduler gère :

- priorité ;
- délai ;
- dépendances ;
- importance ;
- coût ;
- cascade depth.

---

# 19. Protection contre les cascades

Une conversation peut provoquer :

```text
Conversation
   ↓
Insulte
   ↓
Conflit
   ↓
Famille intervient
   ↓
Groupe intervient
   ↓
Ville intervient
   ↓
Guerre
```

Cela peut être passionnant.

Mais techniquement dangereux.

Genesis doit donc imposer :

```text
MAX_EVENTS_PER_TICK
MAX_CASCADE_DEPTH
MAX_LLM_CALLS_PER_TICK
```

Un événement supplémentaire peut être :

```text
executed
delayed
aggregated
summarized
discarded
```

---

# 20. Agrégation différentielle

L'agrégation ne doit pas simplement prendre la moyenne.

Genesis doit rechercher les individus atypiques.

Exemple :

```text
Famine affects 10,000 people.

9,700:
→ hunger
→ migration
→ reduced reproduction

300:
→ political activism

20:
→ rebellion

3:
→ revolutionary leaders

1:
→ discovers alternative food source
```

Les 9 700 peuvent être agrégés.

Les 1, 3 ou 20 individus importants doivent être simulés individuellement.

---

# 21. Escalation temporaire

Un agent Background peut devenir soudainement important.

Exemple :

```text
Unknown farmer
      ↓
finds rare resource
      ↓
importance ↑
      ↓
ACTIVE
      ↓
discovers technology
      ↓
IMPORTANT
      ↓
changes civilization
      ↓
HISTORICAL
```

L'inverse est également possible.

---

# 22. LLM Router

Les modèles sont choisis selon :

```text
Agent Importance
+
Event Importance
+
Available Budget
+
Required Complexity
```

Exemple :

```text
Tiny
→ conversations banales

Medium
→ décisions sociales

Large
→ dirigeants
→ crises
→ découvertes
→ événements historiques
```

Le système doit pouvoir dégrader automatiquement.

---

# 23. Structured Output

Le LLM ne doit jamais être la source directe de vérité.

Il produit une proposition structurée.

Exemple :

```json
{
  "dialogue": [],
  "relationship_delta": 0.12,
  "new_beliefs": [],
  "new_memories": [],
  "intentions": [],
  "emotional_change": {},
  "actions_proposed": []
}
```

Genesis valide ensuite chaque conséquence.

---

# 24. Fallback System

Hiérarchie :

```text
Large LLM
    ↓
Medium / Local LLM
    ↓
Behavioral AI
    ↓
Default Action
```

Une panne LLM ne doit jamais arrêter le monde.

---

# 25. Behavioral AI

Le Behavioral AI est le filet de sécurité déterministe.

Il utilise :

```text
Needs
Personality
Memory
Environment
Relationships
Goals
```

Exemple :

```text
IF hungry
AND food_known
→ seek_food

IF threatened
AND aggression_low
→ flee

IF threatened
AND aggression_high
→ confront

IF lonely
AND sociability_high
→ seek_social_contact
```

Ce système doit rester fonctionnel sans aucun LLM.

---

# 26. Simulation par phases

Genesis fonctionne par couches.

```text
PHASE 1
Environment

PHASE 2
Biology

PHASE 3
Movement

PHASE 4
Needs

PHASE 5
Interactions

PHASE 6
Society

PHASE 7
Culture

PHASE 8
Economy

PHASE 9
Politics

PHASE 10
Technology

PHASE 11
History / Memory

PHASE 12
External Systems
```

Toutes les couches ne sont pas actives dès le départ.

---

# 27. Roadmap

## Genesis 0.0.1 — Two Entities

Objectif :

**Faire vivre deux entités.**

Pas de LLM.

Pas de civilisation.

Pas de Nodyx.

Uniquement :

- WorldState ;
- environnement ;
- ressources ;
- déplacement ;
- énergie ;
- reproduction ;
- mutation ;
- mort ;
- persistance ;
- seed déterministe.

---

# 28. Genesis 0.0.2 — Life

Objectif :

**Créer une population évolutive.**

Ajouts :

- 100+ entités ;
- génome ;
- traits ;
- mutation ;
- vieillissement ;
- sélection naturelle ;
- diversité génétique ;
- statistiques.

---

# 29. Genesis 0.0.3 — Agents

Ajouts :

- personnalité ;
- besoins ;
- mémoire ;
- comportement ;
- préférences ;
- relations simples.

Toujours sans LLM.

---

# 30. Genesis 0.0.4 — Communication

Ajouts :

- signaux ;
- communication ;
- apprentissage ;
- association signal/résultat ;
- premiers comportements sociaux.

---

# 31. Genesis 0.0.5 — Society

Ajouts :

- groupes ;
- familles ;
- relations ;
- coopération ;
- conflit ;
- transmission culturelle ;
- premiers LLM locaux.

---

# 32. Genesis 0.0.6 — Civilization

Ajouts :

- villages ;
- agriculture ;
- métiers ;
- économie ;
- villes ;
- politique ;
- lois ;
- institutions.

---

# 33. Genesis 0.0.7 — Culture

Ajouts :

- mythes ;
- traditions ;
- religions ;
- langues ;
- symboles ;
- mémoire collective ;
- transmission culturelle.

---

# 34. Genesis 0.0.8 — Technology

Ajouts :

- découvertes ;
- connaissances ;
- inventions ;
- recherche ;
- diffusion technologique ;
- spécialisations scientifiques.

---

# 35. Genesis 0.0.9 — Nodyx Integration

Les civilisations commencent à exister dans Nodyx.

Exemples :

```text
Civilization #12

Profile
Forum
History
Religion
Technology
Political System
Economy
Population
```

Les agents importants peuvent disposer de profils.

---

# 36. Genesis 0.1.0 — Living World

Objectif :

Le monde tourne durablement.

```text
Genesis
   ↓
Persistence
   ↓
Godot
   ↓
Nodyx
```

Le serveur peut continuer à simuler même lorsque personne ne regarde.

---

# 37. Architecture du repository

Le repository doit être organisé dès le début.

```text
genesis/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
│
├── docs/
│   ├── vision.md
│   ├── architecture.md
│   ├── simulation.md
│   ├── agents.md
│   ├── memory.md
│   ├── culture.md
│   ├── civilization.md
│   ├── llm.md
│   ├── event_system.md
│   ├── performance.md
│   ├── godot.md
│   ├── nodyx.md
│   └── roadmap.md
│
├── genesis/
│   ├── core/
│   ├── world/
│   ├── biology/
│   ├── agents/
│   ├── memory/
│   ├── society/
│   ├── culture/
│   ├── economy/
│   ├── politics/
│   ├── technology/
│   ├── events/
│   ├── scheduler/
│   ├── llm/
│   └── persistence/
│
├── godot/
│   ├── project/
│   ├── scenes/
│   ├── scripts/
│   ├── assets/
│   └── shaders/
│
├── tests/
│   ├── unit/
│   ├── integration/
│   └── simulation/
│
├── tools/
│   ├── debug/
│   ├── benchmarks/
│   └── migration/
│
├── configs/
│   ├── development/
│   ├── simulation/
│   └── production/
│
└── data/
    ├── worlds/
    ├── snapshots/
    └── archives/
```

---

# 38. Principe de développement

Genesis doit être construit **bottom-up**.

Ne pas commencer par :

- LLM ;
- religion ;
- politique ;
- Nodyx ;
- chat ;
- civilisation.

Commencer par :

```text
World
 ↓
Physics
 ↓
Life
 ↓
Agents
 ↓
Society
 ↓
Culture
 ↓
Civilization
 ↓
Technology
 ↓
Nodyx
```

Chaque couche doit pouvoir fonctionner sans dépendre inutilement de la suivante.

---

# 39. Déterminisme

Le moteur doit utiliser un système de seed.

```text
WORLD SEED
     ↓
Deterministic RNG
     ↓
Simulation
```

Avec la même seed et les mêmes paramètres :

```text
World A
=
World B
```

Cela est extrêmement important pour :

- tests ;
- reproduction des bugs ;
- benchmarks ;
- comparaison de versions ;
- recherche scientifique ;
- debug.

---

# 40. Snapshots

Genesis doit pouvoir sauvegarder :

```text
WorldState
+
SimulationTime
+
RNG State
+
Important Events
+
Agent State
```

Ainsi :

```text
Snapshot #001
Snapshot #002
Snapshot #003
...
```

permettent de revenir dans le passé.

---

# 41. Archives historiques

Les événements importants deviennent archivés.

Exemple :

```text
YEAR 1827

EVENT #92831

"The Great Drought"

Objective:
Climate anomaly

Subjective interpretations:

Religion A:
"The gods were angry."

Religion B:
"The old covenant was broken."

Political faction:
"The neighboring kingdom poisoned the rivers."

Scientists:
"Atmospheric cycle anomaly."
```

C'est ici que la **double histoire** devient un élément central de l'expérience joueur.

---

# 42. Le joueur comme Dieu

Le joueur ne doit pas nécessairement posséder un bouton :

> "Créer une guerre."

Il doit plutôt pouvoir :

- observer ;
- influencer ;
- provoquer exceptionnellement ;
- favoriser ;
- protéger ;
- envoyer un signe ;
- modifier une condition environnementale ;
- intervenir directement.

Mais chaque intervention doit produire des conséquences naturelles.

Le monde doit pouvoir interpréter l'intervention du joueur sans connaître sa véritable nature.

---

# 43. Interaction humaine avec les civilisations

À terme, un humain pourra interagir avec le monde.

Il pourra éventuellement :

- lire ;
- écrire ;
- discuter ;
- observer ;
- participer à certains espaces ;
- rencontrer les communautés.

Mais les civilisations ne doivent jamais recevoir automatiquement la vérité sur Genesis.

Le système doit séparer :

```text
PLAYER KNOWLEDGE
       ≠
AGENT KNOWLEDGE
```

---

# 44. Le phénomène communautaire

Une conséquence potentiellement très intéressante est l'apparition d'une communauté humaine autour des civilisations.

Les visiteurs pourraient commencer à :

- suivre une civilisation ;
- discuter de ses événements ;
- analyser ses guerres ;
- comparer ses religions ;
- suivre certains individus ;
- créer des théories ;
- archiver des événements ;
- produire du contenu.

Le monde simulé devient alors progressivement un **objet culturel réel**.

---

# 45. Conspiration / interprétations humaines

Il est prévisible que certains utilisateurs cherchent des connexions entre :

```text
Genesis
Nodyx
Civilisations
Événements
Messages
Interventions du joueur
```

Le système ne doit pas confirmer artificiellement ces théories.

Mais il peut laisser les utilisateurs interpréter les événements.

Cela crée une seconde couche narrative :

```text
REAL WORLD
     ↓
Humans observe Genesis
     ↓
Interpretation
     ↓
Theories
     ↓
Community discussions
```

Genesis possède donc potentiellement :

1. une histoire objective ;
2. une histoire subjective des agents ;
3. une interprétation humaine du monde.

---

# 46. Performance : règle absolue

**La beauté ne doit pas coûter la simulation.**

Le rendu visuel doit être séparé de la simulation.

Les SVG / assets vectoriels peuvent être utilisés pour obtenir un rendu riche sans multiplier inutilement les textures raster.

Mais :

```text
Simulation complexity
≠
Rendering complexity
```

Godot doit pouvoir afficher :

```text
1 000 agents
10 000 agents
100 000 agents
```

avec différents niveaux de détail.

---

# 47. Level of Detail

Exemple :

```text
Zoom très loin
→ civilisation = point / zone

Zoom moyen
→ ville = structure simplifiée

Zoom proche
→ bâtiments

Zoom très proche
→ individus

Observation d'un individu
→ détails complets
```

Le moteur ne doit jamais dessiner inutilement ce que le joueur ne voit pas.

---

# 48. Objectif final

Genesis ne doit pas être simplement :

> "un jeu de simulation."

L'objectif est de construire :

> **un monde artificiel persistant, autonome, observable et socialement vivant.**

Un monde dans lequel :

- les individus naissent ;
- vivent ;
- meurent ;
- aiment ;
- se détestent ;
- mentent ;
- oublient ;
- inventent ;
- croient ;
- commercent ;
- font la guerre ;
- construisent ;
- détruisent ;
- transmettent leurs histoires ;
- créent des religions ;
- développent des technologies ;
- bâtissent des civilisations.

Et surtout :

**le développeur ne connaît pas à l'avance l'histoire qui va être racontée.**

---

# 49. Premier objectif technique

Avant toute fonctionnalité spectaculaire :

```text
Genesis 0.0.1
```

doit pouvoir faire ceci :

```text
START

        ● Entity A

                         ● Entity B


TICK 1
TICK 2
TICK 3
...
```

Puis :

```text
A moves
B moves

A finds food
B finds food

A reproduces

A1 is born

A dies

B survives
```

Le monde doit être sauvegardable.

Puis rechargeable.

Puis rejouable avec la même seed.

**Si cette boucle fonctionne parfaitement, tout le reste peut être construit dessus.**