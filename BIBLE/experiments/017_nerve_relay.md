# 017 - Le relais nerveux : un tissu qui crie plus fort qu'une entite seule

2026-09-04, v0.0.2. Suite de `009_organism.md` (adipeux, essai inerte). `[voice] nerve_relay`
(defaut false).

## Le probleme

La Voix (0.0.4) donne a chaque entite une portee de perception fixe (`signal_radius`, 5 cases) :
une alarme ou un appel n'est entendu que par ce qui se trouve a portee de l'endroit ou il a ete
emis. Une entite seule et une entite au coeur d'un tissu de 50 cellules percoivent le monde
exactement pareil -- aucun avantage a etre organise. Or `genesis-view` classe deja un tissu
« nerveux » a la LECTURE (fraction d'agents parmi ses membres >> le fond), sans que cela ne
compte cote moteur. La marche suivante du roadmap organisme (apres `013` tissu, `015` muscle,
`016` adipeux inerte) : faire que ce type, pour une fois, fasse quelque chose.

## Le mecanisme

Une fois par tick, avant les phases de perception, le moteur mesure -- sans jamais nommer un
tissu « nerveux » -- combien de membres AGENTS (`entity.mind.is_some()`) compte chaque tissu.
Un tissu qui en compte au moins `nerve_min_agents` (def 3) devient un relais pour SES agents
seulement : leur rayon de perception de signal (alarme ET appel) est multiplie par
`nerve_radius_mult` (def 2,5), au lieu du `signal_radius` simple. Rien d'autre ne change : le
signal lui-meme, son origine, sa duree de vie sont identiques ; seule la portee a laquelle un
membre du reseau peut encore le capter s'etend, comme si le cri se propageait le long du tissu au
lieu de se diffuser seul depuis sa source. Aucun mouvement, aucune energie deplacee -- une pure
extension de perception. Conserve, sans RNG, la mesure (nombre d'agents par tissu) est
recalculee chaque tick avant les phases 2/3 et la phase cognition.

Un compteur direct et non ambigu valide la cause : `WorldState.nerve_signals_relayed` cumule le
nombre de fois ou une alarme a ete percue **seulement** grace a l'extension (hors de
`signal_radius`, dans la portee relayee) -- sans lui, il resterait mecaniquement a zero.

## Test

`nerve_relay_extends_alarm_perception_through_a_tissue` : le compteur `nerve_signals_relayed`
progresse reellement (> 0) avec le relais actif, reste a zero coupe (verifie mecaniquement, pas
seulement statistiquement) ; l'ecosysteme tient ; deterministe ; la trajectoire diverge de la
variante coupee. Passe du premier coup -- pas de piege de seuil cette fois (contrairement a
`016`), la mesure directe du compteur evite d'avoir a deviner un effet ecologique indirect.

## A/B graine 26, 60 000 ticks (config w6 : toute la pile organe) -- POSITIF NET

| Mesure | sans relais | relais nerveux | effet |
| --- | --- | --- | --- |
| population finale | 8 706 | **13 031** | **+50 %** |
| entites en cellule, moyenne plateau | 877 | **1 222** | **+39 %** |
| cellules vivantes, moyenne plateau | 20,4 | 25,8 | +27 % |
| tissus vivants (fin de run) | 5 | **8** | **+60 %** |
| ordre du tissu psi6 (fin de run) | 0,184 | **0,513** | **x2,8** |
| organismes vivants (fin de run) | 5 | 6 | +1 |
| agents eveilles (cumul) | 5 576 | 7 155 | +28 % |
| agents vivants (fin de run) | 1 069 | 1 511 | +41 % |
| morts par famine / predation, part | 24 % / 76 % | 22 % / 78 % | profil inchange |
| diversite genetique (finale) | 0,094 | 0,077 | **-17 %** |

Lecture :

**Le plus net des quatre essais sur "que le type compte"** (`014` digestion : negatif ;
`012bis`/epithelium_shield : positif modeste ; `015` locomotion : positif net ; `016` adipeux :
inerte). Population, cellules, tissus, ordre du tissu ET population d'agents montent tous
ensemble, sans que le PROFIL de mortalite (part famine/predation) ne bouge -- ce n'est pas
"moins de morts", c'est "un monde plus grand et plus tisse dans son ensemble". Le psi6 passe de
0,18 (derive) a 0,51 (l'ordre franchement etabli, dans la fourchette saine notee sur w2). Seul
recul : la diversite genetique (-17 %), coherent avec un monde plus dense et plus stable ou la
selection a moins besoin d'explorer.

Hypothese : un agent au coeur d'un tissu qui entend le danger de plus loin (ou l'aubaine) reagit
plus tot, meurt moins souvent au mauvais endroit ; et un tissu qui garde ses agents vivants plus
longtemps est un tissu qui garde ses cellules -- la boucle qui manquait aux essais precedents
(adipeux ne changeait rien car rien n'avait besoin d'etre redistribue ; ici, savoir plus tot
change vraiment le comportement).

**Retenu comme mecanisme, quand il nait avec le monde.** Pas encore essaye a chaud sur un monde
en direct : par prudence (`013`, `015`), a n'allumer que depuis la genese ou apres un
`--restart`, jamais edite dans le `config.toml` d'un monde qui tourne.

Lien : [[organism-path-predation-first]], `009_organism.md`, `015_muscle_seek_food.md`,
`016_adipeux_reserve.md`.
