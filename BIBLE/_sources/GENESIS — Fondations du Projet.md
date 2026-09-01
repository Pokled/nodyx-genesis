# GENESIS — Fondations du Projet

> **Deux molécules. Un monde. Des milliards d'histoires possibles.**

---

## 1. Présentation

**Genesis** est un projet de simulation de vie artificielle persistante.

L'objectif n'est pas de créer un jeu traditionnel avec une histoire écrite à l'avance.

L'objectif est de créer un **monde artificiel capable de produire sa propre histoire**.

Le monde commence avec des entités extrêmement simples.

Elles évoluent.

Elles se reproduisent.

Elles communiquent.

Elles apprennent.

Elles forment des groupes.

Puis des sociétés.

Puis des civilisations.

À terme peuvent apparaître :

- cultures ;
- langues ;
- religions ;
- économies ;
- gouvernements ;
- géopolitique ;
- guerres ;
- sciences ;
- technologies ;
- exploration spatiale ;
- nouvelles formes de vie ;
- civilisations inconnues.

Le monde doit continuer à évoluer même lorsqu'aucun humain ne l'observe.

---

# 2. Vision

Genesis doit donner au joueur l'impression non pas de jouer à un jeu...

mais de **regarder un univers artificiel naître**.

Le joueur n'est pas nécessairement le héros.

Il est l'**Observateur**.

Il regarde un monde qui possède sa propre temporalité, ses propres habitants et sa propre histoire.

La question centrale devient :

> **"Qu'est-ce qu'ils ont fait pendant que je n'étais pas là ?"**

---

# 3. Genesis n'est pas un jeu classique

Genesis ne repose pas sur :

- une campagne ;
- des niveaux ;
- une histoire linéaire ;
- des quêtes prédéfinies ;
- des PNJ scénarisés ;
- une fin obligatoire.

Genesis repose sur :

- des règles ;
- des systèmes ;
- des contraintes ;
- des interactions ;
- de la simulation ;
- de l'émergence.

Nous définissons les **possibilités**.

Le monde détermine les **événements**.

---

# 4. Principe fondamental : l'émergence

> **Nous ne devons pas écrire les civilisations.**
>
> **Nous devons créer les conditions dans lesquelles elles peuvent apparaître.**

Nous ne devons pas décider à l'avance :

- quelle religion apparaîtra ;
- quelle civilisation dominera ;
- quelle guerre aura lieu ;
- quel individu deviendra célèbre ;
- quelle technologie sera découverte ;
- quelle langue deviendra dominante.

Nous fournissons les mécanismes.

La simulation produit l'histoire.

---

# 5. Architecture générale

Genesis doit être conçu comme un système indépendant de son interface graphique.

```text
                         GENESIS
                            │
                    Simulation Core
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
          ▼                 ▼                 ▼
       Database            API            Event Bus
          │                 │                 │
          └─────────────────┼─────────────────┘
                            │
                 ┌──────────┴──────────┐
                 │                     │
                 ▼                     ▼
               GODOT                 NODYX
                 👁️                    🌐
             Visualisation         Communauté
```

---

# 6. Séparation Simulation / Visualisation

Cette séparation est fondamentale.

**Godot ne doit pas être Genesis.**

Godot est une fenêtre permettant d'observer Genesis.

La simulation doit pouvoir fonctionner sans Godot.

Ainsi :

```text
Genesis Server
     │
     ├── Simulation
     ├── Database
     ├── API
     └── Event Bus
          │
          ├── Godot
          ├── Web
          ├── Nodyx
          └── CLI
```

Si Godot est fermé :

> **Le monde continue.**

---

# 7. Godot

Godot constitue l'interface visuelle principale.

Il pourra permettre :

- visualisation du monde ;
- caméra libre ;
- observation des organismes ;
- cartes ;
- villes ;
- civilisations ;
- graphiques ;
- événements ;
- historique ;
- exploration ;
- visualisation temporelle.

Godot ne doit cependant pas contenir les règles fondamentales de simulation.

---

# 8. Nodyx

Nodyx constitue la couche communautaire.

Genesis peut être intégré à Nodyx afin de permettre aux humains de :

- consulter le monde ;
- suivre des civilisations ;
- suivre certains individus ;
- lire les événements ;
- participer aux discussions ;
- observer les évolutions ;
- partager des analyses ;
- construire une communauté autour du monde.

Nodyx devient ainsi la **fenêtre sociale** de Genesis.

---

# 9. Deux populations

Genesis possède deux catégories d'acteurs.

## 9.1 Les humains

Les utilisateurs réels.

Ils observent le monde.

Ils peuvent éventuellement intervenir.

---

## 9.2 Les entités Genesis

Les habitants artificiels.

Ils possèdent progressivement :

- identité ;
- mémoire ;
- personnalité ;
- besoins ;
- relations ;
- connaissances ;
- culture ;
- croyances ;
- objectifs.

Ils peuvent communiquer entre eux.

---

# 10. Une entité n'est pas un chatbot

Principe fondamental.

Une entité Genesis ne doit jamais être :

```text
LLM
+
Prompt
+
Nom
=
Personnage
```

Ce serait trop simpliste.

Une entité doit être un état persistant du monde.

```text
ENTITY
│
├── Genome
├── Body
├── Needs
├── Perception
├── Memory
├── Knowledge
├── Personality
├── Relationships
├── Culture
├── Beliefs
├── Goals
└── Current State
```

Le LLM peut intervenir dans la cognition ou l'expression.

Il ne constitue pas la totalité de l'individu.

---

# 11. Cognition

La cognition suit idéalement une chaîne :

```text
WORLD
  ↓
PERCEPTION
  ↓
MEMORY
  ↓
PERSONALITY
  ↓
GOALS
  ↓
DECISION
  ↓
ACTION
  ↓
LANGUAGE
```

Le monde produit des événements.

L'individu les perçoit.

Il les interprète selon son expérience.

Il décide.

Il agit.

Puis il peut communiquer.

---

# 12. Langage

Le langage doit lui-même pouvoir évoluer.

```text
Signals
   ↓
Symbols
   ↓
Words
   ↓
Syntax
   ↓
Language
   ↓
Writing
```

À terme :

- langues ;
- dialectes ;
- langues mortes ;
- traductions ;
- écritures ;
- évolution linguistique.

---

# 13. Mémoire

La mémoire est essentielle.

Un individu peut mémoriser :

- personnes ;
- lieux ;
- événements ;
- conversations ;
- conflits ;
- découvertes ;
- traumatismes ;
- relations.

Les souvenirs peuvent être :

- incomplets ;
- erronés ;
- déformés ;
- transmis ;
- interprétés.

Ainsi apparaît la mémoire collective.

---

# 14. Société

Les individus peuvent former :

```text
Individu
   ↓
Famille
   ↓
Groupe
   ↓
Clan
   ↓
Tribu
   ↓
Village
   ↓
Ville
   ↓
Civilisation
```

Les structures sociales doivent émerger des besoins et des interactions.

---

# 15. Culture

Les sociétés développent progressivement :

- traditions ;
- coutumes ;
- symboles ;
- musique ;
- art ;
- architecture ;
- vêtements ;
- fêtes ;
- tabous ;
- mythes.

La culture est transmissible.

Elle peut évoluer.

Elle peut se mélanger.

Elle peut disparaître.

---

# 16. Religion

La religion doit pouvoir émerger.

Un événement exceptionnel peut produire :

```text
Événement
    ↓
Interprétation
    ↓
Récit
    ↓
Tradition
    ↓
Mythe
    ↓
Croyance
    ↓
Religion
```

Le joueur peut éventuellement provoquer un événement.

Mais il ne crée pas directement la croyance.

Les habitants lui donnent un sens.

---

# 17. Économie

L'économie peut évoluer de :

```text
Ressources
   ↓
Échange
   ↓
Spécialisation
   ↓
Commerce
   ↓
Marché
   ↓
Monnaie
   ↓
Banque
   ↓
Industrie
   ↓
Économie complexe
```

Les économies peuvent :

- croître ;
- stagner ;
- s'effondrer ;
- produire des inégalités ;
- créer des crises ;
- provoquer des conflits.

---

# 18. Politique

Les groupes peuvent créer différentes structures :

- chefferies ;
- monarchies ;
- républiques ;
- démocraties ;
- oligarchies ;
- théocraties ;
- dictatures ;
- fédérations ;
- systèmes inconnus.

Aucune forme de gouvernement ne doit être imposée comme trajectoire obligatoire.

---

# 19. Géopolitique

Lorsque plusieurs civilisations existent :

- frontières ;
- commerce ;
- migrations ;
- diplomatie ;
- alliances ;
- espionnage ;
- conflits ;
- guerres ;
- traités ;
- colonisation.

Une décision locale peut produire des conséquences à l'échelle mondiale.

---

# 20. Technologie

Les connaissances peuvent produire des technologies.

Exemple :

```text
Feu
 ↓
Outils
 ↓
Agriculture
 ↓
Métallurgie
 ↓
Machines
 ↓
Électricité
 ↓
Informatique
 ↓
IA
 ↓
Exploration spatiale
```

Mais cette progression n'est qu'un exemple.

Une civilisation peut :

- découvrir une technologie plus tôt ;
- en découvrir une autre ;
- perdre une technologie ;
- stagner ;
- bifurquer ;
- produire des technologies inattendues.

---

# 21. Histoire

Genesis conserve l'histoire complète du monde.

Chaque événement important peut être enregistré.

```text
AN 1842
Fondation de Kareth.

AN 1911
Guerre entre Kareth et Arak.

AN 2047
Découverte de l'électricité.

AN 2120
Chute de Kareth.
```

L'histoire est une conséquence de la simulation.

Elle n'est pas écrite à l'avance.

---

# 22. Le monde qui parle

Une caractéristique majeure de Genesis :

> **Le monde doit pouvoir parler de lui-même.**

Les habitants peuvent :

- discuter ;
- débattre ;
- raconter ;
- mentir ;
- propager des rumeurs ;
- faire de la propagande ;
- défendre leurs croyances ;
- parler politique ;
- parler religion ;
- parler économie ;
- parler de leurs voisins.

---

# 23. Genesis Live

Nodyx peut afficher un flux mondial.

```text
🌍 GENESIS LIVE

🔴 Guerre déclarée entre Arak et Kareth

🔬 Nouvelle découverte scientifique

☀ Nouveau mouvement religieux

💰 Marché du cuivre en hausse

🏛 Révolution en cours

👶 Population mondiale : +12 481

🛰 Premier satellite lancé
```

Les événements deviennent ainsi du contenu communautaire.

---

# 24. Forums des civilisations

Certaines civilisations peuvent posséder des espaces communautaires.

```text
FORUM ARAK

📜 Histoire
🏛 Politique
☀ Religion
🔬 Sciences
💰 Commerce
🎭 Culture
⚔ Guerre
```

Les publications peuvent provenir :

- d'individus ;
- de groupes ;
- de dirigeants ;
- d'institutions ;
- de religions ;
- d'événements.

---

# 25. Une conversation doit avoir une cause

Un habitant ne parle pas uniquement parce qu'un LLM lui demande de parler.

Une conversation doit être liée à son état.

Exemple :

```text
Pénurie de sel
      ↓
Eran perd des revenus
      ↓
Stress économique
      ↓
Discussion avec sa guilde
      ↓
Critique du gouvernement
      ↓
Publication
```

Le texte est donc une conséquence.

---

# 26. Rumeurs et vérité

L'information doit pouvoir être imparfaite.

```text
Événement réel
      ↓
Témoin
      ↓
Souvenir
      ↓
Récit
      ↓
Rumeur
      ↓
Légende
      ↓
Mythe
```

La vérité historique et la vérité perçue peuvent être différentes.

---

# 27. Les humains observent

Les utilisateurs peuvent suivre des civilisations.

```text
👁 OBSERVATION

Civilisation :
Arak

Population :
38 421

Âge :
2 841 ans

Technologie :
Bronze avancé

Religion :
Le Cycle

Guerre :
Kareth
```

Ils peuvent suivre :

- cartes ;
- dirigeants ;
- individus ;
- technologies ;
- religions ;
- économie ;
- événements ;
- conversations.

---

# 28. La communauté humaine

Les humains peuvent développer :

- théories ;
- analyses ;
- communautés ;
- archives ;
- classements ;
- chronologies ;
- cartes ;
- histoires ;
- documentaires.

Genesis devient progressivement un univers suivi par sa propre communauté.

---

# 29. La méta-histoire

Genesis possède deux histoires.

### Histoire interne

```text
Civilisations
Guerres
Religions
Technologies
Migrations
Révolutions
```

### Histoire externe

```text
Observateurs
Communauté
Théories
Interventions
Découvertes
```

Les deux histoires peuvent finir par interagir.

---

# 30. L'Observateur

Le joueur est extérieur au monde.

Il peut observer.

Selon les règles du projet, il peut éventuellement intervenir.

```text
OBSERVATEUR
│
├── Observer
├── Accélérer
├── Ralentir
├── Suivre
├── Analyser
└── Intervenir
```

---

# 31. Miracles

Une intervention peut prendre la forme d'un événement.

```text
Pluie
Sécheresse
Mutation
Récolte exceptionnelle
Catastrophe
Ressource
Événement astronomique
```

L'interprétation reste du ressort des habitants.

---

# 32. Le Dieu inconnu

Les habitants ne savent pas nécessairement qu'ils sont observés.

Ils peuvent développer différentes hypothèses :

- hasard ;
- phénomène naturel ;
- simulation ;
- dieu ;
- plusieurs dieux ;
- civilisation supérieure ;
- phénomène incompréhensible.

Certaines civilisations peuvent même chercher scientifiquement à démontrer l'existence de l'Observateur.

---

# 33. Exploration spatiale

Si une civilisation atteint un niveau technologique suffisant :

```text
Planète
 ↓
Orbite
 ↓
Satellites
 ↓
Lune
 ↓
Planètes
 ↓
Système solaire
 ↓
Espace interstellaire
```

Le monde initial devient alors la première étape d'une histoire beaucoup plus vaste.

---

# 34. Entités extérieures

De nouvelles formes de vie peuvent apparaître ou être découvertes.

Elles peuvent posséder :

- biologies différentes ;
- cultures différentes ;
- intelligences différentes ;
- perceptions différentes ;
- technologies différentes ;
- organisations sociales différentes.

Elles ne doivent pas être de simples copies des habitants initiaux.

---

# 35. Premier contact

Une civilisation avancée peut détecter :

```text
UNKNOWN SIGNAL

Origine :
inconnue

Distance :
inconnue

Technologie :
inconnue
```

Puis :

> **FIRST CONTACT**

Une nouvelle époque commence.

---

# 36. Reproductibilité

Les simulations doivent être reproductibles.

Exemple :

```bash
genesis simulate \
    --seed 482931 \
    --years 100000 \
    --agents 10000
```

Une simulation doit pouvoir être identifiée par :

```text
Seed
+
Version du moteur
+
Configuration
+
État initial
```

Cela permet :

- debugging ;
- benchmarks ;
- comparaison ;
- recherche ;
- reproduction d'événements rares.

---

# 37. Expériences

Le développement doit utiliser des expériences isolées.

```text
experiments/

001-two-entities/
002-reproduction/
003-emergent-language/
004-social-groups/
005-first-civilization/
006-religion/
007-economy/
008-war/
```

Chaque expérience doit répondre à une question.

Exemple :

```text
Question :

Deux entités peuvent-elles développer
une relation persistante ?

Hypothèse :

...

Résultat :

...

Observations :

...
```

---

# 38. Infrastructure cible

Le serveur domestique constitue le premier environnement de développement.

Configuration actuelle :

```text
CPU
Intel Xeon E5-2680 v4

14 cœurs / 28 threads

RAM
32 Go DDR4

GPU
AMD Radeon RX 470/480

Stockage
~900 Go utilisables sur la partition système

Architecture
x86_64

NUMA
1 nœud
```

Cette machine constitue largement une base suffisante pour les premiers prototypes.

L'architecture doit néanmoins permettre une évolution vers plusieurs machines si nécessaire.

---

# 39. Architecture du repository

Le projet doit être organisé en monorepo.

```text
genesis/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
├── GENESIS_PRINCIPLES.md
│
├── docs/
│   ├── vision/
│   ├── architecture/
│   ├── design/
│   └── roadmap/
│
├── simulation/
│   ├── core/
│   ├── biology/
│   ├── evolution/
│   ├── agents/
│   ├── society/
│   ├── economy/
│   ├── politics/
│   ├── religion/
│   ├── technology/
│   ├── geopolitics/
│   └── history/
│
├── cognition/
│   ├── perception/
│   ├── memory/
│   ├── personality/
│   ├── reasoning/
│   ├── language/
│   └── llm/
│
├── server/
│   ├── api/
│   ├── events/
│   ├── persistence/
│   ├── realtime/
│   └── auth/
│
├── godot/
│   ├── project.godot
│   ├── scenes/
│   ├── scripts/
│   ├── shaders/
│   ├── assets/
│   └── ui/
│
├── nodyx/
│   ├── integration/
│   ├── adapters/
│   └── events/
│
├── tools/
│   ├── world-generator/
│   ├── simulation-cli/
│   ├── replay/
│   ├── profiling/
│   └── debugging/
│
├── experiments/
│
├── tests/
│   ├── unit/
│   ├── simulation/
│   ├── integration/
│   └── scenarios/
│
├── infrastructure/
│   ├── docker/
│   ├── compose/
│   ├── database/
│   └── monitoring/
│
└── scripts/
    ├── dev.sh
    ├── start.sh
    ├── backup.sh
    └── benchmark.sh
```

---

# 40. Documentation

La documentation doit être considérée comme une partie du projet.

```text
docs/
│
├── vision/
│
│   Vision globale
│   Monde vivant
│   Genesis × Nodyx
│
├── architecture/
│
│   Architecture générale
│   Simulation
│   Cognition
│   Persistence
│   API
│   Networking
│   Godot
│   Nodyx
│
├── design/
│
│   Entités
│   Temps
│   Événements
│   Société
│   Civilisations
│   Émergence
│
└── roadmap/
```

---

# 41. Principes de développement

## 41.1 Ne pas optimiser trop tôt

Le premier objectif est de faire fonctionner la simulation.

La performance sera mesurée avant optimisation.

---

## 41.2 Mesurer

Tout système important doit pouvoir être observé.

Exemples :

```text
Population
Naissances
Morts
Énergie
Relations
Événements
Décisions
Temps CPU
Mémoire
```

---

## 41.3 Rejouer

Un événement intéressant doit pouvoir être rejoué.

```text
Seed
+
Replay
+
Logs
```

---

## 41.4 Tester l'émergence

Les tests ne doivent pas uniquement vérifier :

> "Le code fonctionne."

Ils doivent également permettre de vérifier :

> "Le système produit-il le comportement attendu ?"

---

# 42. Principes sacrés

### 1.

**Le monde continue sans le joueur.**

### 2.

**L'histoire n'est jamais écrite à l'avance.**

### 3.

**Les conséquences sont persistantes.**

### 4.

**Les individus possèdent une mémoire.**

### 5.

**Les sociétés peuvent émerger.**

### 6.

**Le LLM ne définit pas la réalité.**

### 7.

**Godot n'est qu'une fenêtre sur le monde.**

### 8.

**Nodyx constitue la couche communautaire.**

### 9.

**Les expériences doivent être reproductibles.**

### 10.

**Le monde doit pouvoir surprendre ses créateurs.**

---

# 43. Première milestone

## Genesis 0.0.1 — Two Entities

Objectif :

Créer le plus petit monde possible.

```text
WORLD
│
├── Environment
│
├── Entity A
│
└── Entity B
```

Les entités possèdent au minimum :

- position ;
- énergie ;
- état ;
- perception rudimentaire ;
- mouvement ;
- interaction.

Le monde possède :

- une horloge ;
- un environnement ;
- quelques ressources ;
- une boucle de simulation.

---

# 44. Genesis 0.0.2 — Life

Ajouter :

- besoins ;
- consommation ;
- reproduction ;
- vieillissement ;
- mort ;
- mutation ;
- héritage.

Objectif :

> **Obtenir une population capable de se maintenir sans intervention humaine.**

---

# 45. Genesis 0.0.3 — Agents

Ajouter :

- mémoire ;
- personnalité ;
- objectifs ;
- décisions ;
- relations ;
- comportements sociaux.

Objectif :

> **Créer des individus différents.**

---

# 46. Genesis 0.0.4 — Communication

Ajouter :

- signaux ;
- communication ;
- symboles ;
- vocabulaire ;
- apprentissage linguistique.

Objectif :

> **Faire apparaître une communication persistante.**

---

# 47. Genesis 0.0.5 — Society

Ajouter :

- groupes ;
- familles ;
- rôles ;
- coopération ;
- conflits ;
- culture.

Objectif :

> **Faire apparaître une société.**

---

# 48. Genesis 0.0.6 — Civilization

Ajouter :

- villages ;
- villes ;
- politique ;
- économie ;
- religion ;
- technologie ;
- guerre.

Objectif :

> **Faire apparaître une civilisation.**

---

# 49. Genesis 0.0.7 — History

Ajouter :

- événements ;
- chronologie ;
- archives ;
- biographies ;
- cartes historiques ;
- mémoire collective.

Objectif :

> **Créer une histoire persistante.**

---

# 50. Genesis 0.0.8 — Godot

Créer l'interface visuelle.

Objectif :

> **Pouvoir observer le monde.**

---

# 51. Genesis 0.0.9 — Nodyx

Connecter Genesis à Nodyx.

Objectif :

> **Permettre au monde de parler au monde humain.**

---

# 52. Genesis 0.1.0 — Living World

Première version considérée comme véritablement jouable/observable.

Le monde possède :

- vie ;
- individus ;
- mémoire ;
- langage ;
- société ;
- économie ;
- politique ;
- religion ;
- technologie ;
- histoire ;
- visualisation ;
- communauté.

---

# 53. Roadmap long terme

```text
0.0.1  Deux entités
   ↓
0.0.2  Vie
   ↓
0.0.3  Agents
   ↓
0.0.4  Communication
   ↓
0.0.5  Société
   ↓
0.0.6  Civilisation
   ↓
0.0.7  Histoire
   ↓
0.0.8  Godot
   ↓
0.0.9  Nodyx
   ↓
0.1.0  Monde vivant
   ↓
1.0    Monde persistant
   ↓
???    Exploration spatiale
   ↓
???    Entités extérieures
   ↓
???    Premier contact
```

---

# 54. Objectif ultime

Genesis doit devenir un monde que l'on peut laisser tourner.

Pendant une heure.

Un jour.

Une semaine.

Un mois.

Puis revenir.

Et découvrir :

```text
+ 812 421 naissances
+ 790 182 morts
+ 3 civilisations
+ 14 guerres
+ 2 religions
+ 1 révolution
+ 4 découvertes
- 1 civilisation
```

Puis ouvrir Nodyx.

Lire les discussions.

Voir les habitants débattre.

Voir les humains discuter de ces habitants.

Observer les conséquences.

Et comprendre que :

> **Personne n'avait écrit cette histoire.**

---

# 55. Vision finale

Genesis ne doit pas être un monde contrôlé par son développeur.

Il doit être un monde que son développeur **ne comprend jamais totalement**.

Nous construisons :

- les règles ;
- la matière ;
- les organismes ;
- les mécanismes ;
- les contraintes ;
- les outils.

Puis nous laissons le système évoluer.

---

> **Deux molécules.**
>
> **Un monde.**
>
> **Des milliards d'histoires possibles.**
>
> **Et un Observateur derrière l'écran.**

---

## Statut

**Projet :** Genesis  
**Type :** Simulation de vie artificielle persistante  
**Moteur visuel :** Godot  
**Couche communautaire :** Nodyx  
**Infrastructure initiale :** serveur domestique  
**Phase actuelle :** Conception / fondations  
**Première cible :** Genesis 0.0.1 — Two Entities