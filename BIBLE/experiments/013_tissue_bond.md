# 013 - L'adhesion persistante : des tissus qui tiennent

2026-09-04, v0.0.2. `[cells] tissue_bond` (defaut false, config seulement, schema inchange).

## Le constat

En direct, sur w2, l'utilisateur voit les tissus et les muscles se defaire : leurs cellules
"redeviennent" isolees au bout d'un moment. Pas un bug, un choix de conception qui montre ses
limites.

Un tissu n'etait pas un lien, c'etait un verdict recompose de zero chaque tick :
`tissue_pass` reconstruisait a chaque tick la composante connexe des cellules qui, *a cet
instant*, (a) ont leurs membranes a moins de `tissue_reach`, (b) des genomes a moins de
`tissue_kin`, (c) sont au moins `tissue_min`. Des qu'une condition lache, l'etiquette disparait.
Aucune colle ne persiste.

Quatre forces la font lacher :

1. **Rien ne tient vraiment les cellules ensemble.** La seule cohesion est `tissue_pull = 0,04`,
   tres molle, et elle ne pousse que quand il y a deja un ecart. Pendant ce temps les cellules
   encaissent : les entites errent, un predateur mange les entites de bord (la cellule retrecit
   et glisse), une cellule se divise et la fille derive.
2. **La derive genetique casse la parente.** Les cellules d'un tissu continuent de muter ;
   au fil des generations `trait_l1` depasse `tissue_kin` meme entre cellules qui se touchent.
3. **Le muscle se saborde.** Muscle = allongement >= `muscle_elong` (1,8). Or depasser
   `divide_elongation` (1,9) declenche la division, qui remet les filles a `elongation = 1` et
   casse la connexite locale. Le muscle ne peut pas tenir la forme qui le definit.
4. **Aucune hysteresis.** Les organismes ont recu une identite persistante ; les tissus non.

## Le mecanisme

`tissue_bond = true` : la connexite vient de **liens de paire gardes dans le temps**
(`WorldState.cell_bonds: Vec<(u32, u32)>`, paires triees d'ids, `#[serde(default)]`).

- **Nouer** : deux cellules parentes (`trait_l1 <= tissue_kin`) qui se touchent
  (`dist < (r1+r2) * bond_form`, def 1,15) et ne sont pas deja liees.
- **Casser** : seulement au-dela d'un **etirement franc** (`dist > (r1+r2) * bond_break`,
  def 2,4, soit plus du double de la distance pour nouer -> hysteresis) ou d'une derive
  genetique forte (`trait_l1 > tissue_kin * 1,8`), ou si une des deux cellules disparait.
- **Ressort** : entre les deux, `bond_stiffness` (def 0,12, plus ferme que `tissue_pull`)
  ramene les deux cellules vers le contact. Un lien etire tire vraiment.
- **Tissu** = composante connexe du graphe de liens, `>= tissue_min` cellules, id = plus petit
  id du groupe. `neigh` (donc psi6, abri) en decoule.
- **Resistance a la division** : le seuil d'allongement pour se pincer devient
  `divide_elongation + tissue_bonds * divide_bond_resist` (0,15/lien). Une cellule tissee par
  4 liens : seuil 2,5, il faut vraiment qu'elle s'etire. Une cellule libre ou de bord : seuil
  1,9 inchange. La cellule ancree est somatique, la cellule libre est germinale. Aucun `if` ne
  nomme un role.

Cout : le balayage O(n^2) de formation de liens est le meme que l'ancien test de distance
(~120 cellules). Sequentiel, sans RNG, ordre des id. Deterministe, replay byte-identique.

## Le test

`tissue_bonds_hold_a_tissue_through_perturbation` (graine 1, predation allumee comme
perturbation) :

- il se noue des liens ;
- la somme sur les ticks du nombre de cellules en tissu est **plus grande** avec les liens
  qu'avec la derivation tick a tick, meme graine ;
- `tissue_bond = false` ne garde aucun lien ;
- l'ecosysteme tient ; deterministe.

## A/B graine 1, 60 000 ticks, muscle allume des trois cotes (2026-09-04)

Base = config w2 (tous les leviers : predation, tissu, abri, organisme) + `muscle_contract`.
- **OFF** : `tissue_bond = false` (derivation tick a tick).
- **ON** : `tissue_bond = true`, `bond_stiffness = 0,12`, `divide_bond_resist = 0,15` (valeurs par defaut).
- **ON doux** : `bond_stiffness = 0,09`, `divide_bond_resist = 0,08` (le coeur peut encore se diviser).

Moyennes sur la 2e moitie du run ; tissus / psi6 / entites-en-cellule sur les 20 derniers points.

| Mesure | OFF | ON (defaut) | ON doux |
| --- | --- | --- | --- |
| population moyenne (plateau) | 8080 | 7160 | 7120 |
| population finale | 14749 | 12159 | 12026 |
| cellules vivantes, moyenne | 46,7 | 38,9 | 31,7 |
| entites en cellule (fin de run) | 3440 | 1960 | 2190 |
| tissus vivants (fin de run) | 2,9 | 3,8 | 3,5 |
| ordre du tissu psi6 (fin de run) | 0,24 | 0,50 | 0,46 |
| cellules formees (cumul) | 544 | 569 | 438 |
| divisions de cellule (cumul) | 59 | 22 | 19 |
| diversite genetique (finale) | 0,070 | 0,098 | 0,054 |
| morts par predation (cumul) | 107 900 | 102 400 | - |

Lecture :

- **Les liens font ce qu'on attend.** Les tissus persistent plus (2,9 -> 3,8 en nombre moyen) et
  surtout **s'ordonnent** : psi6 passe de 0,24 (liquide, phase hexatique) a 0,50 (nappe quasi
  hexagonale). C'est une transition de phase, pas une nuance. Le seuil "epithelium" de la vue
  (psi6 >= 0,50) est atteint pour de vrai.
- **La diversite genetique MONTE** (+40 %) avec les valeurs par defaut : sans les liens, une
  cellule de tissu se divise sans arret et amplifie clonalement la lignee dominante ; avec les
  liens et la resistance a la division, le coeur est somatique, il n'amplifie plus, la diversite
  se garde.
- **Le prix** : le coeur se divise beaucoup moins (divisions 59 -> 22), la biomasse
  pluricellulaire fond (entites-en-cellule -43 % en fin de run), la population moyenne perd
  ~11 %. Le tissu paie sa cote (logique du Cambrien : un tissu coute). C'est un tissu plus
  petit mais VRAI (ordonne, durable) au lieu d'un tissu plus gros mais ephemere.
- **ON doux** (`divide_bond_resist` 0,15 -> 0,08, `bond_stiffness` 0,12 -> 0,09) : **pas mieux,
  souvent pire.** Ne recupere pas la biomasse (entites-en-cellule 1960 -> 2190, toujours loin
  des 3440 d'OFF), fait CHUTER la diversite (0,098 -> 0,054, sous OFF) et former moins de
  cellules (569 -> 438). Le retrait de biomasse n'est donc PAS cause par la resistance a la
  division : il vient des liens eux-memes (une nappe qui tient occupe l'espace plus compactement,
  l'economie pluricellulaire se cale plus bas). Adoucir ne fait que degrader le reste. On garde
  les valeurs par defaut.

**Essai sur w2 en direct (2026-09-04) : bascule en cours de vie, echec, revenu en arriere.**
w2 est un monde age (an ~284, tick 2,49 M) dont le genome s'est adapte pendant 2,4 M ticks au
tissu DERIVE. Allumer `tissue_bond` (+ `muscle_contract`) en cours de vie a fait chuter la
lignee pluricellulaire en ~6000 ticks : cellules 25 -> 6, tissus 1-3 -> 0, biomasse en cellule
~900 -> effondree. Population globale non touchee (~10000, dans la respiration saisonniere de
disette). Le genome adapte a l'ancien regime ne supporte pas le choc : la resistance a la
division sterilise des cellules qui comptaient sur la division pour leur fitness, sans qu'une
selection ait eu le temps de recompenser l'ancrage. w2 remis en tissu derive.

**Conclusion.** Le mecanisme est valide (l'A/B graine 1, ne de la genese avec les liens,
montre une transition d'ordre nette et une diversite en hausse). Mais il ne s'introduit pas a
chaud sur un monde adapte autrement. Deux voies propres :
1. regenerer w2 depuis la genese avec `tissue_bond` des le depart (le genome co-evolue avec
   les liens), comme dans l'A/B ;
2. d'abord donner au tissu un BENEFICE de survie (`014`, epithelium barriere) pour que
   l'ancrage paie, PUIS l'allumer.
Graine 1 seulement pour l'A/B : refaire sur d'autres graines avant d'en faire un defaut.

## Suite

Ce qui manque encore : que le **type** de tissu compte (une nappe fait barriere, un muscle se
contracte en bloc, un adipeux tamponne). Voir `009_organism.md`, piste A. Lien :
[[organism-path-predation-first]].
