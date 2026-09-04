# 019 - Le gene de role : une vraie variance, un vrai cout

2026-09-04/05, v0.0.2. Suite de `018_adhesion_gene.md` (piste D, etape 2). `[cells] role_gene`
(defaut false).

## Le probleme laisse par l'etape 1

Le gene d'adhesion (`018`) marchait mecaniquement mais ne se selectionnait presque pas : la
formation de cellule regroupe des entites par parente de TRAITS, sans rapport avec le gene, donc
`Cell.mean_adhesion` (une moyenne par cellule) regressait vers la moyenne de population --
aucune variance exploitable pour la selection individuelle. Le diagnostic pointait vers une
solution : eviter toute moyenne par cellule, lire le gene ENTITE PAR ENTITE.

## Le mecanisme

`tissue_shelter` (deja en place) mesure l'entassement d'une cellule (`Cell.tissue_bonds`,
voisines du meme tissu) contre un seuil FIXE (`shelter_bonds`) pour distinguer coeur (protege,
nourri) et bord. `role_gene` reprend cette meme mesure mais avec un seuil PERSONNEL : chaque
ENTITE porte son propre `genome.structural.germinal_bias` (herite, mute a la division comme les
autres genes structurels, gele au neutre `0,5` sans tirage RNG si le levier est coupe -- meme
regle que `018`). A la phase 7 (reproduction), une entite dans une cellule ne peut se reproduire
que si l'entassement de SA cellule (`tissue_bonds / role_bonds_scale`) atteint SON seuil
personnel -- germinale si assez entouree pour elle, somatique sinon (elle encaisse, ne se divise
pas). Hors cellule : toujours eligible, comportement d'origine inchange.

**La difference cle avec `018`** : deux entites de la MEME cellule peuvent avoir des seuils
differents et donc des verdicts differents -- pas de moyenne qui efface la variance individuelle.

## Verification directe : la variance existe vraiment

Sonde (plusieurs graines, config tissu + predation + abri) : l'ecart-type INTRA-cellule de
`germinal_bias` atteint 0,04-0,06 -- comparable a l'ecart-type de POPULATION entiere du gene
d'adhesion (~0,05). Confirme : contrairement a `mean_adhesion`, ce gene garde une vraie prise
individuelle pour la selection. Compteur direct `role_blocked_total` (entites par ailleurs
eligibles, ecartees par leur propre seuil) confirme aussi que le mecanisme s'exerce reellement,
pas seulement en theorie.

## Mais un vrai cout ecologique, confirme sur deux graines

A/B a l'echelle w7 (toute la pile organe), 60 000 ticks :

| Graine | population finale (off) | population finale (on) | ratio |
| --- | --- | --- | --- |
| 40 | 14 899 | 14 071 | 0,94 (fin de run) -- mais **creux tres profond** en cours de route (ratio ~0,01-0,05 entre les ticks 8 000 et 40 000, ecart qui se resorbe tard) |
| 26 | 13 031 | 7 902 | 0,61 |

Population moyenne au plateau (graine 40, seconde moitie du run) : ratio 0,33 (population),
0,30 (entites en cellule), 0,36 (tissus vivants) -- une suppression severe et prolongee, pas un
simple bruit de depart. Sur la graine 26 le cout est plus doux (0,61) mais net et dans le meme
sens. **Aucune extinction dans les deux graines testees** : le monde encaisse, ralentit
fortement, et finit par recuperer -- different d'un effondrement complet, mais un cout reel,
pas gratuit.

Coherent avec l'avertissement deja ecrit dans `009_organism.md` (piste B) : *"couper la
reproduction d'une part de la population peut effondrer un monde"*. Ici il ne l'effondre pas,
mais il la ralentit fortement, surtout tant que le tissu (et donc l'eligibilite germinale) est
encore rare -- une cellule qui vient a peine de se former n'a souvent pas assez de voisines pour
que quiconque compte comme germinal, donc personne ne se reproduit dans cette cellule tant
qu'elle n'a pas grandi par d'autres moyens (formation de nouvelles cellules libres). Un frein
qui mord fort exactement au moment ou la population a le plus besoin de croitre.

## Test

`role_gene_creates_selectable_variance_without_cell_averaging` : verifie le point qui compte
vraiment pour la piste D -- la variance intra-cellule est reelle (contrairement a `018`) -- plus
le mecanisme s'exerce reellement (`role_blocked_total > 0`), l'ecosysteme tient (pas
d'extinction dans le regime teste), deterministe, `false` fige tout (aucun tirage RNG, aucun
blocage). Pas d'assertion sur le sens ni l'ampleur de la derive de population : deux graines
donnent des couts differents (0,33 a 0,61 au choix de la mesure), la seule chose stable est la
DIRECTION (toujours un cout, jamais un gain) et l'absence d'extinction totale.

## Lecture

**Un vrai progres technique sur la piste D (la variance qu'il faut pour la selection existe
enfin), mais un cout ecologique reel, pas neutre comme `nerve_relay`/`muscle_seek_food`.** Plus
proche de `tissue_bond` (cout net sur la biomasse, garde quand meme comme brique utile) que de
`018` (inerte) ou des mecanismes gratuits de cette session. Garde dans le code (defaut `false`,
aucun monde vivant affecte). A ne PAS allumer sur un monde etabli sans A/B prealable -- et,
comme toujours, jamais a chaud (voir `013`/`015`).

**Pour la suite** : la variance existe maintenant, mais la fonction du role (rester germinal =
pouvoir se reproduire) est peut-etre le mauvais levier -- COUPER la reproduction quand la
prochaine cellule n'a pas encore de tissu revient a punir la population au pire moment.
Alternative a explorer avant la piste D etape 3 (reproduction d'organisme entier) : garder le
gene et sa variance, mais lui faire moduler quelque chose de moins vital que le DROIT de se
reproduire (une part d'energie, une vitesse de gestation) plutot qu'un interrupteur tout ou rien.

Lien : [[organism-path-predation-first]], `009_organism.md`, `018_adhesion_gene.md`.
