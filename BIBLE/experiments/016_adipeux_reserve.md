# 016 - La reserve adipeuse : une graisse qui ne se vide que dans le besoin

2026-09-04, v0.0.2. Suite de `009_organism.md`. `[organism] adipeux_share` (defaut 0).

## Le probleme

`pool_share` (`013`, deja en place) lisse en permanence l'energie de tous les membres d'un
organisme vers leur moyenne : l'organisme a faim ou est repu EN ENTIER. C'est un bon socle,
mais ce n'est pas une reserve -- c'est un lissage constant, pas quelque chose qui se remplit
dans l'abondance et se vide dans le besoin. Et l'essai precedent sur "que le type compte" via
l'energie (`014`, la digestion) avait echoue en ajoutant de l'activite (une cellule qui va
chercher a manger). Il fallait une deuxieme tentative, strictement passive.

## Le mecanisme

En plus du lissage uniforme de `pool_share`, les membres d'une cellule **ronde**
(`elongation < 1.6`, le meme critere geometrique que "adipeux" dans `genesis-view`) et
**gorgee** (energie >= `adipeux_rich_frac` du plafond) versent une part `adipeux_share` de LEUR
surplus aux membres de l'organisme vraiment **en danger** (energie sous
`starve_at + energie_plafond * 0,08`, une marge au-dessus du point de mort par famine plutot
qu'un multiple degenere quand `starve_at = 0`). Conserve, sans RNG, ordre d'id d'organisme puis
d'entite.

**Purement passif** : aucun mouvement, aucune entite hors de l'organisme ponctionnee. Contraint
a l'organisme (pas au tissu comme `tissue_shelter`) : la graisse d'une partie du corps nourrit
n'importe quelle autre partie en danger, meme un tissu different.

## Un piege de seuil, pas de conception

Premiere mesure : **aucun effet du tout** (1,080 des deux cotes, identique au chiffre pres).
Diagnostic : `starve_at` vaut 0 par defaut, et le seuil de danger etait ecrit `starve_at * 2,0`
(copie du motif de `tissue_shelter`) -- qui degenere a 0 quand `starve_at = 0`. Une entite ne
peut quasiment jamais avoir une energie strictement negative, donc "en danger" n'etait jamais
vrai : le bloc entier ne faisait rien. Corrige en une marge ABSOLUE au-dessus du point de mort
(`starve_at + energie_plafond * 0,08`), a l'echelle du monde plutot qu'un multiple d'un seuil
souvent nul. Avec ce seuil, le test passe du premier coup.

**Lecon (troisieme fois cette session) : verifier qu'un mecanisme fait vraiment quelque chose
avant de juger s'il fait bien quelque chose.** Un chiffre identique des deux cotes d'un A/B
n'est jamais un hasard : soit le binaire n'a pas ete reconstruit (`experiments`, memo dedie),
soit -- ici -- une condition ne se declenche jamais.

## Test

`adipeux_reserve_rescues_starving_organism_members` : des organismes multi-cellules existent ;
l'energie MINIMALE parmi les membres d'un meme organisme, moyennee sur la vie du monde, est
plus haute avec la reserve qu'avec le seul lissage uniforme, meme graine ; `adipeux_share = 0`
ne change rien ; deterministe ; l'ecosysteme tient.

## A/B graine 26, 60 000 ticks (config w6 : toute la pile + organism.pool_share)

Meme au seuil le plus permissif essaye (`adipeux_share = 0,9`, `adipeux_rich_frac = 0,4`,
c'est-a-dire un donneur au-dessus de 6,4 sur 16 et un besoin sous 1,28), `series.jsonl` sort
**identique a l'octet pres** entre `adipeux_share = 0` et `adipeux_share = 0,9`, sur 60 000 ticks.
Ni "binaire pas reconstruit" (verifie : l'exe est plus recent que le dernier changement de
`sim.rs`), ni "le champ n'est pas lu" (verifie : le `config.toml` ecrit en sortie, qui vient de la
struct reellement utilisee et pas d'une copie du fichier d'entree, porte bien la bonne valeur).

Diagnostic direct : un compteur temporaire dans `organism_pass` (retire apres coup) compte, a
chaque controle (`check_every = 200` ticks), le nombre d'organismes qui ont au moins un membre
donneur (rond, gorge) ET au moins un membre en danger, EN MEME TEMPS, dans le MEME organisme.
Sur 95 controles consecutifs (ticks ~15 000 a ~46 000, de 1 a 5 organismes vivants selon le
moment) : **zero occurrence**. Les organismes n'ont jamais eu un membre gorge et un membre en
danger a la fois -- soit personne n'est en danger (le cas dominant une fois la population etablie),
soit (plus tot, quand un seul petit organisme existe) donneur et besoin alternent d'un controle a
l'autre sans jamais se croiser.

**Ce n'est pas un bug du mecanisme -- c'est `pool_share` qui lui retire son carburant.**
`pool_share` (deja actif, `0,15` par controle, meme cadence) ramene en permanence TOUS les
membres d'un organisme vers LEUR moyenne commune : l'organisme entier tend a etre comfortablement
nourri ensemble ou en difficulte ensemble, jamais un membre tres au-dessus pendant qu'un autre
est tres en dessous. Exactement l'heterogeneite dont `adipeux_share` a besoin pour se declencher
est celle que `pool_share` efface avant qu'il ait sa chance -- les deux mecanismes, empiles a la
meme cadence, sont en tension directe. Le test unitaire, lui, passe : avec un `pool_share`
identique (`0,15`) mais un monde plus jeune, plus petit, sans `muscle_contract` (moins de bruit
d'energie), assez d'ecart survit entre membres pour que la reserve ait quelque chose a redistribuer.
Le code fonctionne ; a l'echelle et au regime de w6, il n'a simplement jamais l'occasion d'agir.

Lecture : **negatif par inertie, pas par defaut de conception.** Garde dans le code (defaut
`adipeux_share = 0`, donc aucun monde vivant n'est affecte), documente comme piste fermee a ce
regime : pour qu'elle compte, il faudrait soit baisser `pool_share`, soit une source d'ecart
d'energie plus forte que ce que `muscle_contract` + un environnement riche laissent passer.
Pas de troisieme tentative sur ce mecanisme dans cette session -- prochaine marche : nerveux
(relais de signal, la piste suivante du roadmap organisme).

Lien : [[organism-path-predation-first]], `009_organism.md`, `013_tissue_bond.md`,
`014_epithelium.md`, `015_muscle_seek_food.md`.
