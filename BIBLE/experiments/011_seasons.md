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

## Ce qui reste

- **Saison thermique** : coupler la temperature (deja branchee, `008`) a la saison, un optimum
  qui oscille. Demande un trait de genome de preference thermique pour que la selection
  *alterne* vraiment entre genomes chauds et froids (aujourd'hui la disette selectionne les
  memes traits d'economie a chaque cycle).
- **Derive spatiale** : les bosses de fertilite qui migrent lentement sur la carte, pour des
  deplacements de population et de l'adaptation locale. Plus lourd (la fertilite alimente
  toute la regeneration).
- **Cataclysme** : un evenement rare qui decale durablement le climat ou rase une region
  (`00_INDEX.md`). Distinct des saisons : une rupture, pas un cycle.
