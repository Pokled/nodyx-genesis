# 011. Experience, les saisons

Statut : **implementees et actives** (2026-09-03). Gabarit de `008_climate.md`.

Position : le jalon 0.0.4, apres la Voix. Le monde de reference epinglait sa population a la
capacite de charge de la matiere, ligne parfaitement plate depuis la naissance : les
generations montent, la diversite tient, mais aucun boom, aucun krach, aucune expansion. Rien
de spectaculaire a regarder a l'an 70. `005_carrying_capacity.md` l'avait anticipe : une
oscillation ample demande une **mortalite correlee** (famines synchrones), non implementee a
l'epoque. Les saisons sont cette mortalite correlee.

---

## Question

1. Un environnement qui **change lentement et regulierement** (l'abondance des ressources
   oscille au fil de l'annee) fait-il **respirer la population** : des booms en saison grasse,
   des reculs en saison maigre, de facon visible ?
2. La selection reste-t-elle **active** au lieu de se figer : diversite qui pulse, especes qui
   continuent d'emerger, le genome dominant qui peut re-basculer ?
3. Le defaut reste-t-il maitrisable, l'effet **deterministe**, et `amplitude = 0`
   **byte-identique** a un monde sans saisons ?

## Montage

Aucun etat nouveau, aucun bump de schema : la saison est une fonction pure du tick.

- **Phase 1 (regeneration)** : le plafond nourricier de chaque case (`max_per_cell`) **et** sa
  vitesse de regeneration sont multiplies par `season_factor(cfg, t) = max(regen_floor,
  1 + amplitude * sin(2 pi t / periode))`. Periode en `period_years` annees-monde (une annee =
  8766 ticks a 1 h/tick). Saison maigre, les cases portent moins et reviennent moins vite :
  une vraie famine. Une case au-dessus de son plafond descendu n'est pas videe, juste plus
  alimentee (lag realiste). Scaler seulement la regen, ou seulement le plafond, ne suffit pas
  (amplitude de population ~385 et ~1260 respectivement) ; les deux ensemble donnent ~4300.
- Pour que la saison **morde**, il faut que la nourriture, et non la matiere structurelle,
  soit le frein qui compte. `matter_per_cell` passe de 0,14 a 0,26 : la capacite de la matiere
  (~9600 sur 192x192) devient un plafond que seuls les pics de saison grasse approchent. Sans
  ce reglage, la matiere tient la ligne et les saisons ne se voient quasi pas (amplitude ~20).

Nouveau bloc `[season]` : `amplitude` (0,5), `period_years` (1,6), `regen_floor` (0,15).
`amplitude = 0` : `season_factor` vaut 1 partout, aucun effet, byte-identique.

`season_phase(cfg, t)` dans [-1, 1] est expose a l'overlay (`live.json`) : `+1` pleine
abondance, `-1` pleine disette. Le bandeau du bas nomme la saison ; la scene recoit un voile
tres leger (chaud vers l'abondance, froid vers la disette, alpha plafonne a 0,07).

## Resultats

Graine 1, `matter_per_cell = 0,26` dans les deux bras. Figé : 130 000 ticks. Saisons
(`amplitude = 0,5`, `period_years = 1,6`) : 120 000 ticks. Mesures sur le dernier tiers.

| mesure | figé (amplitude 0) | saisons (amplitude 0,5) |
|---|---|---|
| population moyenne | 9 567 | 8 989 |
| population min / max | 9 550 / 9 578 | **5 303 / 9 583** |
| amplitude de population | 28 | **4 280** |
| écart-type de population | 5 | 1 108 |
| diversité génétique (moyenne, plage) | 0,095 (0,063 - 0,194) | 0,090 (0,035 - 0,154) |
| génération moyenne / max | 19,7 / 35 | 21,8 / 32 |
| morts par famine / par âge | 81 700 / 28 300 | **146 800 / 8 900** |
| espèces émergées / basculements de génome | 5 / 1 | 6 / **2** |

Lecture :

- **La population respire.** Sans saisons, une ligne plate à 9 567 (amplitude 28 sur 130 000
  ticks). Avec, elle oscille entre 5 300 et 9 580 : chaque année-monde et demie une disette la
  rabote d'un bon tiers, puis la saison grasse la ramène au plafond. Le monde de référence
  n'est plus une ligne.
- **Les morts basculent de nature.** Figé, la famine et l'âge tuent à peu près autant. Avec
  les saisons, la famine explose (+80 %) et les morts d'âge s'effondrent (-69 %) : on ne vit
  plus assez vieux pour mourir de vieillesse, une disette vous prend avant. C'est la mortalité
  corrélée que `005` attendait.
- **L'évolution se re-déclenche.** Deux basculements de génome au lieu d'un : les goulots de
  disette re-brassent le centre génétique de la population. La diversité moyenne baisse un peu
  et plonge plus bas dans les creux (0,035, quasi clonale) mais remonte ensuite (0,154) : les
  bottlenecks sont réels, pas une érosion lente.

Déterminisme byte-identique 1 vs 8 threads (config par défaut et config figée). `replay` OK
(le bloc `[season]` fait l'aller-retour par `config.toml`). Test `seasons_swing_the_world` :
`amplitude = 0` inerte à l'octet, `amplitude > 0` fait diverger le monde et coûte plus de
morts de faim, le tout déterministe.

## Tranche 2 : la saison thermique et `heat_tol` (2026-09-03, schema v18)

La saison nourriciere fait respirer la population, mais la selection reste la meme a chaque
disette. On ajoute deux choses.

- **Temperature effective** : `temperature_c + season.temp_amplitude_c * season_temp_phase(cfg, t)`,
  ou `season_temp_phase` est **decalee d'un quart de cycle** sur la saison nourriciere (le plus
  froid n'est pas le plus maigre). La population affronte donc deux periodes de stress
  distinctes par annee : la disette (a temperature moyenne) et le grand froid (a nourriture
  correcte). `temp_amplitude_c` (`[season]`, 5 degres ; 8 tuait trop de graines marginales
  apres le decalage RNG du 10e trait) ; `0` = pas de saison thermique.
- **Trait `heat_tol`** (genome, indice 9, `#[serde(default = "half")]`, schema v18). Il place
  l'optimum metabolique propre a une entite entre `temp_optimal_c - heat_tol_span_c/2` (froid)
  et `+ span/2` (chaud). En phase 5 le surcout metabolique passe d'un facteur commun a un
  facteur **par entite**, `1 + temp_metab_slope * |temp_effective - optimum_du_corps|`.
  `heat_tol_span_c` (`[planet]`, 16) ; `0` = trait inerte. `SPECIES_TRAITS` reste 7 : c'est
  un ecotype, pas une espece. `VIEW_VERSION` 10, brin d'ADN a 10 barreaux.

Resultats (graine 1, 130 000 ticks, monde de reference complet, `temp_amplitude_c = 5`,
diversite = moyenne du dernier tiers) :

| bras | diversite genetique | `heat_tol` moyen |
|---|---|---|
| saison thermique coupee (`temp_amplitude_c = 0`) | 0,085 | 0,47 (inerte, il derive) |
| saison thermique + `heat_tol` actif (defaut) | **0,122 (+44 %)** | 0,36 (adapte au froid) |

Lecture : le second stress annuel (le grand froid, decale de la disette) entretient la
diversite genetique, +44 %. `heat_tol` derive vers l'adaptation a la saison la plus dure : la
disette pese plus que le grand froid, donc le trait glisse vers le froid sans s'inverser au
fil de l'annee. **L'ambition de depart, une selection qui alterne (froid l'hiver, chaud
l'ete), n'a pas pris** : l'ete est toujours facile (nourriture abondante), etre adapte au
chaud n'est jamais un avantage de survie. Ce qui reste vrai et mesure : un axe genetique de
plus qui repond au climat, et une diversite sensiblement plus haute. La selection sur `heat_tol`
passe par une survie un peu meilleure, pas par plus de descendance (au plafond de matiere,
plus d'energie ne fait pas plus d'enfants). A temperature statique, un monde chaud tient
`heat_tol` plus haut qu'un monde froid ; l'ecart s'efface a `span = 0` (test
`heat_tolerance_is_selected`). Determinisme byte-identique 1 vs 8 threads, rejeu OK.

Le 10e trait decale le flux RNG : les graines viables changent, `worlds/` est regenere avec
de nouvelles graines (`w2` reste la graine 1).

Ce n'est pas le feu d'artifice espere, mais c'est le premier axe genetique qui repond au
climat, et surtout : le monde a enfin l'environnement changeant, nourricier **et** thermique,
que le sexue emergent attendait. Prochain gros pas.

## Ce qui reste

- **Derive spatiale** : les bosses de fertilite qui migrent lentement sur la carte, pour des
  deplacements de population et de l'adaptation locale. Plus lourd (la fertilite alimente
  toute la regeneration).
- **Sexue emergent** : la roadmap le defere jusqu'a un environnement changeant. Il existe
  maintenant (saison nourriciere + saison thermique). Prochain gros pas.
- **Cataclysme** : un evenement rare qui decale durablement le climat ou rase une region
  (`00_INDEX.md`). Distinct des saisons : une rupture, pas un cycle.
