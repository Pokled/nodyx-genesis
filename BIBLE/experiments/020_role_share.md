# 020 - Le partage de role : moins cher, toujours pas gratuit

2026-09-05, v0.0.2. Suite de `019_role_gene.md` (piste D, etape 2 bis). `[cells] role_share`
(defaut false).

## Le decoupage prealable

`role_gene` controlait deux choses a la fois : l'EXISTENCE du gene (`germinal_bias` mute ou
reste fige) et sa CONSEQUENCE (bloquer la reproduction, `019`). Pour tester une consequence plus
douce sans reecrire `019`, le levier est scinde en deux : `role_gene` (le gene existe et mute,
inchange) et `role_reproduction_gate` (la consequence DURE de `019`, deplacee dans son propre
bouton). Le test de `019` est mis a jour pour activer explicitement les deux (comportement
identique a avant, juste renomme). Sans `role_gene`, les deux consequences restent des no-op
mecaniques garantis : seuil uniforme, aucune scission possible.

## Le mecanisme

Au lieu de bloquer la reproduction, une entite SOMATIQUE (entassement de sa cellule sous SON
seuil personnel) reverse une part `role_share_frac` de son surplus (energie au-dessus d'une
marge de famine) aux entites GERMINALES de LA MEME CELLULE -- un flux conserve, sans RNG, meme
famille que `shelter_feed`/`adipeux_share`, mais gouverne par le gene individuel plutot que la
geometrie ou une moyenne. Personne n'est jamais empeche de se reproduire ; l'avantage passe par
l'energie, pas par un interrupteur.

## Verification directe : moins severe, mais toujours pas neutre

A/B (deux graines, `role_gene = true` partout, sans `role_reproduction_gate`) :

| Mesure | sans `role_share` | avec `role_share` |
| --- | --- | --- |
| Suppression de population en cours de route | -- | 0,20 a 0,73 selon la graine et le tick (jamais aussi severe que le 0,01-0,05 du blocage dur) |
| Extinction observee | non | non |

Nettement moins destructeur que le blocage dur (`019`), mais pas gratuit : un cout reel et
persistant, fluctuant selon la graine. Hypothese : pres d'un seuil de reproduction (pas un taux
continu), rediriger de l'energie est structurellement a perte -- prendre a une entite "loin
au-dessus" du seuil ne lui coute rien (elle se reproduit quand meme) ; prendre a une entite
"juste au-dessus" peut lui faire manquer une naissance ; donner un surplus a une entite deja
au-dessus du seuil ne cree pas une seconde naissance instantanee (bornee par la gestation). Une
asymetrie qui rend toute redistribution pres d'un seuil dur legerement perdante en moyenne,
independamment du sens choisi -- une lecon plus generale que ce seul mecanisme.

**Et la derive de selection ?** Mesuree sur 3 graines (moyenne ponderee-population de
`germinal_bias`, contre la derive neutre pure a `role_gene` seul) : +0,008, -0,055, -0,004 --
pas de direction fiable, un ordre de grandeur comparable au bruit deja observe pour l'adhesion
(`018`). Le partage doux resout le probleme du COUT severe, mais ne resout PAS le probleme de la
selection fiable.

## Test

`role_share_moves_energy_only_when_the_gene_actually_varies` : verifie que le flux change
reellement la trajectoire quand le gene varie (`role_gene = true`), et surtout qu'il reste un
NO-OP MECANIQUE GARANTI quand le gene est fige (`role_gene = false`) -- toutes les entites d'une
meme cellule partagent alors le meme seuil ET le meme `tissue_bonds` (propriete de cellule, pas
d'entite), donc aucune scission donneurs/receveurs n'est possible, pas juste improbable. Pas
d'assertion de cout ni de derive : mesures reelles mais instables, consignees ci-dessus plutot
que figees en invariant.

## Lecture : trois essais, une conclusion honnete sur ce sous-chantier

| Essai | Variance exploitable | Cout ecologique | Retenu |
| --- | --- | --- | --- |
| `018` adhesion (moyenne par cellule) | quasi nulle | aucun | inerte, garde comme piste fermee |
| `019` role, blocage dur | reelle (sd 0,04-0,06) | severe (ratio jusqu'a 0,01-0,05) | garde, cout documente |
| `020` role, partage doux | reelle mais instable | modere (ratio 0,20-0,73) | garde, cout documente |

**Aucun des trois n'atteint ce que `nerve_relay`/`muscle_seek_food` avaient donne : une
variance exploitable ET un gain net sans contrepartie.** Le fil commun : des qu'un gene
individuel est cense influencer une consequence a l'echelle de la CELLULE (adhesion : qui
rejoint le tissu ; role : qui peut se reproduire ou combien elle en garde), soit la mesure se
dilue (moyenne), soit la consequence coute reellement a la population -- la selection sur un
gene individuel, dilue par un contexte collectif, semble structurellement difficile a rendre a
la fois reelle et gratuite dans ce moteur.

**Ce chapitre (piste D, etape 2, "un gene individuel qui module une consequence de cellule")
est clos pour l'instant.** Les trois mecanismes restent au code, eteints par defaut, documentes
honnetement. La suite naturelle indiquee par le diagnostic lui-meme : l'etape 3, la reproduction
a l'echelle de l'ORGANISME ENTIER -- la ou la selection s'exercerait directement sur l'unite qui
porte le genome structurel, sans avoir besoin de le diluer a travers des voisins de cellule non
apparentes au gene.

Lien : [[organism-path-predation-first]], `009_organism.md`, `018_adhesion_gene.md`,
`019_role_gene.md`.
