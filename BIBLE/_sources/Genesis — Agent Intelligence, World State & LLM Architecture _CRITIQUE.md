# Genesis — Agent Intelligence, World State & LLM Architecture

> **Principe fondamental : le LLM ne simule pas le monde.**
>
> Le moteur Genesis simule le monde.  
> Le LLM simule la décision d'un individu à partir d'une vision partielle de ce monde.

---

# 1. Objectif

Genesis doit pouvoir faire vivre :

- des milliers d'individus ;
- des sociétés ;
- des civilisations ;
- des économies ;
- des religions ;
- des systèmes politiques ;
- des cultures ;
- des relations sociales ;
- des conflits ;
- des échanges ;
- des conversations.

Cependant, un LLM possède une fenêtre de contexte limitée et son utilisation est coûteuse.

Il est donc impossible de donner à chaque agent :

```text
tout le World State
+
toute sa vie
+
toutes les conversations
+
tous les événements
```

à chaque décision.

Genesis doit donc séparer :

```text
WORLD STATE
MEMORY
CONTEXT
AGENT
EVENT BUS
LLM
```

---

# 2. Architecture générale

```text
                         WORLD STATE
                              │
             ┌────────────────┴────────────────┐
             │                                 │
             ▼                                 ▼
        EVENT BUS                         MEMORY ENGINE
             │                                 │
             ▼                                 ▼
         SCHEDULER                       CONTEXT BUILDER
             │                                 │
             └────────────────┬────────────────┘
                              ▼
                            AGENT
                              │
                              ▼
                             LLM
                              │
                              ▼
                         WORLD EVENT
                              │
                              ▼
                         WORLD STATE
```

Le système fonctionne en boucle.

---

# 3. World State

Le World State représente la **vérité objective de la simulation**.

Il ne dépend pas de ce qu'un agent pense.

Exemple :

```text
World State

Planet
├── climate
├── geography
├── resources
└── ecosystems

Civilization
├── population
├── territory
├── government
├── economy
├── religion
└── technology

Entity
├── identity
├── location
├── health
├── relationships
├── possessions
├── knowledge
└── current_state
```

---

# 4. Vérité objective vs perception subjective

Un principe fondamental :

```text
WORLD TRUTH
      ≠
AGENT MEMORY
      ≠
AGENT BELIEF
```

Exemple :

```text
WORLD TRUTH

Le frère n'a pas volé l'argent.
```

Mais :

```text
AGENT BELIEF

"Mon frère m'a volé."
```

Et :

```text
AGENT MEMORY

"Je me souviens l'avoir vu prendre l'argent."
```

Le moteur ne doit pas corriger automatiquement la mémoire de l'agent.

Cela permet :

- rumeurs ;
- erreurs ;
- propagande ;
- fausses croyances ;
- mythes ;
- paranoïa ;
- réécriture historique ;
- religions ;
- théories concurrentes.

---

# 5. Agent State

Chaque entité intelligente possède un état propre.

```text
Agent
├── identity
├── personality
├── needs
├── goals
├── emotions
├── beliefs
├── knowledge
├── relationships
├── skills
├── memories
├── culture
├── language
└── current_context
```

L'agent ne connaît pas nécessairement la vérité du World State.

Il possède uniquement :

```text
WHAT I KNOW
WHAT I BELIEVE
WHAT I REMEMBER
WHAT I ASSUME
```

---

# 6. Memory Engine

Le Memory Engine transforme les expériences en mémoire exploitable.

Il ne faut pas stocker toute la vie sous la forme d'un gigantesque prompt.

Genesis utilise une mémoire hiérarchique.

```text
LIFE
 │
 ├── Life Summary
 │
 ├── Era Summaries
 │
 ├── Year Summaries
 │
 ├── Important Events
 │
 └── Episodic Memories
```

---

# 7. Life Summary

Un résumé stable de l'existence.

Exemple :

```text
AREn

Né dans une famille de pêcheurs.
A grandi à Velkar.
Devient forgeron à l'âge adulte.
A participé à la guerre de Velkar.
A rencontré Mira à 34 ans.
```

Ce résumé est constamment mis à jour.

---

# 8. Memory Compression

Les souvenirs anciens peuvent être compressés.

```text
1000 memories
      │
      ▼
100 important memories
      │
      ▼
10 era summaries
      │
      ▼
1 life summary
```

Mais les souvenirs originaux importants peuvent rester accessibles.

La compression ne signifie donc pas :

> suppression définitive.

---

# 9. Mémoire à plusieurs niveaux

## Niveau 0 — Identity

Toujours disponible.

```text
Nom
Age
Origine
Famille
Culture
Langue
```

---

## Niveau 1 — Current State

```text
Situation actuelle
Objectifs
Relations importantes
Émotions
Besoins
```

---

## Niveau 2 — Recent Memory

Événements récents.

```text
Dernière conversation
Dernière journée
Dernier conflit
Dernière découverte
```

---

## Niveau 3 — Long Term Memory

Souvenirs récupérés à la demande.

```text
Voyage
Guerre
Rencontre
Traumatisme
Découverte
Promesse
```

---

## Niveau 4 — Semantic Knowledge

Connaissances générales.

```text
Histoire
Religion
Sciences
Culture
Technologie
Traditions
```

---

# 10. Retrieval Augmented Memory

Lorsqu'un agent doit prendre une décision, Genesis cherche les informations pertinentes.

```text
Agent Query
     │
     ▼
Memory Retrieval
     │
     ├── Recent memories
     ├── Semantic memories
     ├── Emotional memories
     ├── Relationship memories
     └── Historical memories
     │
     ▼
Relevant Context
```

Le LLM ne reçoit donc pas toute la mémoire.

Il reçoit :

> **les souvenirs pertinents pour la situation actuelle.**

---

# 11. Memory Relevance

Chaque souvenir possède un score.

Conceptuellement :

```text
memory_score =
    semantic_relevance
  + emotional_importance
  + relationship_importance
  + recency
  + repetition
  + narrative_importance
```

La pondération exacte devra être expérimentée.

---

# 12. Emotional Memory

Certains événements doivent être plus difficiles à oublier.

Exemple :

```text
Mort d'un parent
Première rencontre amoureuse
Trahison
Guerre
Naissance d'un enfant
Découverte majeure
Humiliation publique
Victoire importante
```

Un souvenir émotionnel peut rester pertinent pendant toute une vie.

---

# 13. Memory Mutation

Les souvenirs ne sont pas nécessairement parfaits.

Une mémoire peut évoluer :

```text
Original Event
      │
      ▼
Memory
      │
      ▼
Reinterpretation
      │
      ▼
Modified Memory
```

Cela permet de simuler :

- souvenirs déformés ;
- reconstruction ;
- légendes personnelles ;
- mythes collectifs.

---

# 14. Context Builder

Le Context Builder est une brique fondamentale.

Sa fonction :

> **Construire le minimum de contexte nécessaire pour qu'un agent puisse prendre une décision cohérente.**

Il rassemble :

```text
Identity
+
Current State
+
Relevant Memories
+
Relevant Knowledge
+
Relationships
+
World Information
+
Current Event
+
Goals
```

Puis produit :

```text
LLM Context
```

---

# 15. Context Budget

Le Context Builder doit respecter une limite.

```text
MAX_CONTEXT_TOKENS
```

Chaque élément possède un coût.

Le système sélectionne les informations les plus importantes.

```text
Context Budget

Identity           ███
Current State      ████
Relationships      ███
Recent Memory      ███
Relevant Memory    █████
World Context     ██
Background         █
```

---

# 16. Context Priority

Ordre approximatif :

```text
1. Current event
2. Current state
3. Immediate goals
4. Important relationships
5. Relevant memories
6. Relevant knowledge
7. Cultural context
8. General background
```

Le système doit toujours privilégier la pertinence plutôt que la quantité.

---

# 17. LLM comme moteur de décision

Le LLM ne doit pas directement modifier le World State.

Il produit une intention ou une décision.

```text
Agent
 │
 ▼
LLM
 │
 ▼
Decision
 │
 ▼
Validation
 │
 ▼
World Event
 │
 ▼
World State
```

Exemple :

```text
LLM Decision

intent = TALK_TO_MIRA
target = MIRA
reason = REPAIR_RELATIONSHIP
```

Genesis décide ensuite si cette action est réellement possible.

---

# 18. Validation

Le moteur doit vérifier :

```text
Agent vivant ?
Agent présent ?
Cible présente ?
Action possible ?
Ressources disponibles ?
Contexte cohérent ?
```

Le LLM ne peut pas :

```text
inventer une ville
inventer une technologie
téléporter un individu
modifier directement la météo
```

sauf si les règles du monde le permettent.

---

# 19. Event Bus

Les agents ne doivent pas communiquer directement de manière incontrôlée.

Genesis utilise un Event Bus.

```text
Agent
 │
 ▼
Event
 │
 ▼
Event Bus
 │
 ▼
Scheduler
```

---

# 20. Event Queue

Chaque événement possède notamment :

```text
event_id
event_type
timestamp
priority
actors
location
cause
context
deadline
```

Exemple :

```text
EVENT #8291

type:
SOCIAL_INTERACTION

actor:
AREN

target:
MIRA

priority:
0.81

reason:
RELATIONSHIP_TENSION
```

---

# 21. Scheduler

Le Scheduler décide quels événements méritent une simulation détaillée.

```text
EVENT
 │
 ▼
PRIORITY
 │
 ├── low
 │    └── aggregate
 │
 ├── medium
 │    └── deterministic / lightweight AI
 │
 ├── high
 │    └── LLM
 │
 └── critical
      └── deep reasoning
```

---

# 22. Tous les événements ne nécessitent pas un LLM

C'est une règle fondamentale.

## Niveau 0 — Simulation classique

```text
Marcher
Dormir
Manger
Travailler
Se déplacer
```

Pas de LLM.

---

## Niveau 1 — Behavioral AI

```text
Acheter de la nourriture
Choisir une route
Suivre une routine
Réagir à un événement banal
```

Pas nécessairement de LLM.

---

## Niveau 2 — Agent léger

```text
Décision inhabituelle
Conflit mineur
Choix social
```

Modèle léger ou logique spécialisée.

---

## Niveau 3 — LLM

```text
Conversation importante
Décision politique
Conflit complexe
Création d'une idée
Relation importante
Découverte
```

---

## Niveau 4 — Deep Reasoning

Très rare.

```text
Crise civilisationnelle
Décision historique
Événement majeur
Premier contact
```

---

# 23. Anti-Explosion LLM

Genesis doit pouvoir gérer :

```text
10 agents
100 agents
1 000 agents
10 000 agents
```

sans multiplier linéairement les appels LLM.

Le principe :

```text
MANY AGENTS
      │
      ▼
SIMULATION
      │
      ▼
EVENTS
      │
      ▼
FILTER
      │
      ▼
IMPORTANT EVENTS
      │
      ▼
LLM
```

---

# 24. Agrégation

Si 10 000 individus ressentent les mêmes effets d'une famine :

Genesis ne lance pas 10 000 appels.

Il simule :

```text
FAMINE
 │
 ├── food shortage
 ├── migration
 ├── crime
 ├── disease
 ├── political tension
 └── mortality
```

Puis sélectionne les individus et événements importants.

---

# 25. Conversation Scheduling

Une conversation entre deux agents devient un événement.

```text
Agent A wants to talk
        │
        ▼
SOCIAL_EVENT
        │
        ▼
Scheduler
        │
        ├── low importance
        │      ↓
        │   summarize
        │
        └── high importance
               ↓
              LLM
```

---

# 26. Conversation Importance

Critères possibles :

```text
Relationship importance
Emotional intensity
Novelty
Conflict
Political importance
Historical importance
Player interest
Narrative importance
```

Une conversation entre deux inconnus peut être résumée.

Une conversation entre :

```text
King + General
Parent + Child
Two lovers
Two political leaders
Two rival scientists
```

peut être simulée beaucoup plus finement.

---

# 27. Conversation Result

Le LLM ne doit pas seulement produire du texte.

Il doit produire des conséquences structurées.

```text
Conversation Result

relationship_delta
new_beliefs
new_memories
knowledge_transfer
emotional_changes
intentions
promises
conflicts
future_events
```

Exemple :

```text
relationship_delta = +14

new_belief:
"Mira trusts Aren again."

new_memory:
"Conversation about the war."

future_intent:
"Visit Aren tomorrow."
```

---

# 28. Conversation → World Event

```text
LLM
 │
 ▼
Conversation
 │
 ▼
Structured Result
 │
 ▼
Validation
 │
 ▼
World Event
 │
 ├── relationship changed
 ├── memory created
 ├── belief changed
 └── future event scheduled
```

---

# 29. Agent Cascade

Une conversation peut créer de nouveaux événements.

```text
A talks to B
     │
     ▼
B changes belief
     │
     ▼
B talks to C
     │
     ▼
C changes political opinion
     │
     ▼
Political event
```

Il faut donc éviter les cascades infinies.

---

# 30. Event Budget

Genesis possède un budget par unité de temps.

```text
MAX_LLM_CALLS_PER_TICK
MAX_HIGH_PRIORITY_EVENTS
MAX_CONVERSATIONS
MAX_DEEP_REASONING
```

Le Scheduler distribue le budget.

---

# 31. Priority Queue

Les événements peuvent être ordonnés :

```text
CRITICAL
HIGH
MEDIUM
LOW
BACKGROUND
```

En cas de surcharge :

```text
CRITICAL → execute
HIGH     → execute
MEDIUM   → aggregate
LOW      → summarize
BACKGROUND → simulate statistically
```

---

# 32. Player Interest

Le joueur peut influencer indirectement la priorité.

Si le joueur observe une civilisation :

```text
Player Focus
     │
     ▼
Civilization Priority ↑
```

Les événements associés peuvent recevoir un bonus.

Cela permet de rendre l'observation interactive sans modifier artificiellement le monde.

---

# 33. Dieu observateur

Le joueur possède un rôle particulier.

Il peut :

```text
Observe
Focus
Zoom
Inspect
Follow
Intervene
```

Mais l'agent ne doit pas nécessairement savoir qu'il est observé.

Cela dépend du système du **Voile**.

---

# 34. Cohérence Agent

Un agent doit toujours conserver :

```text
Identity consistency
Personality consistency
Memory consistency
Belief consistency
Relationship consistency
Goal consistency
```

Le LLM ne doit jamais être considéré comme la source de vérité.

Genesis reste la source de vérité.

---

# 35. LLM Stateless

Le LLM ne possède pas nécessairement une mémoire permanente.

Chaque requête est reconstruite :

```text
World State
+
Agent State
+
Memory Retrieval
+
Current Event
+
Context Builder
      ↓
LLM
```

Le LLM peut donc être remplacé par un autre modèle sans perdre la civilisation.

---

# 36. Model Router

Genesis pourra utiliser différents modèles.

```text
                 MODEL ROUTER
                      │
        ┌─────────────┼─────────────┐
        │             │             │
      Tiny          Medium         Large
        │             │             │
      Routine       Social        Critical
```

Exemple :

```text
Routine        → local lightweight model
Conversation   → local/remote medium model
Major event    → powerful model
```

---

# 37. Local First

L'architecture doit privilégier le traitement local lorsque possible.

```text
CPU
 ├── World Simulation
 ├── Event Bus
 ├── Scheduler
 ├── Database
 └── AI logic

GPU
 └── Rendering / optional AI

LLM
 ├── Local
 └── Remote optional
```

Genesis ne doit pas dépendre d'un fournisseur LLM unique.

---

# 38. Cache

Les résultats répétitifs doivent pouvoir être réutilisés.

Exemple :

```text
Same context
+
Same decision problem
+
Same model
```

→ résultat potentiellement réutilisable selon le niveau de déterminisme choisi.

---

# 39. Determinism

Genesis doit pouvoir fonctionner avec une seed.

```text
WORLD_SEED
SIMULATION_SEED
AGENT_SEED
EVENT_SEED
```

Cela permet :

- reproduction de bugs ;
- tests ;
- comparaison de simulations ;
- replay ;
- debug.

Les appels LLM devront cependant être traités avec une stratégie spécifique si l'on souhaite une reproductibilité stricte.

---

# 40. Observability

Le moteur doit enregistrer :

```text
LLM calls
context size
latency
tokens
cost
decision
validation
world event
memory changes
```

Cela permettra de répondre à :

> Pourquoi cet agent a-t-il fait ça ?

---

# 41. Agent Decision Trace

Chaque décision importante peut produire :

```text
Decision Trace

Agent:
Aren

Situation:
Mira refuses to speak.

Relevant memories:
#194
#382
#991

Current goal:
Repair relationship.

Decision:
Attempt conversation.

Confidence:
0.73

Result:
Conversation accepted.
```

La trace n'est pas nécessairement visible au joueur.

Elle sert au debug et à l'analyse.

---

# 42. Architecture finale

```text
                         ┌───────────────────┐
                         │    WORLD STATE    │
                         └─────────┬─────────┘
                                   │
                 ┌─────────────────┴─────────────────┐
                 │                                   │
                 ▼                                   ▼
          ┌─────────────┐                    ┌───────────────┐
          │  EVENT BUS  │                    │ MEMORY ENGINE │
          └──────┬──────┘                    └───────┬───────┘
                 │                                   │
                 ▼                                   │
          ┌─────────────┐                            │
          │  SCHEDULER  │                            │
          └──────┬──────┘                            │
                 │                                   │
                 └────────────────┬──────────────────┘
                                  ▼
                         ┌─────────────────┐
                         │ CONTEXT BUILDER │
                         └────────┬────────┘
                                  │
                                  ▼
                             ┌─────────┐
                             │  AGENT  │
                             └────┬────┘
                                  │
                                  ▼
                           ┌─────────────┐
                           │ MODEL ROUTER│
                           └──────┬──────┘
                                  │
                    ┌─────────────┼─────────────┐
                    ▼             ▼             ▼
                 Tiny LLM     Medium LLM    Large LLM
                    │             │             │
                    └─────────────┼─────────────┘
                                  ▼
                           STRUCTURED RESULT
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

# 43. Principe absolu

Genesis doit respecter cette règle :

> **Le monde n'est jamais contenu dans le LLM.**

Le LLM n'en reçoit qu'une représentation partielle.

Inversement :

> **Le LLM ne doit jamais être la source de vérité du monde.**

Il propose.

Genesis valide.

---

# 44. Résumé architectural

```text
WORLD STATE
    ↓
What is actually true?

MEMORY ENGINE
    ↓
What does this agent remember?

RAG
    ↓
What memories are relevant?

CONTEXT BUILDER
    ↓
What does the agent need to know right now?

AGENT
    ↓
Who am I?

LLM
    ↓
What would I decide?

VALIDATOR
    ↓
Is that decision possible?

EVENT BUS
    ↓
When should the consequence happen?

WORLD STATE
    ↓
What changed?
```

---

# 45. Philosophie

Genesis ne doit pas créer :

> **des milliers de chatbots.**

Genesis doit créer :

> **un monde simulé contenant des milliers d'entités, dont certaines possèdent une intelligence générative.**

La majorité de la vie est simulée par les règles du monde.

L'intelligence coûteuse intervient uniquement lorsque la situation le justifie.

C'est ce qui permettra d'obtenir simultanément :

```text
Scale
+
Coherence
+
Emergence
+
Personality
+
Memory
+
Performance
+
LLM intelligence
```

sans transformer chaque seconde de simulation en appel API.

---

# 46. Extension future : intelligence collective

À terme, les mêmes principes pourront s'appliquer à des niveaux supérieurs.

```text
Individual
    ↓
Family
    ↓
Group
    ↓
Community
    ↓
City
    ↓
Civilization
```

Chaque niveau peut posséder :

- mémoire collective ;
- croyances ;
- objectifs ;
- identité ;
- événements ;
- représentation du monde.

Une civilisation pourra donc progressivement développer quelque chose qui ressemble à une **mémoire historique collective**.

Cela permettra notamment l'émergence de :

```text
Myths
Religions
National identities
Historical narratives
Political ideologies
Scientific schools
Cultural traditions
```

sans les écrire explicitement à l'avance.

---

# 47. Vision finale

Le système doit donner l'impression que chaque entité dit :

> **"Je ne connais pas le monde. Je connais seulement ce que j'en ai vécu."**

Et que Genesis, lui, connaît la totalité :

```text
THE WORLD
```

Le joueur, en tant que Dieu, possède une troisième position :

```text
WORLD
  │
  ├── Truth
  │
  └── What agents believe
          │
          ▼
       NODYX
          │
          ▼
        PLAYER
```

Le joueur peut donc observer les différences entre :

**ce qui s'est réellement passé**

et

**ce que les habitants pensent qu'il s'est passé.**

C'est probablement l'une des mécaniques les plus puissantes de Genesis.