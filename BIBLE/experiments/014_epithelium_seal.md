# 014 - L'epithelium qui compte : une nappe digere ce qu'elle enserre (essai negatif)

2026-09-04, v0.0.2. Suite de `013_tissue_bond.md`.
**Statut : essaye (`[cells] epithelium_seal`), A/B negatif, CODE RETIRE. L'implementation est
dans l'historique git ; ce document garde le pourquoi.**

## Le probleme

`013` a montre que `tissue_bond` fait des tissus qui TIENNENT et s'ORDONNENT, mais qui coutent :
une cellule tissee se divise beaucoup moins (resistance a la division), la lignee pluricellulaire
retrecit. Un tissu qui ne rapporte rien est selectionne contre. Il faut que porter un tissu
**paie**. C'est aussi la logique du Cambrien : le multicellulaire persiste quand il donne un
avantage, pas avant.

Par ailleurs les types de tissus sont LUS (`genesis-view::tissue_kind`) mais ne COMPTENT pas :
"epithelium" est une etiquette a l'ecran, rien dans le moteur ne le distingue d'un amas.

## Le mecanisme (FAIT)

Une nappe **ordonnee et assez grande** (psi6 du tissu >= `seal_order` def 0,42, >= `seal_cells`
def 5 cellules) **digere les entites libres a sa portee**. A chaque tick, pour chaque cellule
d'une telle nappe :

- on regarde les entites LIBRES (`cell_id == None`) a moins de `seal_reach` x rayon de la cellule
  (`SpatialHash`, borne local) ;
- on prend `seal_drain` (def 0,05) de l'energie de chacune (une prise par entite et par tick) ;
- cette energie va aux membres de la cellule, repartie egalement.

Conserve (l'energie se deplace, elle n'apparait pas), sans RNG, ordre des id de cellule. Tourne
dans `tissue_pass`, apres le calcul du psi6 par tissu (accumule dans une map pendant la boucle
psi6 existante). `psi_by_tissue` : (somme psi6, nombre de cellules) -> une nappe scelle si
`somme / nombre >= seal_order`.

Cout : une `SpatialHash` des entites (comme `muscle_pass`), un balayage local par cellule de
nappe scellee. Les nappes scellees sont rares (il faut 5+ cellules ordonnees). Deterministe,
replay byte-identique, `#[serde(default)]`, schema inchange.

## La doctrine

Aucun `if kind == "epithelium"`. La regle porte sur la geometrie (une nappe ordonnee) plus
l'acte physique d'enserrer (une entite libre a portee de la surface du tissu). Que cette nappe
soit "un epithelium" ou "une poche digestive" est la lecture de la vue, pas la cause. Ce qui
emerge (une nappe qui se courbe autour d'une zone riche et s'en nourrit, un organisme qui a un
dedans qui digere) est laisse a l'emergence.

## Ce qu'on attendait (et qui n'arrive pas)

- Une nappe qui se referme sur une zone riche aurait un **revenu** : cellules gorgees, bords
  assez nourris pour croitre, tissu qui grandit par ses bords, finance par la digestion.
- Le tissu **paierait sa cote** : `entites-en-cellule` remonterait vers le niveau OFF.

C'est l'inverse qui se produit (voir A/B).

## A/B graine 1, 60 000 ticks -- RESULTAT NEGATIF

Base = config w2 + `muscle_contract` + `tissue_bond` (defauts). `seal_drain = 0,05`,
`seal_order = 0,42`, `seal_cells = 5`.

| Mesure | OFF (tissu derive) | tissue_bond seul | + epithelium_seal |
| --- | --- | --- | --- |
| population moyenne (plateau) | 8080 | 7160 | **6790** |
| population finale | 14749 | 12159 | 12200 |
| cellules vivantes, moyenne | 46,7 | 38,9 | 37,7 |
| entites en cellule (fin de run) | 3440 | 1960 | **1630** |
| tissus vivants (fin de run) | 2,9 | 3,8 | 3,4 |
| ordre du tissu psi6 (fin de run) | 0,24 | 0,50 | **0,29** |
| divisions de cellule (cumul) | 59 | 22 | 15 |
| cellules formees (cumul) | 544 | 569 | 626 |
| diversite genetique (finale) | 0,070 | 0,098 | **0,068** |

Lecture :

**La digestion ne compense pas le cout du tissu, elle l'aggrave.** Presque toutes les mesures
reculent par rapport a `tissue_bond` seul :

- **L'ordre du tissu s'effondre** (psi6 0,50 -> 0,29). C'est le contraire du but. Hypothese : le
  revenu d'energie nourrit les cellules de la nappe, qui grossissent et s'agitent plus ; or dans
  le modele KTHNY l'activite cellulaire = temperature effective, et elle **fait fondre l'ordre**.
  On finance le tissu, le tissu chauffe, il se descelle, la digestion s'arrete. Boucle qui
  s'auto-annule.
- **La biomasse pluricellulaire baisse encore** (1960 -> 1630) et la **diversite retombe au
  niveau de base** (0,098 -> 0,068). En ponctionnant les entites libres autour des nappes, on
  asseche le vivier qui forme les nouvelles cellules et qui porte la variete genetique. Plus de
  cellules se forment (626) mais elles ne tiennent pas (churn).
- `seal_drain = 0,05` par tick est probablement trop dur (une entite proche perd la moitie de
  son energie en ~14 ticks), mais la direction est claire meme sans reglage fin.

**Conclusion : abandonne sous cette forme.** Le benefice d'un tissu ne doit pas etre de
l'energie brute (ca ajoute de l'activite, ca melte l'ordre) ni un prelevement sur les libres (ca
asseche le vivier). Une meilleure piste : un benefice qui ne rend pas la cellule plus active --
immunite de predation etendue a toute la nappe scellee (pas seulement au coeur), ou une nappe
qui retient la ressource sous elle (un garde-manger), ou la digestion dirigee vers la
REPRODUCTION (un membre de plus) plutot que vers l'energie des membres existants. Le flag reste
(defaut false, teste) mais n'est pas une voie.

Lien : [[organism-path-predation-first]], `013_tissue_bond.md`, `009_organism.md`.
