# Genesis — Temporal Simulation & Time Perception

> **Le monde doit pouvoir évoluer sur des millions d'années sans que le joueur ait l'impression d'attendre.**
>
> **Mais lorsque le monde devient complexe et intéressant, le temps doit progressivement ralentir afin que chaque événement puisse être observé.**

---

# 1. Objectif

Genesis doit résoudre un problème fondamental :

- l'évolution biologique nécessite énormément de temps ;
- les civilisations peuvent exister pendant des milliers d'années ;
- les événements historiques doivent pouvoir être observés ;
- les joueurs ne peuvent pas attendre des milliers d'années ;
- une simulation trop rapide rendrait les civilisations insignifiantes.

La solution est de séparer :

```text
Simulation Time
World Time
Perceived Time
Narrative Time
```

---

# 2. Les quatre notions de temps

## Simulation Time

Temps interne du moteur.

```text
simulation_tick
```

Il peut évoluer extrêmement rapidement.

---

## World Time

Temps vécu par le monde.

Exemple :

```text
Year 1
Year 2
Year 3
...
Year 100000
```

---

## Perceived Time

Temps réellement ressenti par le joueur.

Une seconde réelle peut représenter :

```text
10 000 années
```

au début.

Puis :

```text
100 années
```

puis :

```text
1 année
```

puis :

```text
1 jour
```

---

## Narrative Time

Temps consacré aux événements importants.

Une période de 500 ans peut être résumée en quelques secondes.

Mais une bataille, une découverte ou une conversation importante peut être détaillée.

---

# 3. Temporal Compression

Le monde utilise une compression temporelle dynamique.

```text
REAL TIME
     │
     ▼
TIME SCALE
     │
     ▼
WORLD TIME
```

Exemple :

```text
1 seconde réelle
        ↓
10 000 années
```

ou :

```text
1 seconde réelle
        ↓
1 journée
```

---

# 4. Échelle dynamique

La vitesse dépend de la complexité du monde.

### Phase cosmique / chimique

```text
× 1 000 000
```

Des millions d'années peuvent passer très rapidement.

---

### Phase biologique

```text
× 100 000
```

---

### Phase animale

```text
× 10 000
```

---

### Premières sociétés

```text
× 1 000
```

---

### Civilisation

```text
× 100
```

---

### Civilisation avancée

```text
× 10
```

---

### Monde technologique

```text
× 1
```

---

### Événement critique

```text
× 0.1
× 0.01
PAUSE
```

---

# 5. Le principe de ralentissement

La vitesse n'est pas seulement liée à l'âge du monde.

Elle dépend également de son **niveau de complexité**.

Exemple :

```text
Complexité
    │
    │                         █
    │                     ████
    │                 ███████
    │            ███████████
    │       ███████████████
    └───────────────────────────
      Temps
```

Plus les systèmes sociaux deviennent complexes :

- économie ;
- politique ;
- religion ;
- diplomatie ;
- technologie ;
- individus importants ;

plus Genesis peut réduire automatiquement la compression temporelle.

---

# 6. Le monde ne doit jamais être "vide"

Pendant les phases rapides, Genesis doit produire des événements observables.

Exemple :

```text
GENESIS FEED

+ 18 294 années

🧬 Nouvelle mutation dominante

+ 4 921 années

🦠 Première cellule multicellulaire

+ 12 003 années

🌱 Nouvelle espèce végétale

+ 84 201 années

🐾 Première créature terrestre
```

Le joueur observe l'évolution au lieu d'attendre.

---

# 7. Fast Forward

Le joueur possède plusieurs vitesses.

```text
⏸ Pause

▶ 1×

▶▶ 10×

▶▶▶ 100×

▶▶▶ 1 000×

⏩ Deep Time
```

La vitesse maximale peut dépendre de la phase du monde.

---

# 8. Auto-Speed

Genesis peut ajuster automatiquement la vitesse.

```text
if world_complexity increases:
    time_scale decreases
```

Exemple :

```text
Chimie
×1 000 000

Vie
×100 000

Intelligence
×10 000

Civilisation
×1 000

Politique complexe
×100

Technologie avancée
×10

Interaction humaine
×1
```

---

# 9. Focus System

Le joueur peut sélectionner un sujet.

```text
FOCUS

🌍 Monde
🧬 Évolution
👤 Individu
🏛️ Civilisation
⚔️ Guerre
💰 Économie
🙏 Religion
🔬 Science
🚀 Technologie
🛰️ Exploration
```

Genesis adapte alors la granularité de simulation et de présentation.

---

# 10. Level of Detail — Simulation

Le concept de LOD utilisé en rendu 3D est appliqué à la simulation.

## Niveau 0 — Macro

```text
Population
Ressources
Climat
Espèces
Civilisations
```

---

## Niveau 1 — Région

```text
Villes
Territoires
Populations
Armées
Ressources
```

---

## Niveau 2 — Société

```text
Familles
Groupes
Institutions
Religions
Marchés
```

---

## Niveau 3 — Individu

```text
Personnalité
Mémoire
Relations
Objectifs
Émotions
Décisions
```

---

## Niveau 4 — Interaction

```text
Conversation
Décision
Conflit
Découverte
Événement
```

Genesis ne simule donc pas systématiquement tout au niveau individuel.

---

# 11. Importance des événements

Chaque événement possède un niveau d'importance.

```text
importance = 0.0 → 1.0
```

Exemple :

```text
Mort d'un individu inconnu
0.01

Fondation d'une ville
0.40

Guerre majeure
0.75

Chute d'un empire
0.90

Découverte d'une nouvelle source d'énergie
0.95

Premier contact extraterrestre
1.00
```

Les événements importants peuvent provoquer :

```text
TIME_SCALE ↓
```

---

# 12. Event Focus

Lorsqu'un événement majeur survient :

```text
WORLD
  │
  ▼
EVENT DETECTED
  │
  ▼
IMPORTANCE
  │
  ├── faible → continuer
  │
  └── forte
        │
        ▼
   ralentissement
        │
        ▼
   notification
```

Exemple :

> **⚡ Une découverte scientifique majeure vient de se produire.**

Le joueur peut cliquer.

Genesis ralentit automatiquement.

---

# 13. Zoom temporel

Le joueur peut passer du macro au micro.

Exemple :

```text
100 000 ans
      ↓
10 000 ans
      ↓
1 000 ans
      ↓
100 ans
      ↓
10 ans
      ↓
1 an
      ↓
1 mois
      ↓
1 jour
      ↓
1 heure
```

Le monde reste le même.

Seule la granularité change.

---

# 14. Historical Replay

Les événements importants doivent être enregistrés.

```text
Timeline

YEAR 0
Origine

YEAR 14 829
Première vie

YEAR 91 202
Première intelligence

YEAR 103 884
Premier langage

YEAR 107 291
Première cité

YEAR 108 004
Première guerre

YEAR 114 882
Premier empire
```

Le joueur peut sélectionner un événement.

---

# 15. Retour sur une période

Le joueur peut demander :

> **"Que s'est-il passé pendant cette période ?"**

Genesis produit un résumé historique.

```text
YEAR 1200 → YEAR 1300

Population : +32%

Nouvelles villes : 18

Guerres : 4

Religions fondées : 2

Technologies découvertes : 7

Extinctions : 1

Événement majeur :
Révolution politique de Velkar
```

---

# 16. Absence du joueur

Genesis continue lorsqu'il n'est pas observé.

Au retour :

```text
WELCOME BACK

Vous avez été absent :

3 heures 42 minutes

Le monde a vécu :

+ 842 années
```

Puis :

```text
Pendant votre absence :

12 nouvelles villes
3 guerres
1 nouvelle religion
2 découvertes scientifiques
1 extinction
```

Le monde doit donner l'impression :

> **d'avoir continué à vivre sans son créateur.**

---

# 17. Idle Progression

Le mode hors ligne doit être fondamental.

```text
Player Offline
      │
      ▼
Genesis continues
      │
      ▼
Events generated
      │
      ▼
History recorded
      │
      ▼
Player returns
      │
      ▼
Catch-up summary
```

---

# 18. Limites du Offline Simulation

La simulation hors ligne peut utiliser plusieurs niveaux.

```text
Offline:

Macro Simulation
      ↓
Population
      ↓
Civilisation
      ↓
Important Individuals
```

Il n'est pas nécessaire de faire tourner tous les agents individuellement pendant toute la période d'absence.

---

# 19. Événements critiques

Certains événements peuvent interrompre temporairement la compression.

Exemples :

```text
☄️ Impact majeur
🌋 Éruption
⚔️ Guerre
👑 Mort d'un dirigeant
🔬 Découverte
🚀 Premier lancement spatial
👽 Contact extérieur
🧠 Apparition d'une nouvelle intelligence
```

Genesis peut alors passer :

```text
×10 000
   ↓
×100
   ↓
×1
   ↓
PAUSE
```

---

# 20. Civilisation vivante

Plus le monde devient complexe, plus son temps doit être lisible.

Une civilisation avancée peut produire :

```text
08:41
Un scientifique publie une théorie.

08:47
Une université la conteste.

09:13
Une seconde école scientifique apparaît.

11:02
Le débat atteint les forums publics.

14:27
Le gouvernement annonce une commission.
```

Le joueur peut réellement **observer l'histoire se créer**.

---

# 21. Nodyx comme fenêtre temporelle

Nodyx devient la fenêtre permettant aux humains d'observer Genesis.

```text
GENESIS
   │
   ▼
WORLD
   │
   ▼
EVENT STREAM
   │
   ▼
NODYX
   │
   ├── Forums
   ├── Chat
   ├── Vocal
   ├── Archives
   └── Observations
```

Les humains ne voient donc pas nécessairement chaque tick.

Ils voient :

> **les conséquences du temps.**

---

# 22. Le rythme idéal

Le joueur doit ressentir trois sensations.

### Phase 1

> **"Wow, ça évolue super vite."**

### Phase 2

> **"Attends... cette civilisation commence à devenir intéressante."**

### Phase 3

> **"Putain, ils vivent réellement."**

Le rythme doit progressivement passer :

```text
WOW
 ↓
CURIOSITÉ
 ↓
ATTACHEMENT
 ↓
OBSERVATION
 ↓
IMMERSION
```

---

# 23. Le moment critique

Il existe un moment particulièrement important dans la progression.

La simulation passe de :

```text
Je regarde l'évolution
```

à :

```text
Je regarde des individus.
```

Puis :

```text
Je connais cet individu.
```

Puis :

```text
Je veux savoir ce qui va lui arriver.
```

C'est à ce moment que Genesis doit fortement réduire la compression temporelle.

---

# 24. Temps émotionnel

La vitesse ne doit donc pas dépendre uniquement de paramètres techniques.

Elle peut également dépendre de l'intérêt du joueur.

Exemple :

```text
Player Focus
     │
     ▼
Entity A
     │
     ▼
Narrative Importance
     │
     ▼
Time Scale ↓
```

Un personnage auquel le joueur s'intéresse peut être observé beaucoup plus finement.

---

# 25. Attention à ne pas tricher

Genesis ne doit pas artificiellement ralentir le temps uniquement pour créer du spectacle.

Les changements doivent être justifiés par :

- complexité ;
- importance ;
- densité d'événements ;
- niveau d'observation ;
- capacité de calcul ;
- choix du joueur.

La simulation reste déterministe ou contrôlablement stochastique.

---

# 26. Performance

Le système temporel doit également servir les performances.

```text
Complexity LOW
      ↓
High compression
      ↓
Aggregate simulation
```

Puis :

```text
Complexity HIGH
      ↓
Lower compression
      ↓
Detailed simulation
```

Le temps devient donc également un outil d'optimisation.

---

# 27. Architecture conceptuelle

```text
                TEMPORAL ENGINE
                       │
         ┌─────────────┼─────────────┐
         │             │             │
     Time Scale     Complexity     Focus
         │             │             │
         └─────────────┼─────────────┘
                       ▼
                Simulation LOD
                       │
              ┌────────┴────────┐
              │                 │
          Macro Sim         Detailed Sim
              │                 │
              └────────┬────────┘
                       ▼
                 Event System
                       │
                       ▼
                  Nodyx Feed
```

---

# 28. Philosophie

> **Au commencement, le temps est immense et le joueur est impatient.**
>
> **À la fin, le temps devient précieux et le joueur ne veut plus qu'il passe.**

C'est cette inversion que Genesis doit rechercher.

Au début :

```text
Millions d'années
→ quelques secondes
```

À la fin :

```text
Quelques minutes
→ plusieurs événements
→ plusieurs décisions
→ plusieurs conséquences
```

---

# 29. Objectif final

Le joueur doit pouvoir regarder une civilisation pendant :

> **cinq minutes réelles**

et avoir l'impression d'avoir assisté à :

- une découverte ;
- une dispute ;
- une décision politique ;
- une relation ;
- un événement religieux ;
- une évolution économique ;
- une conséquence historique.

Puis fermer Nodyx.

Revenir le lendemain.

Et découvrir :

> **"Pendant votre absence, ils ont changé."**

C'est le cœur du système temporel de Genesis.