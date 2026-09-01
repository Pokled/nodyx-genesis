# Projet — Le Jeu de la Vie

> **Une simulation autonome où la vie apparaît, évolue et construit sa propre civilisation.**
>
> Et moi, je suis Dieu.

---

## 1. Vision

Créer sur un serveur domestique une simulation persistante d'un monde virtuel.

Au départ, il n'y a presque rien.

Quelques particules, molécules ou structures primitives évoluent dans un environnement soumis à des règles simples.

À travers les interactions, la reproduction, les mutations et la sélection, la vie apparaît.

Puis elle évolue.

Des organismes apparaissent.

Certains développent une mémoire, des comportements complexes et une capacité à communiquer.

À terme, certaines espèces peuvent développer :

- une intelligence ;
- un langage ;
- des relations sociales ;
- des groupes ;
- des cultures ;
- des croyances ;
- des conflits ;
- des outils ;
- des villes ;
- des sciences ;
- des technologies ;
- des civilisations.

L'objectif n'est **pas de programmer une histoire à l'avance**.

L'objectif est de créer les règles permettant à une histoire d'apparaître.

---

# 2. Le principe fondamental

## Ne pas simuler l'histoire. Simuler les conditions de son apparition.

On ne doit pas écrire :

```text
Jour 10 000 → apparition de l'agriculture
Jour 20 000 → apparition des villes
Jour 30 000 → invention de l'écriture
```

On doit plutôt créer des mécanismes tels que :

```text
Les organismes peuvent :
- apprendre
- mémoriser
- communiquer
- modifier leur environnement
- transmettre des informations
- fabriquer des objets
- coopérer
- entrer en conflit
```

Et laisser le système produire lui-même les conséquences.

---

# 3. Les niveaux du monde

La simulation doit être pensée comme plusieurs niveaux d'émergence.

```text
┌─────────────────────────────┐
│        CIVILISATION         │
├─────────────────────────────┤
│          CULTURE            │
├─────────────────────────────┤
│          SOCIÉTÉ            │
├─────────────────────────────┤
│          LANGAGE            │
├─────────────────────────────┤
│        INTELLIGENCE         │
├─────────────────────────────┤
│         ORGANISME           │
├─────────────────────────────┤
│           CELLULE           │
├─────────────────────────────┤
│         MOLÉCULES           │
├─────────────────────────────┤
│        ENVIRONNEMENT        │
└─────────────────────────────┘
```

Chaque couche doit pouvoir influencer les autres.

Une civilisation modifie son environnement.

L'environnement influence les organismes.

Les organismes évoluent.

L'évolution modifie les sociétés.

Les sociétés produisent de nouvelles technologies.

Les technologies modifient à nouveau l'environnement.

---

# 4. Le monde physique

Le monde possède :

- une géographie ;
- une température ;
- de l'eau ;
- des ressources ;
- des zones habitables ;
- des cycles jour/nuit ;
- éventuellement des saisons ;
- des phénomènes météorologiques ;
- des catastrophes naturelles.

La physique n'a pas besoin d'être réaliste.

Elle doit être :

1. cohérente ;
2. suffisamment riche ;
3. calculable ;
4. capable de produire des comportements émergents.

---

# 5. La chimie

La chimie peut être abstraite.

Exemple :

```text
A + B → C
C + énergie → D
D + eau → E
E + E → F
```

Certaines molécules peuvent être :

- énergétiques ;
- toxiques ;
- nutritives ;
- catalytiques ;
- structurantes ;
- reproductives.

L'objectif initial n'est pas de reproduire la chimie réelle de la Terre.

L'objectif est de créer une **chimie permettant l'émergence de systèmes complexes**.

---

# 6. Apparition de la vie

Une structure est considérée comme vivante lorsqu'elle possède certaines propriétés.

Par exemple :

```text
             ┌──────────────┐
             │    VIE       │
             └──────┬───────┘
                    │
        ┌───────────┼───────────┐
        ↓           ↓           ↓
   Métabolisme   Réplication   Adaptation
        │           │           │
        └───────────┼───────────┘
                    ↓
                 Évolution
```

La première vie n'a pas besoin d'être intelligente.

Elle peut être extrêmement primitive.

---

# 7. Évolution

Les organismes possèdent un génome numérique.

Exemple :

```json
{
  "speed": 0.63,
  "vision": 0.72,
  "memory": 0.41,
  "aggression": 0.18,
  "curiosity": 0.91,
  "sociality": 0.87,
  "intelligence": 0.55
}
```

Lors de la reproduction :

```text
Parent A
   +
Parent B
   ↓
Combinaison génétique
   ↓
Mutation
   ↓
Nouvel organisme
```

Les mutations peuvent toucher :

- les caractéristiques physiques ;
- les capacités sensorielles ;
- le comportement ;
- la mémoire ;
- l'apprentissage ;
- la sociabilité ;
- la curiosité ;
- l'intelligence.

La sélection naturelle résulte simplement des conséquences du monde.

---

# 8. Intelligence

L'intelligence ne doit pas apparaître simplement parce qu'une valeur atteint `1.0`.

Elle doit être liée à des capacités.

Par exemple :

```text
perception
    ↓
mémoire
    ↓
apprentissage
    ↓
anticipation
    ↓
planification
    ↓
raisonnement
    ↓
communication
    ↓
concepts abstraits
```

Une créature intelligente possède :

- une mémoire ;
- des objectifs ;
- des besoins ;
- une personnalité ;
- des connaissances ;
- des expériences ;
- des relations avec d'autres individus.

---

# 9. Les individus

Chaque individu doit posséder un état propre.

Exemple conceptuel :

```json
{
  "id": "entity_8291",
  "species": "species_17",

  "age": 38,

  "needs": {
    "food": 0.71,
    "water": 0.42,
    "safety": 0.81,
    "social": 0.34
  },

  "personality": {
    "curiosity": 0.91,
    "aggression": 0.21,
    "empathy": 0.77,
    "risk": 0.62
  },

  "memory": [],
  "knowledge": [],
  "relationships": [],

  "location": [124, 82]
}
```

---

# 10. Mémoire

La mémoire est fondamentale.

Une créature doit pouvoir se souvenir de choses comme :

```text
"J'ai trouvé de la nourriture près de la rivière."

"Cette créature m'a attaqué."

"Mon père est mort ici."

"Le groupe voisin nous a aidés."

"Le feu est dangereux."

"Cette personne m'a menti."
```

Les souvenirs peuvent influencer les décisions futures.

---

# 11. Langage

Le langage doit émerger progressivement.

Au début :

```text
SIGNAL → danger
SIGNAL → nourriture
SIGNAL → reproduction
```

Puis :

```text
symbole → objet
symbole → individu
symbole → action
symbole → lieu
```

Puis :

```text
concepts
relations
temps
causalité
abstraction
```

Les langues peuvent évoluer.

Deux populations isolées peuvent finir par parler des langues différentes.

---

# 12. LLM

Les modèles de langage ne doivent **pas simuler chaque créature à chaque tick**.

Ce serait beaucoup trop coûteux.

Le moteur principal fonctionne normalement :

```text
Simulation
   ↓
État de l'individu
   ↓
Événement important ?
   ↓
Oui
   ↓
Cognition avancée
   ↓
LLM
```

Le LLM intervient pour :

- réflexion ;
- dialogue ;
- décision complexe ;
- interprétation ;
- création de concepts ;
- narration personnelle ;
- croyances ;
- transmission culturelle.

Le LLM est donc **une couche cognitive**, pas le moteur du monde.

---

# 13. Exemple de pensée

Une créature pourrait recevoir :

```text
OBJECTIF :
trouver de la nourriture

ÉTAT :
faim = 82%

CONNAISSANCES :
rivière → poissons
forêt → fruits

MÉMOIRES :
la forêt est dangereuse
```

Le système cognitif peut produire :

```text
"Je suis très affamé.
La rivière contient des poissons.
Mais je me souviens du prédateur rencontré
dans la forêt."

→ décision : aller à la rivière
```

---

# 14. Société

Lorsque plusieurs individus interagissent régulièrement, des structures sociales peuvent émerger.

Exemples :

```text
famille
tribu
clan
village
ville
royaume
nation
empire
```

Ces structures doivent avoir leur propre état.

Une société peut avoir :

- une population ;
- un territoire ;
- des ressources ;
- des règles ;
- des dirigeants ;
- une culture ;
- une langue ;
- une religion ;
- des relations diplomatiques.

---

# 15. Culture

Les individus transmettent des informations.

Cela permet l'apparition d'une culture.

Exemples :

```text
"Le feu permet de cuire la nourriture."

"Ne traverse jamais cette montagne."

"Nos ancêtres vivaient ici."

"Le soleil est sacré."

"Cette tribu est notre ennemie."
```

Une information répétée suffisamment longtemps peut devenir une tradition.

Une tradition peut devenir une croyance.

Une croyance peut devenir une religion.

---

# 16. Technologie

La technologie apparaît lorsque les individus combinent leurs connaissances.

Exemple :

```text
pierre
  +
bois
  +
connaissance de la friction
  ↓
outil
```

Puis :

```text
outil
  ↓
meilleur rendement
  ↓
spécialisation
  ↓
artisanat
  ↓
industrie
```

La technologie doit dépendre :

- des ressources disponibles ;
- des connaissances ;
- de l'intelligence ;
- de la transmission culturelle ;
- des besoins ;
- de l'environnement.

---

# 17. Civilisation

Une civilisation apparaît lorsqu'une société possède suffisamment de structures complexes.

Exemples :

```text
agriculture
écriture
commerce
urbanisation
gouvernement
armée
science
architecture
industrie
```

Il n'existe aucune garantie qu'une civilisation apparaisse.

C'est justement le but.

---

# 18. Le joueur : Dieu

Le joueur est extérieur à la simulation.

Il peut observer le monde.

Mais il peut également intervenir.

Pouvoirs possibles :

```text
👁 Observer

💬 Parler à un individu

🌱 Faire apparaître une ressource

💧 Créer de l'eau

☀️ Modifier la météo

⚡ Provoquer un événement

🌋 Déclencher une catastrophe

🧬 Modifier un organisme

✨ Créer un miracle

☄️ Faire tomber un météore
```

L'intervention doit avoir des conséquences.

---

# 19. La communication avec Dieu

Une fonctionnalité centrale.

Le joueur sélectionne un individu.

Exemple :

```text
ELD
Âge : 38 ans
Espèce : Homo-X
Village : Kora
```

Puis :

```text
> Je suis celui qui a créé ton monde.
```

L'individu répond selon :

- sa personnalité ;
- ses connaissances ;
- sa culture ;
- ses croyances ;
- son expérience ;
- son niveau d'intelligence.

Exemple :

> « Je ne sais pas qui tu es. Mais depuis trois nuits, j'entends ta voix lorsque je dors. »

L'individu pourrait ensuite raconter cette rencontre.

Une religion pourrait apparaître.

Ou personne ne le croire.

Ou l'individu pourrait être considéré comme fou.

---

# 20. L'histoire du monde

Le serveur conserve une chronologie.

Exemple :

```text
════════════════════════════════════
AN 0
════════════════════════════════════

La simulation commence.

════════════════════════════════════
AN 742
════════════════════════════════════

Première structure autoréplicante.

════════════════════════════════════
AN 19 284
════════════════════════════════════

Première cellule complexe.

════════════════════════════════════
AN 31 882
════════════════════════════════════

Première créature capable de communication
symbolique.

════════════════════════════════════
AN 32 104
════════════════════════════════════

Fondation de la première ville.

════════════════════════════════════
AN 35 001
════════════════════════════════════

Un individu affirme avoir parlé à une
entité extérieure.

════════════════════════════════════
AN 35 002
════════════════════════════════════

Naissance du culte des Observateurs.
```

---

# 21. Persistance

Le monde doit continuer à vivre même lorsque personne ne le regarde.

Le serveur fonctionne comme une horloge.

```text
┌────────────────────┐
│      SERVER        │
│                    │
│  Simulation        │
│  ↓                 │
│  Events            │
│  ↓                 │
│  Database          │
│  ↓                 │
│  Web interface     │
└────────────────────┘
```

Le joueur peut quitter le serveur.

Revenir plusieurs heures plus tard.

Et découvrir que :

**le monde a continué sans lui.**

---

# 22. Architecture envisagée

Une architecture initiale pourrait être :

```text
                    ┌──────────────┐
                    │   FRONTEND   │
                    │              │
                    │ Web UI       │
                    │ Map          │
                    │ Timeline     │
                    │ Individuals  │
                    └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │      API     │
                    └──────┬───────┘
                           │
             ┌─────────────┼─────────────┐
             ▼             ▼             ▼
       ┌──────────┐  ┌──────────┐  ┌──────────┐
       │  WORLD   │  │  AGENTS  │  │   AI     │
       │          │  │          │  │          │
       │ Physics  │  │ Memory   │  │ LLM      │
       │ Biology  │  │ Behavior │  │ Cognition│
       │ Ecology  │  │ Society  │  │ Language │
       └──────────┘  └──────────┘  └──────────┘
             │             │             │
             └─────────────┼─────────────┘
                           ▼
                    ┌──────────────┐
                    │  DATABASE    │
                    │              │
                    │ World state  │
                    │ History      │
                    │ Memories     │
                    │ Events       │
                    └──────────────┘
```

---

# 23. Performance

Principe essentiel :

> **Tout ne doit pas être simulé avec la même précision.**

Une créature très loin de toute civilisation peut être simulée très simplement.

Une créature qui interagit avec le joueur peut être simulée beaucoup plus précisément.

On peut utiliser plusieurs niveaux :

```text
Niveau 0
Simulation statistique

Niveau 1
Simulation individuelle simplifiée

Niveau 2
Simulation comportementale

Niveau 3
Cognition avancée

Niveau 4
LLM + mémoire + dialogue
```

Cela permet au monde de contenir potentiellement énormément d'entités sans exploser les ressources du serveur.

---

# 24. Événements

Le monde produit des événements.

```text
Naissance
Mort
Rencontre
Combat
Découverte
Invention
Migration
Mariage
Trahison
Guerre
Alliance
Catastrophe
Révolution
Découverte scientifique
```

Les événements importants sont enregistrés.

Ils deviennent l'histoire du monde.

---

# 25. Objectif ultime

Le projet ne possède pas réellement de victoire.

Il possède une question :

> **Que va-t-il se passer si je laisse tourner ce monde ?**

Le joueur ne doit pas savoir quelle civilisation apparaîtra.

Ni quelles espèces survivront.

Ni quelles religions seront créées.

Ni quelles technologies émergeront.

Ni même si une intelligence comparable à la nôtre apparaîtra.

Le plaisir vient de l'observation.

---

# 26. Philosophie du projet

Le monde doit être **émergent**.

Le développeur définit les règles.

Les règles produisent des interactions.

Les interactions produisent des comportements.

Les comportements produisent des structures.

Les structures produisent une histoire.

```text
RÈGLES
  ↓
INTERACTIONS
  ↓
COMPORTEMENTS
  ↓
ÉMERGENCE
  ↓
HISTOIRE
```

---

# 27. Première version — MVP

Ne pas commencer par la civilisation.

Commencer extrêmement petit.

### Phase 1 — Monde

- grille 2D ;
- terrain ;
- eau ;
- température ;
- ressources ;
- simulation du temps.

### Phase 2 — Chimie

- particules ;
- molécules ;
- réactions ;
- énergie.

### Phase 3 — Vie

- réplication ;
- mutation ;
- métabolisme ;
- mort ;
- évolution.

### Phase 4 — Organismes

- déplacement ;
- perception ;
- nourriture ;
- reproduction ;
- comportement.

### Phase 5 — Individus

- mémoire ;
- personnalité ;
- objectifs ;
- relations.

### Phase 6 — Communication

- signaux ;
- symboles ;
- langage primitif.

### Phase 7 — Intelligence

- apprentissage ;
- planification ;
- cognition ;
- LLM.

### Phase 8 — Société

- groupes ;
- culture ;
- territoire ;
- commerce ;
- conflit.

### Phase 9 — Civilisation

- villes ;
- institutions ;
- science ;
- technologie.

### Phase 10 — Dieu

- observation ;
- intervention ;
- miracles ;
- dialogue avec les créatures.

---

# 28. Règle d'or

**Ne jamais ajouter une règle uniquement parce qu'elle permet d'obtenir le résultat souhaité.**

Si une civilisation apparaît, elle doit apparaître parce que les mécanismes du monde l'ont rendue possible.

Si une créature devient intelligente, elle doit avoir une histoire qui explique pourquoi.

Si une religion apparaît, elle doit venir des individus qui la composent.

Le monde doit pouvoir raconter son histoire sans que le développeur l'écrive à sa place.

---

# 29. Nom du projet

Nom temporaire :

**Le Jeu de la Vie**

Noms possibles plus tard :

- Genesis
- Emergence
- Genesis Engine
- The Observer
- Deus
- Eden
- World 0
- Project Genesis
- The Living World
- **DIEU**
- **Et si… ?**

---

# 30. Question centrale

> **Si je crée un monde avec suffisamment de règles simples et suffisamment de liberté, quelle histoire va-t-il écrire tout seul ?**

Et surtout :

> **Que se passera-t-il lorsqu'ils découvriront que quelqu'un les observe ?**