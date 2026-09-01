# Genesis — World State, Agents, Memory & LLM
## Architecture technique v0.1

> **Le moteur simule la réalité.  
> Les agents interprètent cette réalité.  
> Le LLM propose des décisions.  
> Genesis valide et applique les conséquences.**

---

# 1. Vision

Genesis ne doit pas être une collection de chatbots.

Il doit simuler un monde autonome dans lequel des entités évoluent progressivement :

```text
Matière
  ↓
Vie
  ↓
Organismes
  ↓
Intelligence
  ↓
Communication
  ↓
Groupes
  ↓
Sociétés
  ↓
Civilisations
  ↓
Technologie
  ↓
Culture
  ↓
Politique
  ↓
Religion
  ↓
Économie
  ↓
Histoire
```

Le LLM intervient uniquement lorsque la simulation atteint un niveau où une décision générative apporte une réelle valeur.

---

# 2. Les trois réalités

Genesis doit maintenir trois niveaux distincts.

```text
                 WORLD TRUTH
                      │
            ┌─────────┴─────────┐
            │                   │
            ▼                   ▼
      AGENT MEMORY         AGENT BELIEF
```

## World Truth

Ce qui s'est réellement produit.

## Agent Memory

Ce que l'individu se souvient avoir vécu.

## Agent Belief

Ce que l'individu pense être vrai.

Ces trois éléments peuvent diverger.

### Exemple

```text
WORLD TRUTH
Le roi n'a pas empoisonné son frère.

MEMORY
Aren pense avoir vu le roi entrer dans la chambre.

BELIEF
"Le roi a tué son frère."
```

Cette divergence est fondamentale pour permettre :

- rumeurs ;
- propagande ;
- mythes ;
- religions ;
- erreurs historiques ;
- paranoïa ;
- réécriture culturelle ;
- théories politiques.

---

# 3. Validation des décisions

Le LLM ne modifie jamais directement le monde.

Il propose une action.

```text
LLM
 ↓
Intent
 ↓
Validation
 ↓
World Event
 ↓
World State
```

La validation possède plusieurs couches.

---

## 3.1 Validation physique / logique

**Obligatoire.**

Elle vérifie les règles objectives.

Exemples :

```text
L'agent existe ?
L'agent est vivant ?
La cible existe ?
La cible est accessible ?
L'agent possède l'objet nécessaire ?
L'action est physiquement possible ?
Les ressources existent ?
La technologie existe ?
```

Exemple :

```text
Aren veut fabriquer une machine à vapeur.

Validation :
TECHNOLOGY_STEAM_ENGINE = false

→ Action impossible.
```

---

# 4. Validation sociale

Couche optionnelle.

Elle vérifie si l'action est compatible avec le contexte social.

```text
Relations
Statut
Obligations
Culture
Lois
Normes
Religion
Réputation
```

Exemple :

```text
Aren trahit son groupe.

Physiquement :
possible.

Socialement :
très coûteux.

Conséquences possibles :
- perte de réputation
- exclusion
- conflit
- arrestation
- guerre
```

La validation sociale ne doit généralement **pas empêcher l'action**.

Elle doit surtout déterminer ses conséquences.

---

# 5. Validation narrative

Couche optionnelle.

Elle estime :

> "Cette action est-elle cohérente avec cet individu ?"

Variables :

```text
Personality
History
Beliefs
Goals
Relationships
Current emotional state
Past actions
```

On peut produire :

```text
coherence_score = 0.0 → 1.0
```

Exemple :

```text
Aren
aggressiveness = 0.12
compassion = 0.91
family_attachment = 0.95

Decision:
murder his brother

coherence_score = 0.04
```

Le moteur peut alors :

```text
HIGH COHERENCE
→ action normale

MEDIUM COHERENCE
→ action possible + conséquences

LOW COHERENCE
→ action improbable / alternative / échec
```

### Important

Une faible cohérence ne doit pas nécessairement interdire l'action.

Une personne peut :

- changer ;
- craquer ;
- mentir ;
- agir sous pression ;
- faire une erreur ;
- commettre quelque chose d'inattendu.

L'incohérence doit donc être un **signal**, pas une prison narrative.

---

# 6. Hiérarchie de validation

```text
                LLM INTENT
                    │
                    ▼
          ┌──────────────────┐
          │ Physical / Logic │
          └────────┬─────────┘
                   │
              impossible?
               /       \
             YES        NO
             ↓           ↓
          REJECT    Social Layer
                         │
                         ▼
                   Narrative Layer
                         │
                         ▼
                     CONSEQUENCE
```

---

# 7. Memory Engine

La mémoire d'un agent est hiérarchique.

```text
Agent Memory
│
├── Identity
│
├── Current State
│
├── Recent Memories
│
├── Episodic Memories
│
├── Semantic Knowledge
│
├── Relationship Memories
│
└── Life / Era Summaries
```

---

# 8. Memory Lifecycle

Une expérience suit ce chemin :

```text
World Event
    ↓
Perception
    ↓
Experience
    ↓
Memory Candidate
    ↓
Importance Evaluation
    ↓
Memory
    ↓
Consolidation
    ↓
Long-Term Memory
```

Tous les événements ne deviennent pas des souvenirs.

---

# 9. Memory Importance

Chaque souvenir possède plusieurs propriétés.

```text
importance
emotional_weight
recency
relationship_weight
novelty
repetition
historical_significance
```

Exemple :

```text
"Mangé une pomme."

importance = 0.02
```

contre :

```text
"Mon père est mort."

importance = 0.98
```

---

# 10. Memory Mutation

La mémoire peut évoluer.

```text
EVENT
 ↓
MEMORY
 ↓
REINTERPRETATION
 ↓
MEMORY'
```

Mais la dérive doit être contrôlée.

---

# 11. Memory Anchors

Certains souvenirs sont liés à des faits objectifs.

```text
Memory
├── subjective_content
├── confidence
├── emotional_weight
└── world_event_reference
```

Exemple :

```text
Memory:

"Ils m'ont attaqué dans la forêt."

confidence = 0.71

world_event_reference = EVENT_18491
```

Le souvenir peut être subjectivement modifié sans perdre complètement son ancrage historique.

---

# 12. Memory Revision

Périodiquement, Genesis peut consolider la mémoire.

Déclencheurs possibles :

```text
N événements
N jours
N années
Transition d'ère
Événement majeur
Mort d'un proche
Changement culturel majeur
```

Le système peut alors reconstruire :

```text
Life Summary
Era Summary
Important Memories
```

à partir de :

```text
World Events
+
Existing Memories
+
Agent Perspective
```

Le résultat reste subjectif.

Genesis ne doit pas remplacer :

> "ce que l'agent croit"

par :

> "ce qui s'est réellement passé".

---

# 13. RAG mémoire

La mémoire est interrogeable.

Chaque élément peut posséder :

```text
tags
embedding
entities
location
timestamp
emotion
participants
topic
```

Exemple :

```text
Memory #1942

tags:
war
father
death
Velkar
trauma

entities:
Aren
Father
Velkar

emotion:
grief = 0.91
```

---

# 14. Context Retrieval

Lorsqu'un événement survient :

```text
Current Event
     ↓
Extract Context
     ↓
Tags
     ↓
Memory Search
     ↓
Semantic Search
     ↓
Relationship Search
     ↓
Temporal Search
     ↓
Relevant Memories
```

Le Context Builder sélectionne ensuite les éléments les plus pertinents.

---

# 15. Context Budget

Le contexte du LLM est limité.

Genesis définit :

```text
MAX_CONTEXT_TOKENS
```

Le Context Builder réalise une optimisation.

```text
Context
│
├── Identity
├── Current State
├── Current Event
├── Goals
├── Relationships
├── Relevant Memories
└── Background
```

Si le budget est dépassé :

```text
Low relevance
      ↓
removed first
```

---

# 16. Event Bus

Les agents ne doivent pas communiquer directement avec le moteur.

Ils génèrent des événements.

```text
Agent
 ↓
Intent
 ↓
Event Bus
 ↓
Scheduler
```

L'Event Bus devient le système nerveux de Genesis.

---

# 17. Event Structure

Un événement contient notamment :

```text
event_id
timestamp
event_type
priority
actors
location
cause
context
parent_event
deadline
```

Exemple :

```text
EVENT_8291

type:
SOCIAL_INTERACTION

actor:
AREN

target:
MIRA

location:
VELKAR

priority:
0.81

parent_event:
EVENT_8287
```

---

# 18. Event Cascades

Les événements peuvent générer d'autres événements.

```text
A parle à B
 ↓
B change d'opinion
 ↓
B parle à C
 ↓
C diffuse une information
 ↓
groupe réagit
 ↓
conflit politique
```

C'est un mécanisme important d'émergence.

Mais il faut le contrôler.

---

# 19. Cascade Protection

Chaque événement possède :

```text
cascade_depth
parent_event
```

Genesis peut imposer :

```text
MAX_CASCADE_DEPTH
MAX_EVENTS_PER_TICK
MAX_EVENTS_PER_AGENT
```

Lorsqu'une limite est atteinte :

```text
delay
aggregate
summarize
deprioritize
```

Plutôt que de simplement supprimer.

---

# 20. Agent Importance

Tous les individus ne doivent pas être traités de manière identique.

Chaque agent possède un score d'importance dynamique.

```text
agent_importance =
    social_status
  + influence
  + knowledge
  + relationships
  + uniqueness
  + current_activity
  + historical_significance
  + player_interest
```

---

# 21. Differential Simulation

L'agrégation doit être différentielle.

Une famine touche :

```text
10 000 agents
```

Mais Genesis identifie :

```text
King
Scientists
Farmers
Religious leaders
Military leaders
Artists
Outliers
Player-followed agents
```

Ces individus peuvent recevoir une simulation détaillée.

Le reste est agrégé.

---

# 22. Population Simulation

```text
10 000 agents

        │
        ▼

Behavioral Simulation

        │
        ├── 9 500 agents
        │       ↓
        │    aggregate
        │
        └── 500 important agents
                ↓
             detailed
```

Le seuil doit être dynamique.

---

# 23. LLM Escalation

Un agent peut monter temporairement de niveau.

```text
NORMAL AGENT
    ↓
interesting event
    ↓
importance ↑
    ↓
LLM ACTIVATED
```

Après l'événement :

```text
LLM
 ↓
Decision
 ↓
Consequences
 ↓
Agent returns to low-cost simulation
```

Cela permet une énorme économie de ressources.

---

# 24. Conversation Engine

Une conversation est un événement.

```text
Conversation Request
       ↓
Scheduler
       ↓
Importance
       ↓
Model Router
       ↓
Context Builder
       ↓
LLM
```

---

# 25. Structured LLM Output

Le LLM ne doit pas uniquement produire du texte.

Il doit retourner un résultat structuré.

Exemple conceptuel :

```json
{
  "dialogue": [
    {
      "speaker": "Aren",
      "text": "..."
    },
    {
      "speaker": "Mira",
      "text": "..."
    }
  ],
  "relationship_delta": 12,
  "new_beliefs": [],
  "new_memories": [],
  "intentions": [],
  "emotional_changes": [],
  "knowledge_transfers": []
}
```

Le schéma exact sera défini côté moteur.

---

# 26. Structured Output obligatoire

Le moteur doit considérer le texte généré comme une représentation secondaire.

La donnée importante est :

```text
STRUCTURED RESULT
```

Le dialogue sert à l'expérience du joueur.

Le résultat structuré sert à la simulation.

---

# 27. Conversation Validation

Après réception :

```text
LLM Result
 ↓
Schema Validation
 ↓
Semantic Validation
 ↓
World Validation
 ↓
Apply Consequences
```

Une sortie JSON invalide doit être rejetée ou régénérée.

---

# 28. Model Router

Le choix du modèle dépend de :

```text
Agent Importance
Event Importance
Complexity
Budget
Player Focus
```

Exemple :

```text
Tiny
↓
routine

Medium
↓
social / local decisions

Large
↓
important decisions

Deep
↓
civilizational events
```

---

# 29. Dynamic Model Escalation

Un agent normalement simulé sans LLM peut devenir important.

Exemple :

```text
Unknown farmer
     ↓
discovers strange material
     ↓
scientific potential
     ↓
importance ↑
     ↓
LLM escalation
```

Il peut alors devenir :

```text
scientist
inventor
political figure
religious prophet
military leader
```

Le système ne doit donc jamais attribuer définitivement un "niveau d'importance".

---

# 30. LLM Budget

Genesis possède un budget configurable.

```text
MAX_LLM_CALLS_PER_TICK
MAX_TOKENS_PER_TICK
MAX_DEEP_CALLS
MAX_CONVERSATIONS
```

Le scheduler distribue ce budget.

---

# 31. Priorisation

```text
CRITICAL
HIGH
MEDIUM
LOW
BACKGROUND
```

En surcharge :

```text
CRITICAL → execute
HIGH → execute
MEDIUM → lightweight / delayed
LOW → aggregate
BACKGROUND → statistical simulation
```

---

# 32. Player Focus

L'attention du joueur peut modifier les priorités.

Si le joueur suit :

```text
Aren
```

Genesis augmente temporairement :

```text
Aren priority ↑
Aren relationships ↑
Aren location ↑
Aren events ↑
```

Cela donne au joueur l'impression que le monde réagit à son observation sans tricher avec la simulation.

---

# 33. Observability

Chaque décision importante doit pouvoir être inspectée.

```text
Decision Trace

Agent:
Aren

Event:
Mira refuses conversation

Relevant memories:
#194
#382
#991

Goal:
Repair relationship

Decision:
Attempt conversation

Coherence:
0.73

Model:
Medium

Result:
Conversation accepted
```

---

# 34. Debugging

Genesis doit permettre de répondre :

> Pourquoi cet agent a-t-il fait ça ?

Le moteur doit pouvoir afficher :

```text
World facts
+
Agent state
+
Retrieved memories
+
Beliefs
+
Goals
+
Decision
+
Validation
+
Consequences
```

---

# 35. Simulation Layers

Le système complet est organisé en couches.

```text
LAYER 0
Physics / Resources

LAYER 1
Life

LAYER 2
Evolution

LAYER 3
Behavior

LAYER 4
Communication

LAYER 5
Social

LAYER 6
Culture

LAYER 7
Economy

LAYER 8
Politics

LAYER 9
Religion

LAYER 10
Technology

LAYER 11
Civilization

LAYER 12
External / Cosmic
```

Une couche supérieure ne doit pas être nécessaire pour faire fonctionner les couches inférieures.

---

# 36. Prototype Roadmap

## Genesis 0.0.1 — Two Entities

Aucune IA.

```text
World State
Grid
Resources
Entities
Energy
Movement
Reproduction
Mutation
Persistence
Godot visualization
```

---

## Genesis 0.0.2 — Life

```text
100+ entities
Age
Death
Genetic traits
Natural selection
Population statistics
Genetic diversity
```

---

## Genesis 0.0.3 — Agents

```text
Personality
Basic memory
Needs
Goals
Behavioral decisions
```

Toujours sans LLM.

---

## Genesis 0.0.4 — Communication

```text
Signals
Detection
Learning
Basic communication
```

---

## Genesis 0.0.5 — Society

```text
Groups
Relationships
Shared information
Basic conversations
Event Bus
Scheduler
Context Builder
First local LLM
```

---

## Genesis 0.0.6 — Civilization

```text
Villages
Cities
Agriculture
Specialization
Trade
Economy
Politics
Religion
Technology
```

---

# 37. Principe de développement

Ne jamais implémenter directement :

```text
Civilization
```

avant que :

```text
Entity
↓
Life
↓
Behavior
↓
Communication
↓
Society
```

soient stables.

La civilisation doit être une **conséquence**, pas une fonctionnalité artificiellement posée au-dessus du moteur.

---

# 38. Architecture cible

```text
                    GENESIS
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
 WORLD ENGINE      AGENT ENGINE     EVENT ENGINE
       │               │                │
       │               │                ▼
       │               │            SCHEDULER
       │               │                │
       │               ▼                │
       │          MEMORY ENGINE         │
       │               │                │
       │               ▼                │
       │        CONTEXT BUILDER ◄───────┘
       │               │
       │               ▼
       │          MODEL ROUTER
       │               │
       │          ┌────┼────┐
       │          ▼    ▼    ▼
       │        Tiny Medium Large
       │          │    │    │
       └──────────┴────┴────┘
                    │
                    ▼
              VALIDATION
                    │
                    ▼
               WORLD EVENT
                    │
                    ▼
                WORLD STATE
```

---

# 39. Principe de performance

Le monde doit continuer à vivre même si le LLM est indisponible.

```text
LLM OFF
  ↓
World simulation continues
```

Le LLM est une couche d'intelligence supplémentaire.

Il ne doit jamais être le moteur fondamental de la simulation.

---

# 40. Principe de résilience

Si :

```text
LLM timeout
LLM unavailable
invalid output
budget exhausted
model crash
```

Genesis doit continuer.

Fallback :

```text
LLM
 ↓
failure
 ↓
behavioral AI
 ↓
default decision
```

Le monde ne doit jamais se figer parce qu'un modèle n'a pas répondu.

---

# 41. Principe d'émergence

Genesis ne doit pas coder explicitement :

```text
"Créer une religion."
```

Il doit fournir les conditions permettant son émergence :

```text
Unknown event
+
Human interpretation
+
Shared belief
+
Repetition
+
Social transmission
+
Ritual
+
Institution
```

→ religion potentielle.

Même logique pour :

```text
Politics
Economy
Culture
Science
Technology
War
Mythology
```

---

# 42. Philosophie finale

Genesis doit produire des histoires que personne n'a écrites.

Ni le développeur.

Ni le joueur.

Ni le LLM.

Elles émergent de :

```text
Rules
+
Environment
+
Evolution
+
Memory
+
Personality
+
Relationships
+
Culture
+
Chance
+
Agent decisions
```

Le LLM n'est donc pas l'auteur du monde.

Il est l'un des mécanismes permettant au monde de produire des comportements complexes.

---

# 43. Règle absolue

> **Genesis connaît la vérité.**
>
> **Les agents connaissent seulement leur perception de la vérité.**
>
> **Le LLM ne connaît que le contexte qu'on lui fournit.**
>
> **Le joueur peut observer les trois.**

C'est cette séparation qui doit permettre à Genesis de devenir un véritable **simulateur de monde émergent** plutôt qu'un simple jeu narratif généré par IA.