# 04. Simulation et modèle de temps

Statut : LOCKED sur le principe (tranchée T-4, deux horloges). PROPOSED sur le détail des
mécanismes, qui seront tranchés par expérience quand le code arrivera.

Dernière révision : 2026-09-01.

Ce document est ce que pointe la tranchée T-4. Il existe parce que le modèle de temps ne
vivait que dans une ligne du registre et un paragraphe de `03_DATA_MODEL.md`, et que
personne n'avait fait l'arithmétique.

---

## Le mur, chiffré

En 0.0.1 :

- 1 tick = 1 heure-monde (`[time] tick_duration_seconds = 3600`).
- Espérance de vie moyenne : `lifespan_ticks_mean = 20000` ticks, soit environ 833
  jours-monde, environ 2,3 années-monde.
- Une génération effective : de l'ordre de 1000 à 1500 ticks (maturité vers 1000 ticks, puis
  gestation d'environ 700 ticks entre deux scissions).
- Runs faits jusqu'ici : 20 000 à 60 000 ticks, soit 2,3 à 6,8 années-monde, soit 15 à 60
  générations au plus.
- Débit mesuré : de l'ordre de quelques milliers de ticks par seconde pour un monde de
  5000 entités, et **ça se dégrade quand la population monte** (le coût de perception est
  proportionnel à la population).

L'évolution biologique demande des milliers de générations. Un million d'années-monde, c'est
environ 8,76 milliards de ticks. À pleine résolution, même pour un monde trivial, c'est des
semaines de calcul ininterrompu, et c'est sans borne pour un monde peuplé. La conclusion est
nette : **à pleine résolution on n'atteint jamais le temps évolutif.** Il faut compresser
les périodes où il ne se passe rien d'important.

Note : la formule "la sim vise environ 60 fois le temps réel" (`00_INDEX.md`,
`genesis.starter.toml`) mélange deux choses. `target_ticks_per_real_second = 60` est un
débit de calcul visé. Avec 1 tick = 3600 secondes-monde, 60 ticks par seconde réelle, c'est
216 000 fois le temps réel, pas 60. Le "60 fois" n'a pas de sens comme ratio de compression.
Ce document remplace cette formulation par le modèle à deux horloges ci-dessous.

---

## Deux horloges

Décision utilisateur du 2026-09-01. Le moteur tient deux horloges et bascule de l'une à
l'autre selon la complexité mesurée du monde et la marche d'échelle (voir `10_ROADMAP.md`,
"L'escalier des échelles").

### Horloge fine

Le tick actuel : un pas fixe de temps-monde, défini par ère (T-4 inchangé, en 0.0.1 une
seule ère, 1 tick = 1 heure-monde). Chaque entité est simulée à chaque tick, pipeline en 9
phases. C'est l'horloge de l'observation : le drame, les naissances et les morts qu'on
regarde, les ères denses en événements, tout ce qui mérite le grain fin.

### Horloge grossière

Un pas d'ère. Il ne simule pas chaque entité à chaque tick. Il applique en gros, sur la
durée du pas :

- le renouvellement des générations (qui meurt, qui se reproduit, combien),
- la sélection sur les distributions de traits (les traits favorisés montent, les autres
  descendent, selon la pression mesurée avant le pas),
- la dérive génétique (variance qui bouge, lignées qui s'éteignent ou dominent),
- la dynamique de population et de ressources (la boucle organisme vers milieu, mais sur
  des moyennes),
- les événements saillants qui tombent pendant le pas (une extinction de lignée, une
  émergence d'espèce), reconstitués depuis les statistiques.

Le pas grossier est déterministe : même graine, même config, même version donnent le même
résultat agrégé. Il puise dans le même RNG que l'horloge fine, dans un ordre fixe.

Ce n'est pas "sauter des ticks et espérer". C'est une mise à jour générationnelle
statistique, cohérente avec l'agrégation déjà décrite dans `02_ARCHITECTURE.md` ("la
pyramide") : ce qu'on garde d'une année compressée, c'est la fourchette de population, les
naissances et morts, les lignées dominantes, la moyenne et l'écart-type de chaque trait, les
événements saillants. On perd le frémissement tick par tick, on garde la forme et les
bascules.

---

## Le détecteur d'ère stable

Le passage à l'horloge grossière est déclenché, pas décidé à la main. Il réutilise le signal
déjà construit pour le Dézoomer (saillance des événements, veilleurs, config `[watch]`).

Une ère est jugée stable sur une fenêtre glissante si, sur cette fenêtre :

- peu ou pas d'événements au dessus d'un seuil de saillance,
- population à plat (dans une fourchette étroite),
- dérive génétique lente (la moyenne des traits bouge peu, pas de nouvelle espèce détectée),
- pas de bascule d'échelle en cours.

Quand ces conditions tiennent, le moteur passe en horloge grossière et avance par pas
d'ère. Dès qu'un seuil est franchi (un événement saillant, une inflexion de population, une
espèce qui émerge, une bascule d'échelle), il repasse en horloge fine et reprend le grain
tick par tick autour de l'événement.

Contrainte de fidélité : le moteur ne ralentit jamais pour le spectacle et n'accélère
jamais parce qu'un monde est ennuyeux à regarder. La bascule dépend de la complexité et de
la densité d'événements mesurées, jamais du focus du joueur (T-5). Un monde compressé pendant
une absence puis rejoué doit donner le même état.

---

## Régime de compression par marche d'échelle

Chaque marche de l'escalier a son régime. Repris de l'esprit du document gelé "Temporal
Simulation & Time Perception", mais mécanisé et déterministe (la version d'origine était
pilotée par le ressenti du joueur, ce qui a été retiré, voir `GENESIS_FIDELITY.md`).

| Marche | Régime attendu | Pourquoi |
|---|---|---|
| Chimie, molécules | très compressé, pas d'ère larges | des millions d'années où rien de nommable ne se passe |
| Biologie, cellules, organismes | compressé, pas d'ère par milliers de générations | la sélection travaille lentement, on veut la forme pas le détail |
| Individus, groupes | grain moyen, horloge fine sur les vies saillantes | une biographie mérite le tick |
| Sociétés, civilisations | grain fin, horloge grossière seulement sur les siècles calmes | les décisions, les mythes, les guerres méritent l'heure, parfois la minute |

L'"Auto-Speed" est donc une fonction de la complexité mesurée du monde : plus il y a
d'événements saillants, de lignées, de structure sociale, plus le moteur reste en horloge
fine.

---

## Conséquence sur les freins de reproduction

Les freins de 0.0.1 (gestation, maturité, échec environnemental, surpopulation locale, voir
`00_INDEX.md` section D) sont réglés pour garder des mondes de quelques centaines d'entités,
observables. C'est bon au stade molécule, en horloge fine.

En ère biologique compressée, en horloge grossière, le renouvellement des générations est
statistique. Les freins ne throttlent plus le temps de calcul : ils façonnent la
distribution (quelle fraction se reproduit par pas, quelle variance de descendance, quel
plancher d'échec). Le même paramètre a deux rôles selon l'horloge.

À reporter comme note dans `00_INDEX.md` section D, en face de la ligne "Freins de
réplication".

---

## Ce que ce document ne tranche pas

- La forme exacte du pas grossier (mise à jour analytique des distributions, ou
  micro-simulation d'un échantillon représentatif, ou les deux selon la marche).
- Les seuils précis du détecteur d'ère stable (à régler par expérience, tranchée 13).
- Le format de stockage d'une ère compressée (recoupe le lot 4 du Dézoomer, rétention
  temporelle géométrique).
- L'interaction entre l'horloge grossière et le LLM à partir de 0.0.5 (un pas d'ère ne peut
  pas contenir d'appels LLM ; les périodes qui en ont besoin restent en horloge fine).

Ces points seront tranchés quand le code de T-4 sera écrit. Le présent document fixe le
principe : deux horloges, bascule mécanisée sur la complexité, pas grossier déterministe et
agrégé.
