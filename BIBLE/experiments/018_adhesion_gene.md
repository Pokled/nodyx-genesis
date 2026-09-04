# 018 - Le gene d'adhesion : un mecanisme reel, une selection trop diluee

2026-09-04, v0.0.2. Suite de `009_organism.md` (piste D, etape 1). `[cells] adhesion_gene`
(defaut false).

## Le probleme

Un organisme est aujourd'hui un pur accident geometrique : des cellules qui se touchent,
reconnues et nommees, mais sans rien qui les rende hereditairement distinctes les unes des
autres. La formation de tissu elle-meme (`tissue_pass`) est gouvernee par un seuil de parente
(`tissue_kin`) FIXE pour tout le monde -- aucune selection ne peut favoriser une lignee qui
adhere mieux qu'une autre. `experiments/009` (piste D, inspiree de Karl Sims 1994) envisageait
un second genome, structurel, distinct du genome de traits : le premier gene a construire est
`adhesion`, la tolerance a la parente pour adherer sans fusionner.

## Le mecanisme

`StructuralGenome { adhesion: f32 }` (genome.rs), un objet SEPARE de `Traits` -- surtout pas
un 11e trait dans `Traits::as_array()`, qui fausserait en silence l'echelle de `trait_l1`
(deja calibree pour `fuse_kin`, `tissue_kin`, `kin_dist` sur 10 dimensions). Le gene est la
TOLERANCE personnelle a la distance mesuree par `trait_l1`, pas une dimension de plus dans
cette mesure. Herite et mute exactement comme les traits (`Genome::divide`), moyenne par
cellule (`Cell.mean_adhesion`) aux trois sites de naissance/rafraichissement de cellule.

Dans `tissue_pass`, si `adhesion_gene`, le seuil `tissue_kin` d'une paire de cellules devient
`tissue_kin * mult`, ou `mult` interpole entre `adhesion_mult_min` (0,4) et `adhesion_mult_max`
(2,0) selon la moyenne d'adhesion des deux cellules -- au lieu du seuil fixe pour tout le monde.
`false` : comportement exactement inchange (le gene existe, mute, mais n'est jamais lu).

## Le diagnostic : ca marche, mais ca ne se selectionne presque pas

Verifie mecaniquement (comptage de cellules, plusieurs graines et seuils) : `adhesion_gene`
change reellement la formation de tissu -- 13 vs 22 cellules (graine 1), 5 vs 3, 11 vs 23, 5 vs 7
selon la graine et les bornes du multiplicateur. Le levier n'est pas un no-op, la trajectoire
diverge toujours (fingerprint different). **C'est le test retenu.**

Mais l'hypothese plus ambitieuse -- que la moyenne ponderee-population du gene DERIVE vers le
haut sous selection, parce que mieux adherer aide a survivre (comme `tissue_shelter` protege,
prouve dans `009`) -- ne tient pas empiriquement. Mesure sur plusieurs graines et plusieurs
reglages :

| Graine | `tissue_kin` | plage mult | derive (on - off) |
| --- | --- | --- | --- |
| 1 | 0,8 | 0,4-2,0 | +0,009 |
| 1 | 0,5 | 0,4-2,0 | -0,002 |
| 1 | 0,3 | 0,2-3,0 | +0,0025 |
| 1 | 0,2 | 0,1-4,0 | +0,0198 |
| 6 | 0,5 | 0,4-2,0 | 0,0000 (trajectoires IDENTIQUES, gene jamais decisif) |
| 6 | 0,3 | 0,2-3,0 | +0,0018 |
| 6 | 0,2 | 0,1-4,0 | -0,0004 |

Un ordre de grandeur sous tout ce qui a ete retenu cette session (nerve_relay +50 %, muscle_seek_food
+26 %, epithelium_shield +4,7 %) : jamais fiable, parfois nul, parfois legerement negatif.

**Cause identifiee, en deux temps :**

1. **La zone ou le gene compte est etroite.** Des cellules voisines dans un meme tissu
   descendent souvent d'une lignee proche (divisions recentes) : leur `trait_l1` est deja
   PETIT, largement sous le seuil quel que soit le multiplicateur. Le gene ne change une
   decision que pour les paires dont la distance tombe justement dans la bande entre
   `tissue_kin * mult_min` et `tissue_kin * mult_max` -- une minorite de cas.
2. **La variance exploitable s'effondre au niveau cellule.** La formation de cellule
   (fusion d'entites) est gouvernee par la parente de TRAITS (`fuse_kin`, `cohesion`), sans
   aucun rapport avec le gene d'adhesion. Les cellules regroupent donc des entites presque au
   hasard du point de vue de ce gene : `Cell.mean_adhesion` regresse vers la moyenne de
   population (ecart-type ~0,01-0,02 entre cellules, contre ~0,05 entre entites -- verifie par
   sondage direct). Un cell qui "gagne" au seuil d'adhesion n'est pas forcement composee des
   entites au gene le plus favorable : le benefice de survie profite a TOUS ses membres, pas
   specifiquement a ceux qui l'ont fait gagner. Un probleme classique de selection de groupe :
   sans regroupement assorti par le gene lui-meme, la moyenne de groupe est trop proche du bruit
   de population pour offrir une prise a la selection individuelle.

## Test

`adhesion_gene_changes_tissue_formation` : verifie seulement l'effet MECANIQUE (le levier
change reellement la formation de tissu, trajectoire deterministe qui diverge). Pas
d'assertion de derive genetique : ce serait un seuil instable, pas un invariant -- la lecon de
`016` ("verifier qu'un mecanisme fait vraiment quelque chose avant de juger s'il fait bien
quelque chose") s'applique ici a l'envers : le mecanisme fait bien quelque chose, mais pas la
chose esperee.

## Lecture

**Ni retenu comme "positif net" (comme `015`/`017`), ni un pur no-op (comme `016`) : un
troisieme cas.** Le code est correct et le mecanisme s'exerce reellement, mais l'architecture
choisie pour l'etape 1 (un gene individuel moyenne au niveau cellule, sans regroupement assorti)
ne donne a la selection presque rien a mordre. Garde dans le code (defaut `false`, aucun monde
affecte), documente honnetement.

**Pour une vraie selection sur ce genome structurel**, il faudrait soit (a) que la formation de
cellule elle-meme tienne compte du gene d'adhesion (regroupement assorti, mais ca melange les
deux couches que l'architecture voulait justement separer), soit (b) attendre l'etape 2 de la
piste D (carte de roles + reproduction a l'echelle de l'organisme entier) ou la selection
s'exercerait directement sur l'unite qui porte le genome structurel, pas sur une moyenne diluee
par des voisins non apparentes au gene.

Lien : [[organism-path-predation-first]], `009_organism.md`, `016_adipeux_reserve.md`.
