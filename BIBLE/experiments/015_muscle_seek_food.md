# 015 - La locomotion dirigee : un tissu qui rampe vers la nourriture

2026-09-04, v0.0.2. Suite de `009_organism.md` (contraction musculaire) et `014_epithelium.md`.
`[cells] muscle_seek_food` (defaut false).

## Le probleme

La contraction musculaire (`muscle_contract`, deja en place) fait battre une cellule fusiforme
au rythme d'une onde peristaltique. Mais l'axe de cette onde est **arbitraire** : une fonction
de l'id du tissu (`cos(tid*0.7), sin(tid*0.7)`), sans rapport avec le monde autour. Un muscle
bat sur place, il ne va nulle part en particulier. Pour que "notre premier etre vivant" ait
vraiment l'air vivant, il fallait qu'un tissu contractile puisse se diriger.

## Le mecanisme

Chaque entite pratique deja la chimiotaxie individuelle (`forage_target`) : elle remonte le
gradient de ressources dans son rayon de perception. `muscle_seek_food` applique la **meme**
fonction, au tissu entier plutot qu'a une entite seule :

1. Pour chaque cellule contractile, `forage_target` cherche s'il y a mieux a portee
   (`muscle_sense_radius` x son rayon). S'il n'y a rien de mieux, retombe sur l'axe arbitraire
   d'origine (comportement inchange).
2. S'il y a une cible reelle, l'axe de l'onde peristaltique s'oriente vers elle.
3. **Le pas qui compte** : reformer sa silhouette sur son propre centre est symetrique, ca ne
   deplace nulle part (constate a l'essai, voir plus bas). Pendant la phase active de
   contraction, la cellule tire maintenant TOUT son nuage de membres d'un cran vers la cible
   reellement sentie -- une extension de pseudopode, pas une teleportation : bornee, au rythme
   du muscle, seulement quand il y a vraiment quelque chose a atteindre.

Deterministe (aucun RNG), `#[serde(default)]`, schema inchange.

## Deux essais avant que ca marche

**Essai 1 (rejete)** : ne changer que l'axe de l'onde (le "quand" de la contraction), sans
toucher a la force elle-meme. Mesure (graine 1, ressource sous les cellules contractiles) :
**aucun effet mesurable** (0,841 vs 0,854, dans le bruit). Diagnostic : chaque cellule se
contracte deja le long de son propre axe principal (`cloud_shape` de ses membres), qui n'a pas
de raison d'etre aligne avec l'axe de l'onde. Changer le "quand" sans changer le "vers ou" ne
deplace rien -- ca ne fait que rephaser des cellules qui, chacune, restent symetriques sur
elles-memes.

**Essai 2 (retenu)** : le pas de reptation decrit plus haut, ajoute a l'essai 1. Premiere
mesure (meme metrique, "ressource sous les cellules") : encore pire (0,749 vs 0,854). **La
metrique elle-meme etait viciee** : une cellule qui trouve vraiment la nourriture la MANGE, et
fait donc baisser la ressource juste sous elle -- "etre sur une case pleine" mesure l'inverse de
"avoir bien mange". Changement de mesure pour ce qui compte vraiment : l'energie des membres.
La le test passe nettement (voir A/B).

**Lecon** : verifier un mecanisme de mouvement par la position/le terrain peut se retourner
contre soi des qu'il y a consommation. Mesurer le resultat (l'energie, la survie), pas le decor.

## Test

`muscle_seek_food_moves_tissue_toward_resources` : des cellules contractiles existent des deux
cotes ; l'energie moyenne des membres de cellules contractiles est plus haute avec la
chimiotaxie qu'avec l'axe arbitraire, meme graine ; `false` ne change rien ; deterministe ;
l'ecosysteme tient.

## A/B graine 24, 60 000 ticks (config w5 : tissue_bond + epithelium_shield + muscle_contract) -- POSITIF

| Mesure | axe arbitraire | locomotion dirigee | effet |
| --- | --- | --- | --- |
| population moyenne (plateau) | 7 547 | 7 635 | +1,2 % |
| population finale | 14 846 | 14 764 | ~stable |
| entites en cellule (fin de run) | 839 | **1 059** | **+26 %** |
| cellules vivantes, moyenne | 24,4 | 26,2 | +7 % |
| tissus vivants (fin de run) | 0,45 | **0,95** | **x2,1** |
| ordre du tissu psi6 (fin de run) | 0,041 | 0,067 | +63 % |
| divisions de cellule (cumul) | 29 | 28 | ~stable |
| cellules formees (cumul) | 347 | 349 | ~stable |
| diversite genetique (finale) | 0,059 | **0,082** | **+39 %** |

Lecture :

**Positif sur (presque) toute la ligne, sans contrepartie visible.** Contrairement aux deux
essais precedents sur "que le type compte" (digestion : negatif ; rempart : positif mais
modeste), la locomotion dirigee bouge nettement les mesures qui comptent le plus pour l'organe :
**deux fois plus de tissus vivants en moyenne**, **+26 % de biomasse pluricellulaire**, une
diversite genetique en nette hausse -- et sans rien perdre sur la population globale ni les
divisions. Hypothese : un tissu qui rampe activement vers la nourriture au lieu de battre sur
place trouve plus souvent de quoi tenir, se dissout moins, et une lignee tissee vit assez
longtemps pour que la selection ait le temps de jouer en sa faveur (au lieu d'etre rabotee par
le hasard de sa position de naissance).

**Retenu comme mecanisme.** Contrairement a `tissue_bond` (cout net sur la biomasse) et a
l'essai 1 de `014` (negatif), c'est la premiere brique de la marche organe qui ameliore les
mesures sans compromis identifie -- **quand elle nait avec le monde**. A verifier sur d'autres
graines avant d'en faire un defaut.

## Essai a chaud sur w5 -- ECHEC, meme lecon que `013`

Allume sur w5 en cours de vie (an ~46, tick 400 000+, genome deja adapte a l'axe arbitraire
d'origine) : population 14 960 -> **8 260 en environ 1800 ticks (~40 secondes reelles a
45 t/s)**, cellules vivantes 46 -> 7, tissus vivants -> 0. Effondrement rapide et net, la
signature exacte du choc deja observe avec `tissue_bond` sur w2 (`013`) : un mecanisme qui
co-evolue proprement avec le genome depuis la genese (l'A/B le montre, population stable,
aucun effondrement en 60 000 ticks) peut choquer une population deja etablie autour du
comportement precedent. **w5 remis sans `muscle_seek_food`**, la population est remontee a la
normale (~14 960, cellules 65-86, tissus 4-8) des le redemarrage suivant.

**Regle qui se confirme, a deux reprises maintenant :** un nouveau mecanisme de tissu ou
d'organisme ne s'allume PAS a chaud sur un monde qui tourne. Soit il nait avec un monde neuf
(comme `muscle_seek_food` a ete verifie sur w5 via l'A/B ci-dessus, jamais deploye a chaud sur
LE w5 en direct), soit il attend que le monde en direct actuel s'eteigne et reparte d'une
graine fraiche (`--restart`). `muscle_seek_food` reste un candidat solide pour la PROCHAINE
genese, pas pour une bascule en direct.

Lien : [[organism-path-predation-first]], `009_organism.md`, `014_epithelium.md`.
