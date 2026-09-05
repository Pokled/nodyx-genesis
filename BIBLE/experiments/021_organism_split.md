# 021 - La reproduction d'organisme : enfin l'unite qui se multiplie sans payer cher

2026-09-05, v0.0.2. Suite de `020_role_share.md` (piste D, etape 3). `[organism] split_enabled`
(defaut false).

## Le probleme laisse par les etapes 1-2

`018`/`019`/`020` ont tous buté sur le meme mur : un gene individuel qui doit influencer une
consequence a l'echelle de la CELLULE finit soit dilue (moyenne, `018`), soit couteux
(consequence sur la reproduction de l'entite, `019`/`020`). Le diagnostic de `020` pointait vers
la sortie : faire que la selection s'exerce sur l'UNITE qui se reproduit vraiment, sans passer
par un intermediaire dilue. Pour ca, il faut d'abord que l'organisme lui-meme se reproduise --
aujourd'hui il persiste ou se dissout, il ne se MULTIPLIE jamais.

## Le mecanisme

Un organisme qui atteint `split_cells` cellules se scinde en deux : ses cellules sont projetees
sur l'axe de plus grande dispersion (`cloud_shape`, exactement la technique deja utilisee par la
division de cellule) et reparties en deux moities egales. La moitie qui reste garde l'id et le
nom du parent (continuite) ; la moitie qui part recoit un id et un nom neufs
(`names::organism_name`), une vraie naissance d'organisme. Deterministe, sans RNG, ordre des id
d'organisme puis de cellule. Purement structurel pour cette premiere tranche : **aucun genome
d'organisme ni consequence genetique encore cablee** -- l'objectif ici est seulement de prouver
que l'unite peut se multiplier sans casser l'ecosysteme, avant d'y accrocher une heredite.

## Verification mecanique : ca marche, et ca ne coute presque rien

Sonde (graine 1, tissu + organisme, `split_cells = 8`, config compacte pour observer plusieurs
generations en 60 000 ticks) :

| Mesure | sans `split_enabled` | avec `split_enabled` |
| --- | --- | --- |
| `organisms_formed_total` (cumul) | 71 | **213** (x3) |
| population finale | 9 572 | 9 575 (quasi identique) |
| organismes vivants au pic | 11 | 16 |
| taille max d'organisme atteinte | 48 | 30 (plafonnee par la scission, attendu) |

Les scissions ont lieu massivement (142 evenements de plus que la seule reconnaissance
naturelle) sans que la population ne bouge d'une virgule.

## A/B a l'echelle w7 (toute la pile organe), graine 40, 60 000 ticks

| Mesure | sans split | avec split | ratio |
| --- | --- | --- | --- |
| population finale | 14 899 | 14 900 | 1,00 (identique) |
| population moyenne au plateau | 9 098 | 8 893 | 0,977 |
| entites en cellule, moyenne | 2 784 | 2 490 | 0,895 |
| cellules vivantes, moyenne | 56,7 | 52,2 | 0,921 |
| tissus vivants, moyenne | 6,07 | 5,62 | 0,927 |
| ordre du tissu psi6, moyenne | 0,330 | 0,314 | 0,952 |
| diversite genetique (fin de run) | 0,057 | 0,048 | 0,837 |
| organismes vivants (fin de run) | 12 | **14** | +17 % |

**Le plus doux de tous les mecanismes de la piste D essayes cette session sur la population
finale** (ratio 1,00, contre 0,61-0,94 pour le blocage dur de role et 0,20-0,73 pour le partage
doux). Un leger recul sur les mesures de structure/diversite (5 a 16 %), coherent avec le fait
que les organismes sont plus petits en moyenne (plafonnes par la scission) et donc un peu moins
stables individuellement -- mais sans commune mesure avec le cout observe pour `019`/`020`, et
la METRIQUE QUI COMPTE pour cette piste (le nombre d'organismes vivants) est en HAUSSE, pas en
baisse.

## Test

`organism_split_multiplies_organisms_without_ecological_cost` : le levier multiplie reellement
les organismes (`organisms_formed_total` plus du double du temoin) ; toute cellule pointant un
organisme pointe un organisme vivant (pas de reference pendante apres une scission) ;
l'ecosysteme tient des deux cotes, avec un ratio de population > 0,7 (contre le seuil bien plus
severe qu'aurait echoue `role_reproduction_gate`) ; deterministe.

## Lecture

**Premier franchissement reel de l'etape 3 : l'organisme peut enfin se multiplier, et ca ne
coute presque rien.** Contrairement a `018`-`020`, rien ici ne force un compromis entre variance
et cout -- la structure se reproduit proprement. Ce que cette tranche NE fait PAS encore : elle
ne porte aucune heredite. Les deux organismes issus d'une scission ne different en rien
(memes cellules, memes membres, juste separes) -- il n'y a pas encore de "genome d'organisme"
qui pourrait deriver sous selection entre generations d'organismes.

**Note (2026-09-05, `022_organism_split_gene.md`)** : la tranche suivante a bien ete tentee --
`Organism.split_bias`, un seuil de scission propre a chaque organisme, herite et mute a chaque
scission. Le mecanisme est sain (le gene existe et varie reellement), mais la preuve de
selection reste hors de portee a cette echelle de test : trop peu d'organismes vivent
simultanement pour un echantillon statistique fiable en 60-80k ticks. Une limite de mesure, pas
de conception -- voir `022` pour le detail.

Lien : [[organism-path-predation-first]], `009_organism.md`, `018_adhesion_gene.md`,
`019_role_gene.md`, `020_role_share.md`, `022_organism_split_gene.md`.
