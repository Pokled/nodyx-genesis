# 006. Experience, la bascule molecule vers cellule

Statut : **etape 1 (membrane) implementee et active** (2026-09-01). Jalon 0.0.2, tranche 2.
Gabarit de `001_emergence.md`.

Position : la premiere marche de l'escalier des echelles (`10_ROADMAP.md`). L'utilisateur a
choisi de la monter en deux temps :
- **Etape 1, la membrane (ici)** : les entites restent simulees, taguees d'un `cell_id` ;
  un objet `Cell` les agrege, detecte la formation, applique un avantage de groupe, le
  lecteur dessine une membrane. Aucun invariant casse.
- **Etape 2, la de-simulation** (plus tard) : les membres quittent `world.entities`, la
  cellule devient un pur bilan. Retravaille les invariants de conservation. Plan a part.

---

## Question

Le trait `cohesion` existe depuis l'experience 003 mais son effet est « reel mais doux » :
la retenue sur les communs ne le fait monter que faiblement. La bascule cellule doit lui
donner un vrai avantage.

1. Un amas coherent de parents devient-il une **cellule detectee**, de facon mecanisee
   (jamais un `if` qui la nomme, T-7) ?
2. L'avantage de groupe (partage d'energie + reproduction protegee) fait-il **monter
   `cohesion` par selection** ?
3. Sans destabiliser l'ecosysteme (contraste attendu avec la force de mouvement de 003) ?
4. De facon **deterministe** et **reversible** (une cellule qui se disperse se dissout) ?

## Montage

Nouvelle phase 5b (`sim.rs`), entre le metabolisme et la mort.

- **Detection** (tous les `[cells] check_every` ticks) : `SpatialHash` sur les entites
  libres, composantes connexes par union-find (lien = proximite <= `link_dist` ET distance
  L1 de genome <= `kin_dist`). Une composante est candidate si taille >= `min_members`,
  dispersion <= `max_spread`, cohesion moyenne >= `min_cohesion`. Persistance :
  `Watch.cell_pending` suit les centroides des candidats ; apres `persist_checks` controles
  tenus, la cellule se forme (`CellFormed`).
- **Entretien** (chaque tick) : bilan rafraichi (centroide, rayon, effectif, traits
  moyens), **partage d'energie** (chaque membre tend vers la moyenne de la cellule,
  `energy_share`), depart d'un membre trop loin du centre, **dissolution** si trop petite,
  trop dispersee, ou cohesion moyenne retombee (`CellDissolved`).
- **Avantage** : partage d'energie (ci-dessus) + phase 7, un membre de cellule voit son
  `birth_loss` multiplie par `(1 - cell_birth_relief)`.

Deterministe : phase 5b sequentielle, `HashMap` en lecture seule, union-find a regle fixe,
iteration sur `BTreeMap` et tranches triees. Aucun nouveau tirage RNG.

Schema v5 (`WorldState.cells`, `Entity.cell_id`, `Watch.cell_pending`).

## Resultat, etape 1 (2026-09-01)

A/B a la meme graine, 12 graines, `ticks = 40000`. ON = defaut. OFF = `min_members` tres
grand (aucune cellule ne se forme).

| Graine | ON pop | ON cohesion | cellules formees | OFF pop | OFF cohesion |
|---|---|---|---|---|---|
| 1 | 2253 | 0,578 | 321 | 2226 | 0,582 |
| 3 | 2252 | **0,647** | 328 | 2274 | **0,368** |
| 5 | 2278 | 0,377 | 8 | 2260 | 0,302 |
| 6 | **723** | 0,638 | 156 | **308** | 0,668 |
| 11 | 549 | 0,635 | 152 | 629 | 0,650 |
| 2, 4, 7, 8, 9, 10, 12 | 0 | | 0 | 0 | (extinctions identiques) |

Lecture :
- **La selection joue, fort, sur au moins une graine** : graine 3, cohesion 0,65 avec
  cellules contre 0,37 sans. Meme graine, meme genome de depart : un ecart de 0,28, tres au
  dela du bruit. C'est de la selection.
- **L'avantage de survie est reel** : graine 6, la population ON tient a 723 contre 308
  sans (la reproduction protegee sauve le monde).
- **Effet non universel** : graines 1 et 11, la cohesion ON et OFF sont proches. Les
  cellules se forment mais churnent (beaucoup formees, peu vivantes a un instant donne) et
  la selection ne s'accroche pas. Frequence-dependant, honnete.
- **Aucune destabilisation** : les 7 extinctions sont identiques ON et OFF. Contraste total
  avec la force de mouvement de 003 (~0/16 survie).
- **Determinisme** : snapshots et journaux byte-identiques 1 vs 8 threads. Rejeu `w2` : OK.
- **Consistance** : test `cells_stay_consistent`, aucune cellule ne reference une entite
  morte, effectifs coherents, invariant de population inchange.

Verdict : le mecanisme fait ce qu'on lui demande. Detection mecanisee, reversible,
deterministe, sans casse. La cohesion devient adaptative la ou les cellules tiennent. Base
honnete. Le churn (cellules fragiles) est le point durci en V2.

## Resultat, V2, cellules durcies (2026-09-01)

Choix utilisateur : reduire le churn. Trois leviers, tous dans la phase 5b, aucun sur le
mouvement (la force de mouvement de 003 reste eteinte) :
- **hysteresis de dissolution** : une cellule formee ne se dissout qu'en dessous de
  `dissolve_members` (6, pas `min_members` = 12), au dela de `dissolve_spread` (11, pas
  `max_spread` = 6), ou sous `min_cohesion * 0.7`. Sur le modele du detecteur d'espece.
- **delai de grace** : `grace_ticks` (600) pendant lesquels une cellule fraiche ne peut pas
  se dissoudre, sauf si elle tombe a zero membre.
- **formation plus stricte** : `persist_checks` 3 -> 4, `leave_factor` 1.6 -> 1.9.

Effet (graines 1, 3, 5, 6, 11, 40000 ticks) :
- **Churn divise par 2 a 4** : graine 3, 328 cellules formees -> 91 ; graine 1, 321 -> 180.
- **Cellules bien plus stables** : cellules vivantes en fin de run, graine 3 : 11 -> 34 ;
  graine 1 : 3 -> 39. Entites en cellule : graine 3, 349 -> **1419** sur ~2276 (62 %) ;
  graine 1, ~300 -> **1788** sur 2268.
- **Selection de cohesion toujours claire ou elle bite** : graine 3, 0,61 avec contre 0,37
  sans ; graine 5, **0,46 contre 0,30** (V1 n'avait presque aucun ecart ici). Graines 1 et
  11 restent proches (les cellules y sont communes mais pas assez discriminantes).
- **Survie encore meilleure** : graine 6, population 723 (V1) -> **1022** (V2).
- Determinisme byte-identique 1 vs 8 threads. `worlds/w2` (60k ticks) : 27 cellules
  vivantes, 1364 entites en cellule sur 2255, 202 cellules formees sur la duree (contre 893
  en V1). Conservation et rejeu : OK.

Verdict V2 : les cellules sont maintenant des structures durables (60 % de la population y
vit sur `w2`) au lieu d'un scintillement. La cohesion reste adaptative la ou l'avantage
discrimine. On garde V2 par defaut.

## Livrable

`worlds/w2` (graine 3, 60k ticks) : monde de reference, panneau « Cellules » et membranes
dans le lecteur, chapitres « cellule ». A/B reproductible par `[cells] min_members = 100000`
pour le bras OFF.

## Etape 2, a faire

De-simulation des membres. `WorldState.population()` devient `entites libres + somme des
effectifs de cellule` ; `structural_matter_is_conserved` et `population_is_conserved`
gagnent un terme cellule ; toutes les stats de `world.rs` replient les cellules ; un
`CellView` porte deja le necessaire pour le lecteur. Un plan dedie.
