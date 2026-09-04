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

## A/B sur w2 (graine 1, 60 000 ticks, muscle allume des deux cotes)

<!-- REMPLIR APRES LE RUN -->

| Mesure | OFF (derive) | ON (liens) | Effet |
| --- | --- | --- | --- |
| population moyenne (plateau) | | | |
| population minimale | | | |
| cellules vivantes, moyenne | | | |
| entites en cellule, moyenne | | | |
| tissus vivants, moyenne | | | |
| ordre du tissu (psi6), moyenne | | | |
| cellules formees (cumul) | | | |
| cellules dissoutes (cumul) | | | |
| divisions (cumul) | | | |
| morts par predation (cumul) | | | |
| derive de cohesion | | | |

Lecture :

<!-- REMPLIR : les tissus persistants font-ils enfin emerger des muscles durables, des nappes
     qui bougent en bloc ? la resistance a la division localise-t-elle la reproduction au bord ?
     l'ecosysteme paie-t-il un prix ? -->

## Suite

Ce qui manque encore : que le **type** de tissu compte (une nappe fait barriere, un muscle se
contracte en bloc, un adipeux tamponne). Voir `009_organism.md`, piste A. Lien :
[[organism-path-predation-first]].
