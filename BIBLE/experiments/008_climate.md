# 008. Experience, le climat branche sur la simulation

Statut : **temperature et gravite implementees et actives** (2026-09-02). Gabarit de
`001_emergence.md`.

Position : la section `[planet]` de la config (`temperature_c`, `medium`, `gravity`,
`pressure_atm`) existe depuis 2026-09-01, mais elle n'etait qu'**affichee**. Choix delibere
a l'epoque : ne pas rejouer le whack-a-mole d'equilibrage tant que la population n'etait pas
stable et selectionnee. Elle l'est (0.0.2 complet, 0.0.3 atteint). On branche.

---

## Question

1. Deux constantes physiques du monde, la temperature et la gravite, peuvent-elles
   **faconner la vie** de facon mesurable, sans casser les mondes ?
2. Le defaut reste-t-il **inerte** (aucune regression sur les mondes existants) ?
3. De facon **deterministe** ?

## Montage

Aucun etat nouveau, aucun bump de schema : le climat est une propriete de la config, pas du
`WorldState`. Deux effets, chacun un simple facteur multiplicatif calcule une fois par tick.

- **Temperature**, phase 5 (metabolisme) : `base_burn *= 1 + temp_metab_slope *
  |temperature_c - temp_optimal_c|`. Un monde loin de sa temperature optimale coute plus
  cher a habiter. Le surcout frappe la depense de base de toutes les entites, egalement.
  `temp_metab_slope = 0` ou `temperature_c == temp_optimal_c` : facteur = 1, aucun effet.
- **Gravite**, phase 4 (mouvement) : `move_cost *= gravity`. Un monde lourd rencherit chaque
  deplacement. `gravity = 1` (Terre) : aucun effet.

Nouvelles cles `[planet]` : `temp_optimal_c` (15), `temp_metab_slope` (0,012). Defaut :
`temperature_c = temp_optimal_c = 15`, `gravity = 1`. Un monde par defaut se comporte a
l'identique ; `worlds/w1` et `w4` rejouent `deterministe : OK` sans regeneration.

## Resultats

Graine 1, 50 000 ticks. Reference : monde tempere (15 C, a l'optimum, 1 g).

| mesure | tempere 15 C | froid 2 C | chaud 35 C | lourd 1,6 g |
|---|---|---|---|---|
| population finale | 2292 | 2286 | 2289 | 2292 |
| morts par famine | 4644 | 5015 (+8 %) | 5754 (+24 %) | 5124 (+10 %) |
| morts par age | 2050 | 1929 | 1495 (-27 %) | 2067 |
| generation max | 17 | 16 | **20** | 20 |
| diversite genetique finale | 0,099 | **0,065 (-34 %)** | 0,096 | 0,079 (-20 %) |
| efficacite moyenne | 0,60 | 0,60 | 0,63 | 0,61 |
| vitesse moyenne | 0,39 | 0,38 | **0,54 (+36 %)** | 0,41 |

Lecture :

- **Monde froid** (13 C sous l'optimum, +16 % de depense de base) : la diversite genetique
  s'effondre d'un tiers. La taxe thermique constante resserre la selection : la population
  converge plus dur vers le genome le plus econome, elle perd sa variete. Les creatures sont
  aussi un peu plus lentes (moins d'energie a depenser en deplacement). Un monde dur et
  homogeneisant.
- **Monde chaud** (20 C au-dessus, +24 %) : +24 % de morts par famine, mais -27 % de morts
  par age : on meurt de faim avant d'avoir le temps de vieillir. Le renouvellement s'accelere,
  d'ou 20 generations au lieu de 17. La vitesse moyenne bondit de 36 % : dans un monde qui
  coute cher, il faut se deplacer vite pour trouver a manger, et etre efficace. Un monde
  frenetique qui evolue vite.
- **Monde lourd** (deplacement +60 %) : +10 % de morts par famine, l'efficacite est poussee
  vers le haut, la diversite baisse de 20 %. La vitesse ne change quasi pas : il faut
  toujours bouger, on paie juste plus.

Meme graine, autre planete, autre vie. Le defaut est inerte : la reference tempere ci-dessus
est byte-identique a un monde d'avant le branchement.

Determinisme byte-identique 1 vs 8 threads (config par defaut et config froide). `replay` OK
sur un monde froid (la config, avec `temp_optimal_c` et `temp_metab_slope`, fait l'aller-
retour par `config.toml`). Test `climate_shapes_the_world` : un monde a 0 C voit plus de
morts par famine qu'un monde a l'optimum, l'effet est deterministe.

L'overlay du direct (`stream.html`) mene son bandeau du bas par le climat : milieu, degres
Celsius, gravite, atmospheres. Vraies constantes, aucun chiffre invente.

## Ce qui reste

- **Milieu** (eau / acide / air) : change ce qui est comestible et l'efficacite metabolique.
- **Pression** : affichee, sans effet mecanise. A brancher sur l'efficacite ou l'acces au feu
  (donc a la technique) plus tard.
- **Gravite** : plafonner en plus la taille corporelle (trait), contraindre les structures et
  le vol quand le stade organisme sera la.
- **Cataclysme** : `00_INDEX.md` note qu'un evenement rare pourrait decaler ces constantes
  (volcanisme, impact). Non implemente : le climat est encore constant sur la vie du monde.
