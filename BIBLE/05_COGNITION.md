# 05. Cognition

Statut : TRANCHES 1 ET 2 FAITES (le premier souvenir, puis la page biographie). Le pont
Entity vers Agent et la question semé ou cultivé sont posés ; la mémoire épisodique minimale
est construite, tourne, et se lit dans `lives.html`. L'architecture détaillée (Context
Builder, Model Router, RAG) attend 0.0.5 et sera reprise ici à ce moment.

Dernière révision : 2026-09-01.

Ce document est ce que pointe la tranchée T-6 pour la partie cognition. Il cadre le jalon
0.0.3, la cible probante minimale du projet (voir `10_ROADMAP.md`).

---

## Entity et Agent

| | Entity (0.0.1) | Agent (0.0.3) |
|---|---|---|
| Nature | organisme sans cognition | entité promue, avec vie intérieure |
| État | `id, genome, position, energy, age`, 7 traits | tout ça, plus mémoire, besoins, personnalité, objectifs, relations |
| Décision | chimiotaxie : vers la meilleure ressource perçue, sinon errance | modèle de comportement déterministe qui lit la mémoire et les besoins |
| Coût | négligeable, tout le monde à chaque tick | modéré, seuls les agents saillants au grain fin (invariant 6) |
| LLM | jamais | jamais avant 0.0.5 (T-6) |

En 0.0.1, `03_DATA_MODEL.md` le dit : "une entité est un organisme sans cognition". Ce qui
n'est volontairement pas là (mémoire, personnalité, relations, LLM, culture) arrive à partir
de 0.0.3.

---

## La promotion Entity vers Agent

Une entité devient un agent quand elle franchit un seuil mécanisé sur ses capacités, jamais
un `if age > X` (T-7). Capacités visées : perception, mémoire, apprentissage, assez
développées pour qu'une vie intérieure ait du sens.

Propriétés de la promotion :

- **Détectée, pas devinée.** On définit le seuil, le monde le franchit, le moteur émet un
  événement saillant (dans l'esprit de `SpeciesEmerged`).
- **Réversible.** Un agent dont les capacités retombent redevient une entité de fond. La
  cognition n'est pas un aller simple.
- **Progressive.** Un agent n'est pas d'un coup un individu complet. Le niveau de
  simulation monte par paliers (statistique, comportemental, cognitif), comme la simulation
  différentielle le fait déjà pour l'importance.

C'est aussi une marche de l'escalier des échelles (`10_ROADMAP.md`) : au dessus,
l'organisme ; en dessous à partir de là, la biologie devient un état de fond (santé, âge,
génome) et la cognition passe au premier plan.

---

## Le substrat fixe de 0.0.3

Ce qui est construit en dur pour le jalon 0.0.3, sans LLM :

- **Moteur de mémoire.** Hiérarchique (résumé de vie, résumés d'ère, événements importants,
  souvenirs épisodiques). Chaque souvenir garde un `world_event_reference` vers le fait
  objectif : la mémoire subjective ne réécrit jamais l'histoire objective (invariant 5). La
  divergence entre souvenir et fait est mesurée, jamais corrigée.
- **Besoins.** Quelques jauges (faim, sécurité, social, ...) qui montent et descendent selon
  ce que l'agent vit.
- **Personnalité.** Un petit jeu de paramètres hérités (curiosité, sociabilité, prudence,
  ...), fixes pour la vie de l'agent, transmis avec mutation à la reproduction.
- **Modèle de comportement.** Déterministe. Il prend l'état courant, les besoins, les
  souvenirs pertinents, la personnalité, et produit une action. C'est lui qui rend la
  biographie lisible : on doit pouvoir dire "elle a évité cette zone parce qu'elle s'y était
  fait attaquer".

Critère de réussite du jalon (repris de `10_ROADMAP.md`) : le comportement dépend
visiblement d'un souvenir vérifiable, une biographie auto-générée tient debout, zéro LLM.

---

## Tranche 1 : le premier souvenir (fait, 2026-09-01)

Une chaîne verticale minimale, bout en bout, pour que la mémoire soit observable tout de
suite. Schéma v7. Config `[cognition]`.

**Éveil.** Une entité gagne un `Mind` (phase 5c de `sim.rs`, `cognition_phase`, séquentiel,
sans RNG) quand elle réunit trois conditions : `perception >= perception_min` (au dessus du
milieu de la plage de départ, donc sélectif, et le trait peut monter si être agent aide à
survivre), âge `>= age_min_frac` de l'espérance de vie (un juvénile n'a pas d'histoire), et
un choc récent à mémoriser. Événement `AgentAwoke` (saillance 215). C'est une marche de
l'escalier des échelles, détectée par seuil comme `SpeciesEmerged` ou `CellFormed`.

**Choc.** Toutes les entités (agents ou non) portent un `last_shock` (coût nul) : écrit en
phase 5 quand l'énergie tombe sous `peril_frac * energy_threshold` (péril) ou qu'un gain
dépasse `bounty_abs` (aubaine), espacé de `shock_interval`. C'est la graine d'un souvenir.

**Souvenir.** `Memory { formed_tick, place, kind: Peril | Bounty, event_seq: Option<u64>,
strength }`. Borné à `max_memories` (le plus faible cède la place), fusion des souvenirs de
même nature à moins de `memory_merge_dist` cases, décroissance `* memory_decay` par tick,
oubli sous `memory_eps`. En tranche 1 le péril (sa propre famine) n'a pas d'événement
source : `event_seq = None`, souvenir purement subjectif. Invariant 5 : un souvenir ancré
garde son `event_seq`, la divergence avec le fait se mesure, elle ne se corrige jamais.

**Comportement.** En phase 2/3 (décision, parallèle, lecture seule sur `mind`), un agent
non affamé laisse sa mémoire tirer sa cible de déplacement hors des lieux de péril, vers les
lieux d'aubaine : somme de noyaux gaussiens pondérés par la force, atténuée par la faim
(comme `hunger_damp` pour la cohésion) et modulée par une personnalité **dérivée des
traits** (`caution` de `lifespan`, `curiosity` de `perception`).

**Retombée.** Un agent dont la mémoire est vide depuis `lapse_ticks` (et passé le délai de
grâce) perd son `Mind` : `AgentLapsed` (saillance 195). La cognition n'est pas un aller
simple. Observé : dans un monde affamé, les agents meurent souvent avant d'oublier ; la
retombée est câblée et testée (invariant), elle se déclenche peu avec ces paramètres.

**Résultat (A/B graine 3, 60000 ticks, mémoire active vs `mem_weight = 0`).** Le
déplacement guidé par la mémoire réduit les morts par famine d'environ 15 % (13 100 contre
15 400), pour une population d'équilibre inchangée. Effet réel et doux, dans l'esprit de la
cohésion. La perception est déjà fortement sélectionnée vers le haut (moyenne ~0,92) : avec
les défauts, l'éveil devient courant en cours de partie (quelques centaines d'agents vivants
sur ~2300 entités). Rendre l'éveil plus rare ou le lier à une capacité plus discriminante
est un chantier de tranche 2.

## Tranche 2 : la page biographie (fait, 2026-09-01)

`lives.html` : la première biographie auto-générée, le livrable du jalon. Style éditorial
SVG comme `series.html`, généré en local. Aucun changement moteur ni de schéma : la CLI
échantillonne la vie de chaque agent (position, énergie, mémoire) toutes les 150 ticks
(`lives.jsonl` enrichi), garde la trajectoire complète pour les 80 vies les plus notables,
en met 24 en chapitre.

Chaque chapitre : une carte mémoire (la grille du monde, le chemin de l'agent, tous ses
lieux de péril et d'aubaine), une frise énergie plus force des souvenirs dans le temps, et
une prose de gabarits (aucun LLM) tirée des données : « Éveillé l'an 3 près de (67, 121)...
Au fil de sa vie il a retenu 5 lieux de péril... Après le péril de (113, 121), son chemin
s'en est tenu à distance : 2 cases de moyenne avant, 16 après. » C'est là que la dépendance
mémoire vers comportement se lit, agent par agent.

Observé sur `worlds/w2` : les vies notables retiennent 3 à 6 lieux de péril (jusqu'à 3 en
mémoire à la fois), presque aucune aubaine (l'aubaine, un gros gain en un tick, est rare) ;
beaucoup meurent de faim, aucune ne retombe (elles meurent avant d'oublier). Un coin de la
grille (~120, 122) revient souvent comme lieu de péril partagé : les agents repoussés des
zones centrales s'y échouent.

**Ce qui reste (tranche 3 et après).** Besoins (jauges), personnalité héritée (paramètres
au génome, pas dérivés), souvenirs ancrés sur un événement (mort d'un proche vue) avec le
lien `event_seq` vers le fait objectif, modèle de comportement complet, dé-simulation de la
biologie sous l'agent.

---

## Question ouverte : semé ou cultivé

À trancher à 0.0.3 avec des données (décision utilisateur du 2026-09-01 : laisser ouvert
pour l'instant).

**Option A, substrat semé.** Le moteur de mémoire et le modèle de décision sont construits
en dur, identiques pour tous les agents. Ce qui évolue : les paramètres de personnalité, et
plus tard la culture au dessus (mèmes, croyances, institutions). La règle du non-`if` porte
sur les résultats (mythes, guerres, institutions), pas sur le fait de donner un cerveau.

- Pour : rapide à construire, testable tôt, l'émergence se concentre là où le projet la
  veut (le social, le culturel).
- Contre : moins fidèle à "tout émerge des mécanismes". Le cerveau est un cadeau, pas une
  conquête.

**Option B, cerveau cultivé.** Des traits cognitifs eux-mêmes évoluent : capacité de
mémoire, forme de la règle d'apprentissage, portée et coût de la perception, profondeur de
planification. La cognition émerge de la sélection, comme le reste.

- Pour : fidèle au principe. Une vraie histoire de l'intelligence, pas une intelligence
  posée.
- Contre : beaucoup plus lent et incertain. Demande le modèle de temps à deux horloges
  opérationnel (`04_SIMULATION.md`) pour tourner assez de générations. Risque de ne jamais
  décoller.

Position probable : commencer en A pour atteindre la cible probante, garder B comme
expérience parallèle une fois l'horloge grossière en place. À confirmer à 0.0.3.

Note : même en option A, le LLM à partir de 0.0.5 est un organe cognitif acheté, pas simulé.
Il exprime (dialogue) et propose de la nuance (croyance, réinterprétation), il ne décide pas
la réalité du monde (invariant 1). L'intelligence d'expression est donc semée dans tous les
cas ; c'est l'identité, la mémoire, les objectifs et le social qui sont en jeu dans le choix
A ou B.

---

## Renvois

- `10_ROADMAP.md` : la place de 0.0.3 dans la feuille de route, l'escalier des échelles.
- `04_SIMULATION.md` : pourquoi l'option B a besoin de l'horloge grossière.
- `experiments/001_emergence.md` : le prototype d'émergence sociale (croyance partagée).
- `06_EMERGENCE.md` (à écrire) : l'expérience 004, le vrai test du pari avant 0.0.5.
- `02_ARCHITECTURE.md` : le module Agent, le module Memory, leurs frontières.

Ce document reste un squelette jusqu'à 0.0.3. Il sera étoffé avec le modèle de mémoire
détaillé, le format des souvenirs, et le modèle de comportement quand le code de 0.0.3
commencera.
