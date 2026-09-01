# Genesis — Rendering & Optimization

> **Objectif : obtenir un monde visuellement riche et vivant sans sacrifier les performances.**
>
> Genesis doit pouvoir évoluer d'une poignée d'entités primitives jusqu'à des civilisations complexes, tout en restant fluide sur le serveur et dans les clients Godot/WebAssembly.

---

## 1. Philosophie

Genesis ne doit pas chercher à afficher **tout ce qui existe**.

Il doit chercher à afficher **tout ce qui est pertinent**.

La performance ne doit donc pas être obtenue en rendant le monde laid, mais en adaptant intelligemment :

- le niveau de détail ;
- la fréquence de mise à jour ;
- la représentation ;
- la géométrie ;
- les effets ;
- les informations visibles.

### Principe directeur

> **La complexité de la simulation et la complexité du rendu sont deux problèmes différents.**

Une civilisation peut contenir 500 000 individus sans que 500 000 modèles détaillés soient nécessaires à l'écran.

---

# 2. Architecture générale du rendu

```text
                         GENESIS
                            │
              ┌─────────────┴─────────────┐
              │                           │
          SIMULATION                    RENDER
              │                           │
      ┌───────┼────────┐          ┌───────┼────────┐
      │       │        │          │       │        │
   Entities Culture Economy     Geometry Effects  UI
      │       │        │          │       │
      └───────┴────────┘          └───────┴────────┘
              │                           │
              └─────────────┬─────────────┘
                            ▼
                     VIEW / LOD SYSTEM
                            │
                ┌───────────┼───────────┐
                ▼           ▼           ▼
              Macro       Medium       Detail
                │           │           │
                ▼           ▼           ▼
             Agrégats     groupes    individus
```

Le moteur de simulation conserve la réalité complète.

Le moteur de rendu choisit la représentation appropriée.

---

# 3. Vectoriel comme langage visuel

Le projet privilégie une identité graphique vectorielle.

Le vectoriel permet :

- une identité visuelle cohérente ;
- une excellente lisibilité ;
- un redimensionnement propre ;
- des formes procédurales ;
- une génération algorithmique ;
- une faible dépendance aux textures ;
- une adaptation naturelle aux interfaces web.

Cependant :

> **SVG ne doit pas devenir synonyme de milliers de fichiers SVG complexes.**

Le SVG est principalement considéré comme :

1. format artistique ;
2. format d'échange ;
3. source de géométrie ;
4. format pour les éléments importants.

Le rendu massif doit privilégier les primitives et la géométrie procédurale.

---

# 4. Hiérarchie des primitives

Ordre de préférence :

```text
1. Primitive GPU
   │
   ├── Circle
   ├── Rectangle
   ├── Line
   ├── Polygon
   └── Point

2. Géométrie procédurale
   │
   ├── Mesh
   ├── Path
   └── Generated geometry

3. SVG simple
   │
   ├── Symboles
   ├── Drapeaux
   ├── Emblèmes
   └── Interfaces

4. SVG complexe / texture
   │
   └── Utilisation ponctuelle
```

### Règle

> **Plus un élément est nombreux, plus sa représentation doit être simple.**

---

# 5. Level of Detail — LOD

Le LOD est fondamental.

Le monde possède plusieurs niveaux de représentation.

## LOD 0 — Planétaire

Vision globale.

Les individus n'existent pas graphiquement.

On affiche :

- continents ;
- océans ;
- grandes civilisations ;
- frontières ;
- migrations ;
- grandes villes ;
- phénomènes majeurs.

```text
          PLANÈTE

      ███████████
    ███████████████
   █████████████████
       ~~~~~~~~~
    ~~~~~~~~~~~~~~~
```

---

## LOD 1 — Continental

On affiche :

- pays ;
- régions ;
- villes ;
- réseaux ;
- armées ;
- ressources ;
- infrastructures majeures.

Les individus sont agrégés.

---

## LOD 2 — Urbain

On affiche :

- bâtiments ;
- rues ;
- quartiers ;
- populations ;
- groupes sociaux ;
- activités.

Les individus peuvent apparaître sous forme simplifiée.

---

## LOD 3 — Individuel

Lorsque le joueur observe une entité précise :

- morphologie ;
- vêtements ;
- expression ;
- objets ;
- comportement ;
- relations.

---

## LOD 4 — Observation rapprochée

Réservé aux scènes importantes :

- conversation ;
- événement historique ;
- rencontre ;
- découverte ;
- naissance ;
- mort ;
- cérémonie ;
- bataille ;
- invention.

C'est ici que la richesse visuelle maximale est autorisée.

---

# 6. Agrégation

Le moteur ne doit pas représenter chaque individu lorsque ce n'est pas nécessaire.

Exemple :

```text
500 000 habitants
       │
       ▼
Population Renderer
       │
       ├── 200 000 agriculteurs
       ├── 100 000 ouvriers
       ├── 80 000 commerçants
       ├── 50 000 militaires
       └── 70 000 autres
```

À l'échelle macro, cela peut devenir :

```text
🌾 █████████████
⚒  █████████
💰 ███████
⚔  █████
```

Mais dans Genesis :

> **pas d'emojis.**

Les représentations seront des formes vectorielles et des symboles graphiques cohérents avec la direction artistique.

---

# 7. Instancing

Les objets répétitifs doivent être instanciés autant que possible.

Exemple :

```text
Ville
│
├── 50 000 bâtiments
│
└── seulement 25 modèles architecturaux
```

Le moteur réutilise la même géométrie.

Seuls les paramètres changent :

```text
position
rotation
scale
matériau
état
```

Cela réduit considérablement :

- mémoire ;
- allocations ;
- appels de rendu ;
- duplication de géométrie.

---

# 8. Culling

Un objet invisible n'a aucune raison d'être rendu.

Le système doit utiliser :

### Frustum culling

Objets hors caméra :

```text
[ NE PAS RENDRE ]
```

### Distance culling

Objets trop éloignés :

```text
Individu
   ↓
Groupe
   ↓
Population
   ↓
Symbole
```

### Occlusion

Si une zone est complètement masquée :

```text
        CAMERA
          ↓

     ███████████
     █ bâtiment █
     ███████████

       derrière
       → inutile
         à rendre
```

---

# 9. Fréquence de mise à jour

Tout n'a pas besoin d'être recalculé à chaque frame.

Exemple :

```text
Rendu                 60 FPS
Animation             60 FPS
Déplacement            30 FPS
IA locale              10 FPS
Population              5 FPS
Économie                2 FPS
Géopolitique             1 FPS
Histoire                 1 événement
```

La simulation peut donc fonctionner à plusieurs fréquences.

---

# 10. Simulation ≠ rendu

Architecture obligatoire :

```text
Simulation Tick
       │
       ▼
World State
       │
       ▼
Change Detection
       │
       ▼
Render Update
```

Le renderer ne doit pas recalculer inutilement le monde.

Si une ville n'a pas changé :

```text
Ville
  │
  └── aucune modification
          ↓
     aucun rebuild
```

---

# 11. Dirty Flags

Les objets utilisent des états de modification.

Exemple :

```text
Entity
├── transform_dirty
├── appearance_dirty
├── animation_dirty
├── relationship_dirty
└── cognition_dirty
```

Une modification déclenche uniquement le travail nécessaire.

Exemple :

```text
Changement de nom
      ↓
UI uniquement

Mutation physique
      ↓
Visual representation

Déplacement
      ↓
Transform

Mort
      ↓
Population + relations + histoire
```

---

# 12. Object Pooling

Les objets temporaires doivent être réutilisés.

Particulièrement :

- particules ;
- effets ;
- bulles de dialogue ;
- notifications ;
- indicateurs ;
- marqueurs ;
- unités temporaires.

Éviter :

```text
create
destroy
create
destroy
create
destroy
```

Préférer :

```text
POOL
 │
 ├── disponible
 ├── disponible
 ├── utilisé
 ├── disponible
 └── utilisé
```

---

# 13. Animation

Les animations doivent être pilotées par états.

```text
IDLE
 │
 ├── WALK
 ├── WORK
 ├── TALK
 ├── FIGHT
 ├── SLEEP
 └── DEAD
```

Pas besoin d'une animation complexe par individu.

Plusieurs individus peuvent partager :

```text
Animation
     +
Paramètres différents
```

---

# 14. Génération procédurale

Le monde doit être capable de générer sa représentation.

Exemples :

### Individu

```text
Genome
   ↓
Morphology Generator
   ↓
Vector Geometry
```

### Architecture

```text
Culture
   +
Technology
   +
Climate
   ↓
Building Generator
   ↓
Vector Geometry
```

### Drapeau

```text
Religion
   +
Culture
   +
History
   ↓
Symbol Generator
   ↓
Vector Flag
```

La diversité visuelle doit venir principalement des données du monde.

---

# 15. Identité graphique des civilisations

Chaque civilisation doit progressivement développer une identité visuelle.

```text
Civilisation
│
├── Culture
├── Religion
├── Histoire
├── Technologie
├── Architecture
├── Symboles
└── Esthétique
        │
        ▼
Visual Identity
```

Cette identité peut influencer :

- architecture ;
- vêtements ;
- interfaces ;
- drapeaux ;
- symboles ;
- typographie ;
- couleurs ;
- motifs ;
- véhicules ;
- monuments.

Ainsi, deux civilisations technologiquement équivalentes peuvent être immédiatement reconnaissables.

---

# 16. Shaders

Les shaders sont réservés aux phénomènes nécessitant une animation continue.

Exemples :

- eau ;
- nuages ;
- atmosphère ;
- lumière ;
- énergie ;
- phénomènes spatiaux ;
- météo ;
- effets de transition.

Ils doivent éviter de remplacer inutilement une géométrie simple.

---

# 17. WebAssembly

Le client WebAssembly doit être considéré comme une cible importante.

Objectifs :

- démarrage rapide ;
- faible mémoire ;
- téléchargement raisonnable ;
- rendu fluide ;
- absence de dépendance à un matériel spécifique.

Le renderer doit donc éviter les fonctionnalités inutilement coûteuses.

### Principe

> **Le client web ne doit jamais avoir besoin de simuler le monde complet.**

Il reçoit uniquement :

```text
World Snapshot
+
Visible Region
+
Relevant Entities
+
Events
```

---

# 18. Serveur

Le serveur possède la simulation complète.

```text
                    SERVER
                       │
              Complete World State
                       │
             ┌─────────┴─────────┐
             │                   │
          Simulation            API
             │                   │
             │             ┌─────┴─────┐
             │             │           │
             ▼             ▼           ▼
          Database       Godot       Nodyx
```

Le serveur ne doit pas dépendre du rendu.

La simulation doit pouvoir fonctionner :

```text
sans fenêtre
sans GPU
sans Godot
sans client
```

---

# 19. Nodyx

Nodyx devient une couche communautaire au-dessus de la simulation.

Le monde peut produire des événements :

```text
Civilisation A découvre l'écriture.

          ↓

EVENT

          ↓

Nodyx
```

Les utilisateurs peuvent alors :

- observer ;
- commenter ;
- suivre une civilisation ;
- suivre une entité ;
- discuter d'un événement ;
- consulter l'histoire ;
- créer des communautés ;
- assister à des événements importants.

Le renderer Genesis n'a donc pas besoin d'afficher tout.

**Nodyx devient une seconde fenêtre sur le monde.**

---

# 20. Priorité visuelle

Quand les performances deviennent limitées, Genesis doit sacrifier dans cet ordre :

```text
1. Effets secondaires
2. Animations secondaires
3. Détails géométriques
4. Entités éloignées
5. Entités agrégées

NE JAMAIS sacrifier en premier :

1. Lisibilité
2. Identité des civilisations
3. Événements importants
4. Informations historiques
5. Entités observées par le joueur
```

---

# 21. Budget de rendu

Chaque scène doit avoir un budget.

Exemple initial :

```text
                    Budget
────────────────────────────────
CPU simulation       séparé
CPU rendering        limité
GPU geometry         limité
GPU effects          limité
Memory               surveillée
Draw calls            surveillées
Entities visible     dynamique
```

Les valeurs exactes seront déterminées par benchmark.

**Aucune optimisation basée uniquement sur une intuition.**

---

# 22. Profilage

Chaque version importante doit pouvoir être profilée.

Mesures principales :

```text
FPS
Frame time
CPU time
GPU time
Draw calls
Vertices
Triangles
Memory
Entity count
Visible entity count
Simulation tick
Network traffic
```

Le projet doit intégrer un mode développeur :

```text
[ F3 ] DEBUG OVERLAY

FPS: 59.8
Frame: 16.4 ms

Entities:       482 391
Visible:          4 218
Rendered:           932

Draw calls:         184
Vertices:        82 421

Simulation tick:  7.2 ms
Render:            5.1 ms
Network:           1.3 ms
```

---

# 23. Scalabilité

Genesis doit être conçu pour fonctionner sur plusieurs échelles.

### Petite simulation

```text
10 – 100 entités
```

Rendu détaillé possible.

### Simulation moyenne

```text
1 000 – 10 000 entités
```

LOD + instancing.

### Grande simulation

```text
100 000+ entités
```

Agrégation + culling + LOD agressif.

### Civilisation avancée

```text
1 000 000+ individus
```

La majorité des individus existent dans la simulation mais ne sont jamais représentés individuellement.

---

# 24. Règle d'or

> **La simulation peut être infiniment complexe.**
>
> **Le rendu doit rester sélectif.**

Le joueur doit avoir l'impression que :

> *"Tout ce monde existe."*

même lorsque seulement une petite fraction est réellement dessinée à l'écran.

---

# 25. Objectif final

Genesis doit donner l'impression d'observer une véritable planète vivante.

Pas une accumulation de sprites.

Pas une grille remplie d'icônes.

Pas une interface Excel animée.

Mais un monde dont la représentation évolue naturellement avec :

```text
Matière
  ↓
Vie
  ↓
Organismes
  ↓
Individus
  ↓
Groupes
  ↓
Sociétés
  ↓
Civilisations
  ↓
Technologies
  ↓
Planète
  ↓
Espace
```

Le moteur graphique doit être suffisamment intelligent pour que cette complexité apparaisse **sans que la machine ait besoin de tout dessiner en permanence**.

---

# 26. Principe ultime

### Optimiser sans appauvrir.

Genesis ne doit jamais dire :

> "Nous avons trop d'entités, donc supprimons le détail."

Il doit dire :

> **"Nous avons trop d'entités, donc choisissons intelligemment lesquelles méritent d'être détaillées."**

La performance devient alors une partie intégrante de la mise en scène.

**Le joueur regarde le monde.  
Le moteur décide ce qui mérite d'être vu.**