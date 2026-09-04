# 014 - L'epithelium qui compte : une nappe fait barriere (DESIGN, pas encore code)

2026-09-04, v0.0.2. Suite de `013_tissue_bond.md`. Statut : **conception, a valider avant code.**

## Le probleme

Les types de tissus sont LUS (`genesis-view::tissue_kind`) mais ne COMPTENT pas : un epithelium
bien pave se lit "epithelium" a l'ecran, mais rien dans le moteur ne le distingue d'un amas
quelconque. Un vrai epithelium est d'abord une **frontiere** : il separe un dedans d'un dehors,
il retient, il protege ce qu'il enferme. C'est le premier pas concret vers l'organe : un
organisme avec un epithelium a un **interieur**.

## L'idee : les liens sont des murs

`013` a introduit `WorldState.cell_bonds` : des liens de paire persistants entre cellules qui
adherent. Un lien entre deux cellules d'une **nappe** (chacune tissee par >= 3 liens : c'est une
feuille, pas une chaine) definit un **segment de mur** entre leurs deux centroides.

`[cells] epithelium_seal` (defaut false, config seulement) :

1. Une fois par tick, construire les segments de mur a partir de `cell_bonds` retenus : les deux
   cellules dans le meme tissu, chacune avec `tissue_bonds >= seal_min` (def 3).
2. Ranger les segments dans une grille grossiere (comme `SpatialHash`, sur le milieu du segment).
3. Apres la phase 4 (mouvement) : pour chaque entite **libre** (`cell_id == None`), si son
   deplacement du tick `[position_avant, position_apres]` **croise** un segment de mur, la
   ramener juste en deca du croisement, cote depart. Les membres de cellules ne sont pas
   bloques (ils constituent le mur, ils glissent le long).
4. Borne : on ne teste que les entites proches d'un mur (grille), un seul mur bloquant par
   entite et par tick, deplacement jamais augmente (seulement raccourci).

Cout : un instantane `prev_pos` avant la phase 4 (une allocation `Vec<Position>`), plus un test
de croisement segment/segment par entite libre contre ~quelques murs locaux. Sequentiel apres
la phase 4, sans RNG, ordre des id. Deterministe.

## Ce que ca produit (laisse a l'emergence)

- Une boucle epitheliale fermee **piege** les entites libres a l'interieur (un lumen) ou les
  **tient dehors**. L'organisme gagne une cavite.
- Un predateur ne peut plus traverser une nappe pour atteindre les cellules a l'abri
  (`tissue_shelter` rendait la cellule interieure non-ciblable, mais le predateur pouvait
  quand meme entrer). Barriere + abri = une vraie poche protegee.
- Une nappe incurvee qui se referme sur des ressources : une premiere "bouche" / poche
  digestive, si les entites piegees s'y font consommer. Non code, observe.

## Doctrine

Aucun `if kind == "epithelium"`. La regle porte sur la geometrie : un lien dans une feuille
dense (lien + `tissue_bonds >= seal_min`) arrete les entites libres qui voudraient le franchir.
Que cette feuille soit "un epithelium" est la lecture de la vue, pas la cause.

## Risques a surveiller

- **Performance** : au pic de population (~15000 entites), le test de croisement doit rester
  dans la grille locale, jamais O(entites x murs).
- **Piege mortel** : des entites enfermees sans ressource meurent de faim en masse -> creux de
  population. A/B a mener : la barriere doit ajouter une structure, pas decimer.
- **Entites coincees dans le mur** : si une entite est pile sur un segment, la projection doit
  la pousser d'un cote franc, jamais osciller. Tiebreak deterministe (cote du depart).

## Alternative plus simple si le collision-test coute trop

`epithelium_shield` : pas de collision, juste une extension de `tissue_shelter`. Une entite
libre a l'interieur du **polygone** d'un tissu scelle (test point-dans-polygone sur l'enveloppe
convexe des cellules du tissu) est comptee "abritee" : hors predation. Moins riche (pas de
lumen, pas de piege), mais O(entites x tissus) et pas de snapshot de position.

Lien : [[organism-path-predation-first]], `013_tissue_bond.md`, `009_organism.md`.
