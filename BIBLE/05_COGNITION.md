# 05. Cognition

Statut : TRANCHES 1 A 6 FAITES (le premier souvenir, la biographie, les souvenirs ancrés,
les besoins, la personnalité héritée, le mode de comportement lisible). La mémoire
épisodique tourne, se lit dans `lives.html`, un souvenir de mort vue pointe le fait
objectif, l'agent a un état interne (faim, peur, solitude), une personnalité (prudence,
curiosité) transmise avec mutation, et sa biographie dit quelle force a dominé chaque
décision. L'architecture détaillée (Context Builder, Model Router, RAG) attend 0.0.5.

**Monde de référence : `worlds/w2` = graine 1 depuis le schéma v10.** Le passage à 9 traits
a décalé le flux RNG ; la graine 3 (référence des tranches 1 à 4) s'éteint désormais tôt.
Les mesures A/B ci-dessous marquées « (v9, graine 3) » portent sur un monde qui n'existe
plus ; l'état consolidé en bas de section est sur la graine 1, schéma v10.

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

## Tranche 3 : le souvenir ancré (fait, 2026-09-01)

En tranches 1 et 2, tous les souvenirs sont subjectifs : le péril, c'est la propre famine de
l'agent, `Memory.event_seq = None`. Le critère du jalon parle d'un souvenir **vérifiable**.

Schema v8. Nouveau genre `MemoryKind::Witnessed` : un lieu où l'agent a vu mourir un des
siens. Formation en phase 6 de `sim.rs`, après le retrait des morts (pas d'auto-témoignage),
séquentiel, sans RNG : pour chaque `EntityDied` du tick, tout agent à moins de
`witness_radius` (défaut 4) et de la même lignée fondatrice (`witness_kin_only`) enregistre
un `Memory { kind: Witnessed, event_seq: Some(seq de l'EntityDied), strength: 1.0 }`. Le
souvenir se comporte comme un péril pour le biais de déplacement (il repousse, modulé par la
prudence). `MemoryKind::is_aversive()` regroupe péril et mort vue.

Invariant 5 : le souvenir porte le `seq` du fait, il ne le réécrit pas. La biographie peut
alors écrire « au tick 45 000, il a vu mourir un des siens près de (113, 113), événement
191 641 ; il s'en est ensuite tenu à distance : 6 cases de moyenne avant, 37 après » ; le
numéro se retrouve dans `events.jsonl`. C'est la boucle fermée avec la traçabilité causale
de 0.0.2 (les `seq` attribués à la création, tranchée 15).

**Résultat (A/B graine 3, 60000 ticks, `witness_radius` défaut vs 0).** Les morts par famine
tombent de 13 100 à 8 700 (**-33 %**, en plus du -15 % de la tranche 1), les morts d'âge
montent un peu (les agents vivent plus longtemps), la population d'équilibre ne bouge pas,
le nombre d'agents vivants passe de ~300 à ~530. Voir mourir les siens et retenir le lieu
est fortement adaptatif : les agents évitent les grappes de mortalité. Effet secondaire : la
pression sur la perception se relâche un peu (perception moyenne 0,92 -> 0,88). Déterministe
byte-identique 1 vs 8 threads. Test `witnessed_memories_are_anchored`.

## Tranche 4 : les besoins (fait, 2026-09-01)

Le comportement d'un agent était un réflexe : viser la nourriture perçue, s'écarter des
lieux de sa mémoire. Il a maintenant un **état interne**. `Mind.needs` (schema v9), trois
jauges dans [0, 1], mises à jour en phase 5c de `sim.rs`, sans RNG :

- **faim** : suit vers le haut le manque d'énergie instantané, se relâche lentement
  (`hunger_relief`). Un agent récemment affamé reste tendu vers la nourriture même à énergie
  correcte.
- **peur** : monte près d'un souvenir aversif (noyau gaussien, `fear_radius`) et après un
  choc de famine récent (`fear_shock_window`), se relâche lentement (`fear_relief`).
- **solitude** : `1 - support_de_colonie / support_cap`, déjà calculé sur l'entité (aucune
  requête spatiale nouvelle).

En phase 2/3 (décision, parallèle, lecture seule), les besoins pondèrent la cible : `drive =
1 - faim` remplace l'indicateur d'énergie (affamé -> fonce manger, ignore la mémoire
d'aubaine) ; le gate d'évitement des souvenirs aversifs est `max(drive, peur)` (un agent
effrayé fuit le danger **même affamé**) et la peur amplifie cet évitement (`fear_gain`) ;
un agent isolé glisse vers le centre de masse des siens (`social_pull`). Bouton maître
`needs_weight` (défaut 1) : à 0, le comportement est exactement celui de la tranche 3.

**Résultat (A/B graine 3, 60000 ticks, `needs_weight` 1 vs 0).** Les morts par famine
tombent de 8 700 à 4 000 (**-54 %**, après les -15 % et -33 % des tranches 1 et 3). Les
morts d'âge doublent (850 -> 1 600) : la population meurt maintenant surtout de vieillesse.
Agents vivants ~520 -> ~790. La perception moyenne descend encore (0,91 -> 0,83) : plus la
cognition porte la survie, moins la perception brute pèse ; la cohésion monte un peu (0,51
-> 0,54, la solitude récompense la proximité des siens). Déterministe byte-identique 1 vs 8
threads, `needs_weight = 0` reproduit la tranche 3 à l'octet près. Test `needs_stay_bounded`.

`lives.html` : une troisième figure par chapitre (les trois jauges dans le temps) et une
phrase de tempérament (« Il a vécu surtout affamé, sur ses gardes... »). Les chapitres
mettent en avant les survivants (mémoire riche = a duré), leurs jauges sont donc calmes ;
c'est la population large qui montre l'effet (l'A/B).

## Tranche 5 : la personnalité héritée (fait, 2026-09-01)

`caution` et `curiosity` étaient dérivés des traits de corps (`caution = f(lifespan)`,
`curiosity = f(perception)`). Ils deviennent deux vrais traits du génome, indices 7 et 8 :
`N_TRAITS` 7 -> 9, schéma v10. Transmis avec mutation par `Genome::divide` comme les autres,
donc hérités et soumis à sélection sans code en plus. La signature d'espèce
(`genome_key`, `SPECIES_TRAITS = 7`) ne porte que sur les traits de corps : une population
qui ne diverge qu'en tempérament n'est pas une espèce distincte.

En phase 2/3, la personnalité vient du génome : `caution_eff = 0.25 + 0.7 * traits.caution`
(idem curiosity), sinon, si `heritable_personality` est faux, les anciennes formules
dérivées (le génome porte les deux traits dans les deux cas, le flux RNG est identique :
l'A/B est propre).

**Résultat (A/B graine 1, 60000 ticks, v10).**
- **héritée vs dérivée** : morts par famine 4 300 contre 5 100 (-16 %), agents vivants
  1 620 contre 1 560. Laisser le tempérament flotter libre plutôt que l'attacher au corps
  donne une petite avance : la population trouve son propre équilibre.
- **la prudence gagne-t-elle ?** Non, pas nettement. Sur toute la course, `caution` et
  `curiosity` restent près de 0,5, la bande p10-p90 tient dans ~0,48-0,53 : les deux traits
  sont **quasi neutres**, ils dérivent, ils ne balaient pas. Avec ces paramètres, la peur
  et les besoins portent déjà l'évitement ; la variation individuelle de prudence ne creuse
  pas d'écart de survie assez grand pour être sélectionnée, et une prudence haute a un coût
  (moins de nourriture). C'est un résultat, pas un échec : le tempérament est un degré de
  liberté que la sélection ne contraint presque pas ici.

Le brin d'ADN du lecteur passe à 9 barreaux, `series.html` trace 9 courbes de traits.

---

## État consolidé de la cognition (schéma v11, graine 1)

A/B graine 1, 60000 ticks : cognition complète (mémoire + ancré + besoins + personnalité)
contre `mem_weight = 0` (aucune cognition, comportement 0.0.2). Chiffres revérifiés au
schéma v11 (le mode de comportement de la tranche 6 ne change pas la trajectoire).

| | cognition complète | aucune cognition |
|---|---|---|
| morts par famine | **4 300** | 25 800 |
| morts d'âge | 2 250 | 46 |
| agents vivants | ~1 600 | ~190 |
| perception moyenne | 0,75 | 0,95 |
| générations en 60k ticks | 17 | 27 |

La cognition **divise les morts par famine par six** et fait basculer la population d'un
régime limité par la famine à un régime limité par la vieillesse. Effet le plus net sur le
génome : sans cognition la perception est sélectionnée à fond vers 1,0 ; avec, elle se
stabilise à 0,75, parce que la mémoire porte la survie et que percevoir loin devient moins
vital. Le monde sans cognition tourne plus de générations (mortalité forte = renouvellement
rapide).

## Tranche 6 : le mode de comportement lisible (fait, 2026-09-01)

Le comportement d'un agent est un mélange de forces (chimiotaxie vers la nourriture, mémoire
aversive qui repousse, aubaine qui attire, glissement vers les siens si isolé). Utile, mais
opaque : on ne pouvait pas dire *ce qu'il a choisi*.

Schéma v11. `Mind.mode: BehaviorMode` (`forage | flee | join | seek_bounty | wander`).
`blend_target` renvoie maintenant la cible **et** le mode qui a le plus pesé sur la
décision : magnitude du déplacement dû à la mémoire aversive (fuir), à l'aubaine (chercher),
au glissement social (suivre), sinon manger (ou errer si aucune nourriture perçue).
**C'est une lecture, pas un changement de comportement** : la trajectoire est byte-identique
à la tranche 5 (vérifié, mêmes morts par famine à l'unité). Coût zéro, légèreté totale.

`lives.html` : une quatrième figure par chapitre (bandes colorées du mode dans le temps) et
une phrase « Côté décisions : 84 % du temps à chercher à manger, le reste à fuir 16 % ». Le
critère du jalon (« on doit pouvoir dire *pourquoi* ») devient littéral.

Approche essayée puis abandonnée : un vrai modèle d'utilité (l'agent évalue chaque mode et
prend le meilleur). Il était lisible mais moins efficace que le mélange (deux fois plus de
morts par famine) : le choix discret abandonne les autres pulsions. La lecture-du-mélange
garde la performance ET la légèreté.

**Ce qui reste (tranche 7 et après).** Souvenirs sociaux (reconnaître un autre agent),
dé-simulation de la biologie sous l'agent.

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
