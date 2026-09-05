# 022 - Le gene sur l'unite qui se reproduit : le mecanisme tient, la preuve reste incomplete

2026-09-05, v0.0.2. Suite de `021_organism_split.md` (piste D, etape 3, tranche 2).
`[organism] split_gene` (defaut false).

## L'idee

`018`-`020` ont tous echoue a rendre un gene individuel a la fois REEL et GRATUIT parce que la
consequence s'exercait a l'echelle de la CELLULE, un contexte collectif non apparente au gene
(dilution par moyenne, ou cout de redistribution pres d'un seuil). `021` a prouve que
l'ORGANISME peut enfin se reproduire (scission) sans cout ecologique. La suite logique : faire
porter un gene DIRECTEMENT par l'organisme -- l'unite qui se reproduit vraiment -- pour voir si
la selection, cette fois, a enfin prise.

## Le mecanisme

`Organism.split_bias` (0.0.2, `[0, 1]`, `0,5` = neutre) : le seuil personnel de scission. Sans
`split_gene`, `split_cells` est un nombre fixe pour tout le monde. Avec, chaque organisme
interpole son propre seuil effectif entre `split_cells_min` (permissif, "beaucoup de petits") et
`split_cells_max` (prudent, "peu de grands") selon son `split_bias`. A la scission, le seuil de
l'enfant s'ecarte de celui du parent (mutation gaussienne, meme mecanique que les genes
structurels d'entite) -- mais ici, contrairement a `018`-`020`, RIEN N'EST MOYENNE : l'organisme
porte et transmet directement son propre gene, sans intermediaire collectif.

## Verification mecanique : ca marche

Confirme (plusieurs graines) : `split_gene = false` fige tout organisme a `split_bias = 0,5`
(aucun tirage RNG, meme regle que tous les genes structurels de cette session) ; `split_gene =
true` fait reellement varier le seuil, et deux organismes freres issus d'une meme scission
peuvent deja differer legerement l'un de l'autre.

## Mais la preuve de selection reste hors de portee, a cette echelle de test

Sonde (5 graines, echantillonnage periodique de `split_bias` moyen sur toute la duree du run,
80 000 ticks) :

| Graine | derive (on - off) |
| --- | --- |
| 1 | -0,044 |
| 26 | +0,002 |
| 40 | +0,003 |
| 6 | (aucun organisme forme) |
| 15 | (aucun organisme forme) |

Pas de direction coherente, un ordre de grandeur comparable au bruit deja observe pour
l'adhesion (`018`) et le partage de role (`020`). **Mais la cause n'est probablement pas la
meme.** Pour `018`/`020`, le probleme etait structurel : le gene etait dilue par un contexte non
apparente. Ici, l'unite mesuree EST la bonne unite -- le probleme est plus simplement la
PUISSANCE STATISTIQUE : `organisms_formed_total` atteint 174 a 274 sur 80 000 ticks, mais rarement
plus de 1 a 3 organismes vivent SIMULTANEMENT (les organismes sont grands, rares, et un tissu
prend du temps a s'etablir avant qu'un premier organisme existe seulement). Un echantillon aussi
petit, sur un nombre de "generations" d'organismes limite en 80 000 ticks, ne permet
probablement pas de faire emerger un signal meme s'il existe en principe.

## Test

`organism_split_gene_is_heritable_and_frozen_without_the_lever` : verifie uniquement ce qui est
solide -- le gene existe et varie reellement quand le levier est actif, reste rigoureusement
fige au neutre sinon (aucun tirage RNG), l'ecosysteme tient, deterministe. Aucune assertion de
direction ni d'ampleur de derive : ce serait un chiffre instable, pas un invariant (meme lecon
que `018`/`020`).

## Lecture

**Ni un succes net, ni un echec -- une limite de mesure, pas de conception.** Le mecanisme
lui-meme (l'organisme se reproduit, porte et transmet un gene propre a l'unite, sans dilution ni
cout, `021`) reste le vrai acquis solide de l'etape 3. Ce que cette tranche n'a pas reussi a
prouver dans le temps disponible : que ce gene, une fois porte par la bonne unite, derive
reellement sous selection -- pas parce que l'idee est fausse, mais parce que les organismes
restent trop rares dans les configurations testees pour qu'un run de 60-80k ticks donne un
echantillon assez grand.

**Pour trancher vraiment**, il faudrait soit des runs beaucoup plus longs (des centaines de
milliers de ticks, pour accumuler plus de generations d'organismes), soit un monde ou les
organismes se forment plus tot et plus souvent (seuils `min_cells`/`reach` plus permissifs), soit
mesurer la survie/le nombre d'enfants par lignee d'organisme plutot qu'un instantane de
population -- une piste pour une session future, pas une urgence de celle-ci. Garde dans le
code (defaut `false`, aucun monde vivant affecte).

## Bilan de la piste D pour cette session

| Etape | Mecanisme | Verdict |
| --- | --- | --- |
| 1 | `adhesion_gene` | mecanique OK, selection quasi nulle (dilution par cellule) |
| 2 | `role_reproduction_gate` | variance reelle, cout ecologique severe |
| 2 bis | `role_share` | variance reelle, cout modere, selection instable |
| 3 | `split_enabled` | **l'organisme se reproduit, cout quasi nul -- le vrai acquis** |
| 3 tranche 2 | `split_gene` | mecanique OK, selection non prouvee (echantillon trop petit) |

Le fil qui traverse toute la piste : la DILUTION par un contexte collectif non apparente est le
vrai ennemi de la selection individuelle dans ce moteur (confirme quatre fois). L'eviter en
portant le gene directement par l'unite qui se reproduit (`021`/`022`) est la bonne direction ;
ce qui manque encore n'est pas conceptuel, c'est un echantillon plus grand ou un run plus long.

Lien : [[organism-path-predation-first]], `009_organism.md`, `018_adhesion_gene.md`,
`019_role_gene.md`, `020_role_share.md`, `021_organism_split.md`.
