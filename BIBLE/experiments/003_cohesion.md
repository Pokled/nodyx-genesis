# 003. Experience, l'agregation en colonies (cohesion)

Statut : **V1 implementee et active** (2026-09-01). La force de mouvement reste eteinte
(`[cohesion] pull_max = 0`). Le trait `cohesion` agit par la retenue sur les communs.
Gabarit de `001_emergence.md`.

## V1, resultat (2026-09-01)

`cohesion` ne bouge plus personne. En phase 5, une entite `cohesion` haute et entouree de
parents (`colony_support`) mange moins (`eat_restraint = 0.25`, cout prive) et fatigue
beaucoup moins la case (`strain_restraint = 0.7`, bien commun). Code : `sim.rs` phase 5.

A/B a la meme graine, 16 graines, cooperation ON vs OFF (`eat_restraint = strain_restraint = 0`) :
- Survie : **10/16 avec, 8/16 sans.** Pas de destabilisation (contraste total avec la force
  de mouvement, ~0/16). La cooperation aide meme un peu.
- Trait `cohesion` moyen, graine 12 : demarre ~0.42, descend a ~0.405 tant qu'il n'y a pas
  de colonies (cout sans benefice), **remonte a ~0.47** quand la population grandit et que
  des groupes de parents se forment. L'ecart-type passe de 0 a ~0.11 : des variants
  exploiteurs et cooperateurs coexistent.
- Verdict : le mecanisme repond a la selection, dans les deux sens, sans casser l'ecosysteme.
  L'effet est reel mais doux (pas de balayage brutal). C'est une base honnete. On peut
  durcir plus tard (V2, V3) si on veut une dynamique plus tranchee.

## Contexte

Lot 2 du chantier « Dezoomer ». Idee : les entites proches et genetiquement voisines
s'attirent, forment des amas (coacervats d'Oparine, l'etape avant la premiere cellule), et
une colonie protege la reproduction. Un 7e trait de genome `cohesion`, evoluable, donne la
tendance a s'agreger. Le benefice : etre entoure de voisins proches abaisse le plancher
`birth_loss` (l'agregat sert d'infrastructure).

**Premier essai, 2026-09-01 : echec.** Avec la force active, quasiment tous les mondes
s'eteignent (les colonies se disputent la nourriture localement et s'affament) ou explosent
en mondes geants lents. Baisser `pull_max` de 0.6 a 0.15 n'a pas suffi. Le mecanisme est
sur un fil : trop faible il ne fait rien, assez fort pour compter il declenche une boucle
de retroaction qui tue ou emballe.

Conclusion honnete (tranchee 16) : on ne rallume pas une force qui casse l'ecosysteme juste
pour la montrer. On la garde en place, eteinte, et on la reprend proprement ici.

## Ce qui reste en place

- Le trait `cohesion` (genome a 7 traits, schema v2). Il evolue, il est affiche dans le
  brin d'ADN et les stats, il ne fait juste rien tant que `pull_max = 0`.
- `SpatialHash` (`crates/genesis-core/src/spatial.rs`), reutilise par la reproduction
  (densite locale) et le detecteur d'especes (coherence spatiale).
- `Entity.colony_support`, calcule chaque tick, inerte tant que `support_birth_relief = 0`.

## Question

Existe-t-il un reglage ou une variante de la force d'agregation qui :

1. fait emerger des amas visibles et stables,
2. sans effondrement ni emballement sur la majorite des graines,
3. avec un trait `cohesion` qui repond a la selection (monte quand l'agregation aide,
   descend quand elle nuit),
4. de facon deterministe et bornee.

## Le changement d'angle (recommande)

La force active sur le mouvement a echoue parce qu'elle fait abandonner la nourriture pour
s'entasser. Mais **l'agregation passive existe deja** : les entites convergent sur les
cases riches par chimiotaxie, ca fait des amas de fait, sans aucune force.

Donc `cohesion` ne bouge plus personne. Elle decrit **comment une entite se comporte une
fois deja dans un groupe** : la tragedie des communs.

- `cohesion` haut = retenue. Entouree de voisins proches et eux aussi coherents, l'entite
  recolte moins agressivement -> ajoute moins de `strain` a la case partagee -> le patch
  reste productif -> tout le groupe de parents survit plus longtemps.
- `cohesion` bas = exploitation. On vide le patch, boom, puis bust, on s'en va.
- Selection : dans un milieu stable, un groupe de parents qui menage son patch se reproduit
  mieux qu'un groupe qui le crashe. `cohesion` monte la ou l'environnement recompense la
  gestion. Une entite seule, ou entouree d'etrangers, ne gagne rien a la retenue ->
  `cohesion` derive vers le bas. Frequence-dependant, les deux sens.

Pourquoi c'est stable, contrairement a la force : ca ne module que le terme `strain`, deja
borne [0,1] et auto-correcteur ; pas de boucle sur le mouvement ; le benefice est diffus et
differe (le patch reste bon plus tard), pas un pic de natalite instantane.

Second levier, plus petit : le soulagement `birth_loss` de la colonie, mais qui **remplace**
le cout de surpopulation au lieu de s'y ajouter (dans une colonie coherente et parente, la
densite ne penalise plus), plafonne fort, et exige un vrai voisinage dense de parents.

## Autres pistes, en A/B a la meme graine

| Variante | Idee | Pourquoi ca pourrait marcher |
|---|---|---|
| V1 | `cohesion` = retenue sur la recolte (ci-dessus) | selection reelle, pas de boucle sur le mouvement |
| V2 | + benefice `birth_loss` qui remplace le cout de densite dans une colonie parente | l'optimum devient une vraie colonie |
| V3 | la colonie partage la recherche de nourriture (un membre bien nourri guide les autres) | l'agregation devient un avantage de survie clair |
| V4 | force de mouvement, mais seulement avec un surplus d'energie franc (seuil dur) et une repulsion a courte distance pour ne pas s'ecraser sur un point | garde l'idee de l'attraction sans l'effondrement |

## Mesures, tracees dans le temps

- population, extinction ou emballement,
- diversite genetique, nombre de `SpeciesEmerged`,
- trait `cohesion` moyen (repond-il a la selection ?),
- taille et nombre d'amas (via `SpatialHash`), duree de vie d'un amas.

## Critere de reussite

Une variante ou, sur au moins la moitie des graines, des amas stables se forment, la
population tient, et le trait `cohesion` moyen bouge dans le sens de l'avantage. Sinon,
l'agregation attend le lot chimie (les coacervats sont peut-etre un phenomene de la couche
chimique, pas du moteur abstrait).

## Livrable

Un run reproductible par variante, les courbes, un paragraphe de verdict, dans
`experiments/003-cohesion/`.
