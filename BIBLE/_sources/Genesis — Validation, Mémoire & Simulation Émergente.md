# Genesis — Validation, Mémoire & Simulation Émergente
## Spécification technique v0.2

---

# 1. Principe général

Genesis doit distinguer quatre niveaux :

```text
WORLD TRUTH
     │
     ├── Physical Reality
     │
     ├── Social Reality
     │
     └── Historical Reality
             │
             ▼
       AGENT PERCEPTION
             │
       ┌─────┴─────┐
       ▼           ▼
    MEMORY       BELIEF
       │           │
       └─────┬─────┘
             ▼
          DECISION
             │
             ▼
        CONSEQUENCES
             │
             ▼
         WORLD EVENT
```

Le monde possède une réalité objective.

Les agents possèdent une perception subjective.

Les conséquences sont calculées par Genesis.

---

# 2. Validation en trois couches

Chaque décision importante passe par :

```text
1. Physical / Logical Validation
2. Social Validation
3. Narrative Coherence
```

Ces couches n'ont pas le même rôle.

---

# 3. Layer 1 — Physical / Logical Validation

**Obligatoire.**

Elle répond à :

> Est-ce possible ?

Exemples :

```text
Agent vivant ?
Cible existante ?
Distance compatible ?
Objet disponible ?
Ressources suffisantes ?
Technologie connue ?
Action physiquement réalisable ?
```

Cette couche est déterministe autant que possible.

---

# 4. Layer 2 — Social Validation

Elle répond à :

> Qu'est-ce que cette action risque de provoquer dans la société ?

Elle ne bloque généralement pas l'action.

Elle produit plutôt un ensemble de conséquences potentielles.

---

# 5. Social Consequence Model

Une action produit des conséquences sociales pondérées.

Exemple :

```text
Action:
Trahir un allié
```

Genesis calcule :

```text
Reputation Loss       0.85
Group Exclusion       0.70
Resource Loss         0.50
Conflict              0.30
Arrest                0.20
Retaliation           0.45
```

Mais ces valeurs ne sont pas des probabilités arbitraires.

Elles sont calculées à partir du World State.

---

# 6. Social Risk Factors

Les conséquences dépendent notamment de :

```text
Relationship
Trust
Social Status
Group Laws
Culture
Religion
Political System
Historical Context
Witnesses
Evidence
Previous Reputation
Power Balance
```

Exemple :

```text
Trahir un roi

Agent :
noble puissant

Witnesses :
0

Evidence :
faible

Political system :
authoritarian

Result :
low immediate risk
high long-term political risk
```

La même action dans une autre civilisation peut produire un résultat complètement différent.

---

# 7. Social Consequence Resolution

Le système peut fonctionner ainsi :

```text
ACTION
 ↓
SOCIAL ANALYSIS
 ↓
POTENTIAL CONSEQUENCES
 ↓
CONTEXTUAL WEIGHT
 ↓
WORLD RESOLUTION
```

Le résultat final devient un événement du monde.

---

# 8. Exemple

```text
Aren vole du pain.
```

Dans une société pauvre :

```text
Food scarcity = HIGH
Law severity = LOW
Community solidarity = HIGH
```

Résultat possible :

```text
Neighbor helps Aren
```

Dans une société autoritaire :

```text
Food scarcity = MEDIUM
Law severity = HIGH
Surveillance = HIGH
```

Résultat possible :

```text
Arrest
```

Même action.

Deux mondes différents.

---

# 9. Layer 3 — Narrative Coherence

Cette couche répond à :

> Est-ce cohérent avec ce personnage ?

Elle produit :

```text
coherence_score
```

entre :

```text
0.0 → 1.0
```

---

# 10. Calcul de cohérence

Le score peut prendre en compte :

```text
Personality
Goals
Beliefs
Memory
Relationships
Past actions
Current emotions
Current needs
Social pressure
```

Exemple :

```text
Aren

Compassion       0.91
Aggression       0.12
Family Loyalty   0.95

Action:
Kill his brother

Coherence = 0.07
```

---

# 11. Personality Rigidity

Tous les personnages ne réagissent pas de la même manière à une action incohérente.

Chaque agent possède une caractéristique :

```text
behavioral_flexibility
```

Exemple :

```text
Rigid agent
    flexibility = 0.15

Impulsive agent
    flexibility = 0.85
```

---

# 12. Dynamic Coherence Threshold

Les seuils sont donc personnalisés.

Conceptuellement :

```text
effective_threshold =
base_threshold × personality_rigidity
```

Mais une faible cohérence ne signifie jamais :

```text
ACTION = IMPOSSIBLE
```

Elle signifie :

```text
ACTION = UNEXPECTED
```

---

# 13. Unexpected Behavior

Une action très incohérente peut devenir intéressante.

Exemple :

```text
Aren est pacifiste.

Coherence = 0.18

Mais :
- son enfant est menacé
- il est paniqué
- il vient d'apprendre une trahison
```

Genesis peut laisser l'action se produire.

Cela crée :

```text
Character Break
```

qui peut devenir un événement historique.

---

# 14. Narrative Coherence comme pression

La cohérence devient donc une force, pas une règle absolue.

```text
Personality
     ↓
Behavioral Pressure
     ↓
Decision Probability
```

Un agent peut toujours sortir de son comportement habituel.

Mais cela doit avoir une raison.

---

# 15. Memory Anchoring

Chaque souvenir important peut conserver :

```text
memory_id
world_event_reference
subjective_content
confidence
emotional_weight
```

Exemple :

```text
MEMORY #291

Subjective:
"Les soldats ont brûlé notre village."

World Event:
EVENT #819

Objective:
Accidental fire during battle.

Confidence:
0.72
```

---

# 16. Memory Divergence

Genesis peut calculer :

```text
memory_divergence
```

entre :

```text
World Event
      ↕
Subjective Memory
```

Ce score ne sert pas à corriger automatiquement la mémoire.

Il sert à mesurer sa transformation.

---

# 17. Types de divergence

```text
LOW
→ mémoire fidèle

MEDIUM
→ mémoire déformée

HIGH
→ interprétation fortement subjective

EXTREME
→ nouvelle construction narrative
```

---

# 18. Memory Detachment

Lorsqu'une mémoire devient trop éloignée de son événement original, elle peut devenir un :

```text
Detached Memory
```

Elle conserve :

```text
origin_event
```

mais son contenu est désormais principalement subjectif.

Exemple :

```text
World Event:
Sécheresse de trois mois.

Agent Memory:
"Les dieux ont puni notre peuple."
```

L'ancrage reste :

```text
EVENT #18291
```

mais la signification devient culturelle.

---

# 19. Importance dynamique des agents

L'importance n'est jamais permanente.

Elle évolue.

```text
Agent Importance
       ↓
recalculated periodically
```

Exemple :

```text
social_status
influence
knowledge
relationships
uniqueness
recent_events
historical_significance
player_interest
```

---

# 20. Importance Decay

Un agent qui cesse d'avoir un rôle important peut progressivement redescendre.

```text
Important
   ↓
Normal
   ↓
Background
```

Inversement :

```text
Unknown
   ↓
Interesting event
   ↓
Influential
   ↓
Historical figure
```

---

# 21. Importance Recalculation

Le système peut recalculer périodiquement :

```text
every N ticks
```

et immédiatement après certains événements :

```text
war
discovery
political change
birth of leader
death of leader
major invention
religious event
```

---

# 22. Differential Simulation

La simulation est divisée entre :

```text
BACKGROUND POPULATION
```

et

```text
FOCUS AGENTS
```

---

# 23. Background Population

Les individus ordinaires peuvent être simulés statistiquement.

```text
10000 agents
 ↓
population model
 ↓
aggregate behavior
```

Mais ils restent des individus dans le World State.

Ils peuvent être promus.

---

# 24. Focus Agents

Les agents importants bénéficient de :

```text
Detailed behavior
Memory
Relationships
LLM
Conversation
Historical tracking
```

Le nombre de Focus Agents est limité par le budget disponible.

---

# 25. Promotion System

Un agent peut être promu :

```text
BACKGROUND
     ↓
ACTIVE
     ↓
IMPORTANT
     ↓
HISTORICAL
```

Exemple :

```text
Unknown farmer
     ↓
discovers metalworking technique
     ↓
knowledge ↑
     ↓
influence ↑
     ↓
IMPORTANT
```

---

# 26. Demotion

Inversement :

```text
HISTORICAL
     ↓
IMPORTANT
     ↓
ACTIVE
     ↓
BACKGROUND
```

Mais certains événements peuvent créer une **importance historique permanente**.

Exemple :

```text
Inventor of agriculture
Founder of civilization
Author of major religion
First astronaut
Leader of revolution
```

---

# 27. Structured Output

Le LLM doit produire une sortie structurée.

Le texte humain et les conséquences doivent être séparés.

```text
LLM OUTPUT
│
├── Dialogue
│
└── Simulation Result
      ├── beliefs
      ├── memories
      ├── emotions
      ├── relationships
      ├── intentions
      └── knowledge
```

---

# 28. Output Validation Pipeline

```text
LLM
 ↓
JSON Parser
 ↓
Schema Validation
 ↓
Semantic Validation
 ↓
World Validation
 ↓
Apply
```

---

# 29. Fallback System

Si le LLM échoue :

```text
1. Parse
2. Repair
3. Regenerate
4. Fallback AI
```

Plus précisément :

```text
INVALID JSON
     ↓
REPAIR
     ↓
still invalid?
     ↓
REGENERATE
     ↓
still invalid?
     ↓
BEHAVIORAL FALLBACK
```

Le monde ne doit jamais dépendre de la validité d'une réponse LLM.

---

# 30. Logging

Chaque erreur doit être enregistrée :

```text
timestamp
agent
event
model
prompt_hash
response
validation_error
fallback_used
```

Cela permettra d'améliorer progressivement les prompts et les modèles.

---

# 31. Collective Memory

La mémoire collective devient un système indépendant.

```text
INDIVIDUAL MEMORY
        │
        ▼
SOCIAL TRANSMISSION
        │
        ▼
COLLECTIVE MEMORY
```

---

# 32. Collective Memory Entities

Chaque niveau social peut posséder une mémoire.

```text
Family
Group
Village
City
Nation
Civilization
Religion
```

---

# 33. Collective Memory Structure

```text
CollectiveMemory
├── HistoricalEvents
├── Myths
├── Traditions
├── Heroes
├── Traumas
├── Victories
├── Rituals
├── Laws
└── SharedBeliefs
```

---

# 34. Transmission

Une mémoire collective peut être transmise par :

```text
Parents
Education
Religion
Stories
Songs
Books
Rituals
Political institutions
Monuments
Art
```

---

# 35. Collective Memory Mutation

Une mémoire collective peut elle aussi évoluer.

```text
Historical Event
       ↓
Witness Accounts
       ↓
Stories
       ↓
Repeated Transmission
       ↓
Cultural Narrative
       ↓
Myth
```

Exemple :

```text
EVENT

Un chef survit à une bataille.

↓

MEMORY

"Le chef a été courageux."

↓

LEGEND

"Le chef était invincible."

↓

MYTH

"Les dieux protégeaient le chef."
```

---

# 36. Individual ↔ Collective Memory

Les deux systèmes doivent communiquer.

```text
Individual
   │
   ├── tells story
   │
   ▼
Collective Memory
   │
   ├── modifies cultural narrative
   │
   ▼
Other Individuals
   │
   └── internalize belief
```

Une boucle de rétroaction apparaît.

---

# 37. Cultural Feedback Loop

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
        ↓
NEW EXPERIENCES
```

C'est potentiellement l'un des mécanismes d'émergence les plus importants de Genesis.

---

# 38. Religion émergente

La religion ne doit pas être une table :

```text
religion = christianity
```

mais un processus.

```text
EVENT
 ↓
INTERPRETATION
 ↓
BELIEF
 ↓
SHARED BELIEF
 ↓
RITUAL
 ↓
TRADITION
 ↓
INSTITUTION
 ↓
RELIGION
```

---

# 39. Politique émergente

Même principe.

```text
Leadership
 ↓
Influence
 ↓
Coalition
 ↓
Institution
 ↓
Government
 ↓
Political Ideology
```

---

# 40. Technologie émergente

La technologie repose sur :

```text
Knowledge
+
Resources
+
Observation
+
Experimentation
+
Transmission
+
Accumulation
```

Une civilisation ne débloque donc pas simplement :

```text
TECHNOLOGY_LEVEL = 5
```

Elle construit progressivement ses connaissances.

---

# 41. Economie émergente

Même philosophie :

```text
Resources
 ↓
Scarcity
 ↓
Exchange
 ↓
Specialization
 ↓
Trade
 ↓
Markets
 ↓
Currency
 ↓
Institutions
```

---

# 42. World History

Genesis doit conserver une chronologie objective.

```text
WORLD HISTORY
│
├── Geological Events
├── Biological Events
├── Civilization Events
├── Political Events
├── Wars
├── Discoveries
├── Technologies
├── Religious Events
└── Cultural Events
```

Cette histoire constitue la source de vérité.

---

# 43. Double Histoire

Il existe donc deux histoires :

```text
OBJECTIVE HISTORY
        │
        │
        ▼
SUBJECTIVE HISTORY
```

### Objective History

Ce qui s'est réellement produit.

### Subjective History

Ce que les civilisations racontent.

Les deux peuvent diverger.

---

# 44. Exemple historique

```text
OBJECTIVE:

Year 1821
A volcano erupts.
3 villages are destroyed.
```

Civilization A :

```text
"The mountain gods were angry."
```

Civilization B :

```text
"The eruption was natural."
```

Civilization C :

```text
"Our enemy caused the disaster."
```

Un seul événement.

Trois interprétations.

---

# 45. Architecture mémoire finale

```text
                    WORLD HISTORY
                          │
                          ▼
                  ┌───────────────┐
                  │ WORLD EVENTS  │
                  └───────┬───────┘
                          │
             ┌────────────┴────────────┐
             ▼                         ▼
      INDIVIDUAL MEMORY        COLLECTIVE MEMORY
             │                         │
             ▼                         ▼
          BELIEFS                  CULTURE
             │                         │
             └────────────┬────────────┘
                          ▼
                       AGENT
                          │
                          ▼
                       DECISION
                          │
                          ▼
                       EVENT
                          │
                          ▼
                    WORLD HISTORY
```

---

# 46. La boucle fondamentale

Genesis devient finalement une boucle :

```text
WORLD
 ↓
EXPERIENCE
 ↓
MEMORY
 ↓
BELIEF
 ↓
DECISION
 ↓
ACTION
 ↓
CONSEQUENCE
 ↓
WORLD
```

Et au niveau collectif :

```text
WORLD
 ↓
INDIVIDUALS
 ↓
STORIES
 ↓
COLLECTIVE MEMORY
 ↓
CULTURE
 ↓
INSTITUTIONS
 ↓
CIVILIZATION
 ↓
WORLD
```

---

# 47. Principe de non-triche

Genesis ne doit jamais modifier artificiellement le monde pour créer une "bonne histoire".

Il doit seulement :

```text
simulate
observe
prioritize
record
```

Les événements historiques doivent émerger des règles.

---

# 48. Principe de surprise

Le développeur doit pouvoir lancer une simulation et découvrir :

```text
"Pourquoi cette civilisation est-elle devenue comme ça ?"
```

Puis remonter :

```text
Civilization
 ↓
Institutions
 ↓
Culture
 ↓
Collective Memory
 ↓
Individual
 ↓
Decision
 ↓
Original Event
```

C'est la **traçabilité causale**.

---

# 49. Causal Graph

Chaque événement important peut référencer son origine.

```text
EVENT_100
   ↓
EVENT_142
   ↓
EVENT_981
   ↓
EVENT_1204
   ↓
WAR
   ↓
CIVILIZATION_COLLAPSE
```

Genesis pourra alors reconstruire une chaîne causale.

---

# 50. Vision finale

Le système ne doit pas simplement raconter une histoire.

Il doit pouvoir expliquer :

> **Pourquoi cette histoire est arrivée.**

Et encore mieux :

> **Pourquoi ses habitants pensent qu'elle est arrivée.**

Ces deux réponses peuvent être différentes.

C'est cette divergence entre **vérité, mémoire, croyance et conséquence** qui doit donner à Genesis son caractère émergent.