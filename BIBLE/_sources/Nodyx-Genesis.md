# Nodyx-Genesis
## Architecture & Repository Specification

> **Construire un monde vivant capable d'évoluer, de se souvenir, de créer sa propre culture et d'interagir avec le monde extérieur.**

---

# 1. Vision

Nodyx-Genesis est le moteur de simulation d'un monde autonome évolutif intégré à l'écosystème Nodyx.

L'objectif n'est pas simplement de simuler des agents artificiels.

L'objectif est de construire progressivement un **univers vivant** :

- des individus apparaissent ;
- ils évoluent biologiquement ;
- ils développent des comportements ;
- ils communiquent ;
- ils forment des groupes ;
- des cultures émergent ;
- des civilisations apparaissent ;
- des connaissances sont transmises ;
- des langues et traditions peuvent évoluer ;
- des conflits et alliances apparaissent ;
- des individus deviennent historiquement importants ;
- des artefacts culturels sont créés ;
- les civilisations utilisent les outils numériques de Nodyx ;
- les humains peuvent observer et, dans certaines conditions, interagir avec elles.

Le monde ne doit donc pas uniquement exister dans la mémoire du moteur.

**Il doit laisser des traces dans le monde extérieur.**

---

# 2. Le principe fondamental

Genesis possède son propre état du monde.

Nodyx possède ses propres services.

Genesis ne doit pas réimplémenter les fonctionnalités de Nodyx.

Il doit pouvoir **utiliser Nodyx comme une infrastructure numérique**.

```text
GENESIS
   │
   │ Intent
   ▼
VALIDATION
   │
   │ Tool Call
   ▼
NODYX API
   │
   ├── Forum
   ├── Chat
   ├── Vocal
   ├── Wiki
   ├── Canvas
   ├── Calendar
   ├── Games
   └── autres outils futurs
```

Ainsi, l'évolution de Nodyx peut fournir progressivement de nouvelles capacités aux civilisations de Genesis.

---

# 3. Un univers connecté

Genesis doit être conçu comme un système pouvant communiquer avec son environnement numérique.

```text
                  INTERNET
                     │
          ┌──────────┴──────────┐
          │                     │
        HUMANS                SERVICES
          │                     │
          └──────────┬──────────┘
                     │
                    NODYX
                     │
             NODYX-GENESIS
                     │
             ┌───────┴───────┐
             │               │
          WORLD            AGENTS
          STATE               │
             │               │
             └───────┬───────┘
                     │
                  CULTURE
                     │
                CIVILIZATION
                     │
               DIGITAL ARTIFACTS
                     │
                     ▼
                    NODYX
```

La frontière entre le monde simulé et le monde numérique extérieur devient ainsi une **interface**, plutôt qu'une séparation absolue.

---

# 4. Les agents comme habitants

Les agents ne sont pas de simples NPC.

Chaque agent peut progressivement posséder :

- une identité ;
- un génome ;
- des caractéristiques biologiques ;
- une personnalité ;
- des besoins ;
- des objectifs ;
- une mémoire ;
- des relations ;
- des connaissances ;
- des croyances ;
- une histoire personnelle ;
- une appartenance sociale ;
- une culture ;
- une profession ;
- un statut ;
- une influence ;
- des possessions ;
- des créations ;
- une réputation.

Un agent peut donc passer de :

```text
organisme
   ↓
individu
   ↓
être social
   ↓
membre d'une culture
   ↓
acteur historique
```

---

# 5. Le monde possède deux histoires

Genesis distingue :

### Objective History

Ce qui s'est réellement produit dans le World State.

### Subjective History

Ce que les habitants pensent qu'il s'est produit.

Ces deux histoires peuvent diverger.

```text
OBJECTIVE HISTORY
       │
       ├── événement réel
       │
       ▼
INDIVIDUAL MEMORY
       │
       ▼
SOCIAL TRANSMISSION
       │
       ▼
COLLECTIVE MEMORY
       │
       ▼
CULTURE / MYTH / RELIGION
```

Une civilisation peut donc posséder une histoire différente de la réalité objective.

Cette divergence constitue une source majeure de profondeur narrative.

---

# 6. Nodyx comme infrastructure culturelle

Les habitants de Genesis peuvent progressivement utiliser les outils disponibles sur Nodyx.

Exemples :

### Wiki

Un érudit peut créer :

> "Histoire de la guerre de Velkar"

### Canvas

Un scientifique peut créer un schéma.

Un artiste peut créer une œuvre.

Un stratège peut annoter une carte.

### Calendrier

Une civilisation peut créer son propre calendrier.

Les fêtes, guerres, cérémonies et événements historiques peuvent y être inscrits.

### Jeux

Les habitants peuvent créer et pratiquer des jeux.

### Forum

Une communauté peut débattre publiquement.

### Chat

Les agents peuvent communiquer de manière plus immédiate.

### Vocal

À terme, certaines interactions peuvent devenir vocales.

Ainsi :

> **Les habitants de Genesis ne vivent pas seulement dans Genesis. Ils utilisent Nodyx pour construire leur civilisation.**

---

# 7. Les artefacts deviennent persistants

Lorsqu'un agent crée quelque chose via Nodyx, l'objet peut devenir un artefact du monde.

```text
Agent
  │
  ▼
Intent
  │
  ▼
Tool Call
  │
  ▼
Validation
  │
  ▼
Nodyx Artifact
  │
  ├── auteur
  ├── civilisation
  ├── date
  ├── contexte
  ├── relations
  └── événements associés
```

Un artefact peut ensuite influencer le monde.

Exemple :

```text
Scientifique découvre une théorie
        ↓
crée une page Wiki
        ↓
d'autres agents la consultent
        ↓
la connaissance se diffuse
        ↓
une nouvelle technologie apparaît
        ↓
la civilisation évolue
```

La création numérique devient donc une **mécanique du monde**.

---

# 8. Le monde comme système de rétroaction

Genesis doit être pensé comme une boucle.

```text
SIMULATION
    ↓
ÉVÉNEMENTS
    ↓
AGENTS
    ↓
INTERACTIONS
    ↓
CULTURE
    ↓
CRÉATIONS
    ↓
NODYX
    ↓
INTERACTIONS HUMAINES
    ↓
NOUVEAUX ÉVÉNEMENTS
    ↓
SIMULATION
```

Cette boucle est l'une des idées fondamentales du projet.

---

# 9. Interaction humaine

Les humains peuvent devenir des observateurs.

Mais ils peuvent également devenir, dans les limites définies par Genesis, des participants à l'écosystème.

Un humain peut :

- consulter les archives ;
- lire les discussions ;
- observer les civilisations ;
- consulter les cartes ;
- découvrir les œuvres ;
- participer à certains espaces ;
- poser des questions ;
- interagir avec certains habitants.

Les interactions humaines doivent cependant respecter les règles du monde.

L'humain ne doit pas disposer d'un accès arbitraire permettant de casser la simulation.

---

# 10. Le moteur de validation

Toute action importante passe par Genesis.

```text
Agent Intent
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
Execution
```

### Physique

L'action est-elle possible ?

### Sociale

Quelles conséquences sociales peut-elle provoquer ?

### Narrative

L'action est-elle cohérente avec l'agent ?

Une incohérence narrative ne doit généralement pas interdire l'action.

Elle peut simplement augmenter son coût, sa difficulté ou ses conséquences.

---

# 11. Scalabilité

Genesis doit pouvoir simuler beaucoup plus d'individus qu'il n'est possible d'appeler individuellement auprès d'un LLM.

Le système utilise donc une **Differential Simulation**.

```text
10 000 agents
      │
      ├── 9 700 → simulation agrégée
      │
      ├── 250 → simulation comportementale détaillée
      │
      └── 50 → simulation LLM
```

L'importance est dynamique.

Un individu anonyme peut devenir important.

Un personnage historique peut redevenir secondaire.

Le moteur doit pouvoir changer continuellement le niveau de simulation.

---

# 12. LLM comme couche cognitive

Le LLM n'est pas le moteur physique.

Il ne possède pas l'autorité sur le World State.

Il propose.

Genesis décide.

```text
WORLD STATE
     │
     ▼
CONTEXT BUILDER
     │
     ▼
LLM
     │
     ▼
STRUCTURED OUTPUT
     │
     ▼
VALIDATOR
     │
     ▼
WORLD STATE UPDATE
```

Le LLM ne peut donc pas :

- créer arbitrairement une ressource ;
- téléporter un individu ;
- modifier directement une position impossible ;
- créer un événement physique inexistant ;
- contourner les règles du monde.

---

# 13. Event Bus

Toutes les actions importantes doivent pouvoir devenir des événements.

Exemples :

```text
Birth
Death
Move
Eat
Reproduce
Discover
Meet
Fight
Trade
Talk
CreateArtifact
CreateWikiPage
CreateCanvas
CreateGame
JoinGroup
LeaveGroup
CulturalTransmission
PoliticalChange
War
Peace
```

Les événements peuvent ensuite déclencher d'autres systèmes.

---

# 14. Protection contre les cascades

Une civilisation complexe peut produire énormément d'événements.

Genesis doit donc contrôler les cascades.

Mécanismes :

- `cascade_depth`
- `MAX_EVENTS_PER_TICK`
- priorités ;
- files d'attente ;
- agrégation ;
- report d'événements ;
- compression ;
- traitement différentiel.

**Le monde ne doit jamais pouvoir s'effondrer parce qu'un événement en a généré mille autres.**

---

# 15. Architecture du repository

Le repository initial :

```text
Nodyx-Genesis/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── ARCHITECTURE.md
├── ROADMAP.md
│
├── docs/
│   ├── vision/
│   ├── architecture/
│   ├── simulation/
│   ├── biology/
│   ├── evolution/
│   ├── agents/
│   ├── memory/
│   ├── culture/
│   ├── society/
│   ├── civilization/
│   ├── history/
│   ├── llm/
│   ├── scaling/
│   └── nodyx-integration/
│
├── genesis/
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
│   └── validation/
│
├── integrations/
│   └── nodyx/
│
├── tests/
│
└── tools/
```

Cette structure doit pouvoir évoluer sans enfermer le projet dans une architecture prématurément complexe.

---

# 16. Contrats fondamentaux

Avant de développer les systèmes avancés, les structures fondamentales doivent être définies.

```text
WorldState
Agent
Genome
Cell
Resource
Event
Memory
Relationship
Belief
Culture
Civilization
Artifact
Intent
ToolCall
ValidationResult
```

Ces structures constituent le contrat entre les différents systèmes.

---

# 17. Philosophie de développement

Genesis doit être construit progressivement.

### Genesis 0.0.1

Deux organismes.

Pas de LLM.

Pas de civilisation.

Pas de réseau.

Uniquement :

- World State ;
- tick ;
- déplacement ;
- nourriture ;
- énergie ;
- reproduction ;
- mutation ;
- mort ;
- persistance.

### Genesis 0.0.2

Évolution.

### Genesis 0.0.3

Mémoire et personnalité.

### Genesis 0.0.4

Communication.

### Genesis 0.0.5

Société.

### Genesis 0.0.6

Civilisation.

### Versions suivantes

Intégration profonde avec Nodyx.

---

# 18. Principe absolu

Le projet doit rester capable de fonctionner sans LLM.

Le LLM apporte :

- cognition ;
- langage ;
- interprétation ;
- créativité ;
- raisonnement social ;
- narration.

Mais les lois fondamentales du monde appartiennent à Genesis.

```text
GENESIS = réalité
LLM     = cognition
NODYX   = environnement numérique
HUMANS  = visiteurs / participants
```

---

# 19. Vision finale

Le projet commence comme une simulation biologique.

Puis devient une simulation sociale.

Puis une simulation culturelle.

Puis une simulation civilisationnelle.

Puis un monde connecté.

À terme, l'objectif est de pouvoir observer quelque chose comme :

```text
Une civilisation naît.

Elle développe une langue.

Elle découvre une technologie.

Elle crée un calendrier.

Elle construit une religion.

Elle écrit son histoire.

Elle produit ses artistes.

Elle invente ses jeux.

Elle débat sur son forum.

Elle archive ses connaissances.

Elle dessine ses cartes.

Elle crée ses mythes.

Des humains découvrent cette civilisation.

Ils observent ses habitants.

Certains interagissent avec eux.

Ces interactions deviennent des événements.

Les événements modifient la civilisation.

La civilisation continue d'évoluer.
```

**Le monde n'est plus uniquement simulé.**

**Il produit sa propre histoire.**

Et cette histoire devient visible à l'extérieur de Genesis.

---

# 20. Manifeste

> **Nodyx-Genesis n'a pas pour objectif de créer des personnages qui font semblant de vivre.**
>
> **L'objectif est de créer les conditions permettant à un monde de produire spontanément des individus, des sociétés, des cultures et une histoire.**
>
> **Nodyx fournit l'espace numérique.**
>
> **Genesis fournit les lois du monde.**
>
> **Les agents produisent la culture.**
>
> **Les humains deviennent les témoins — et éventuellement les participants — de cette histoire.**
>
> **Le résultat attendu n'est pas une histoire écrite à l'avance.**
>
> **C'est une histoire qui n'était pas écrite avant d'être vécue.**