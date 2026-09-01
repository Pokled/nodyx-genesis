# 005. Experience, la capacite de charge (matiere structurelle)

Statut : **V1 implementee et active** (2026-09-01). Premiere tranche du jalon 0.0.2.
Gabarit de `001_emergence.md`.

Position dans la roadmap : le "frein propre" du stade molecule, inscrit au backlog
`00_INDEX.md` section D bis. En ligne avec la regle de subordination (`10_ROADMAP.md`) :
une population stable et selectionnee est le prerequis de la bascule cellule (tranche 2) et
donc de l'agent qui se souvient (0.0.3).

---

## Question

En 0.0.1 il n'y a pas de vrai plafond de capacite : les mondes qui ne s'eteignent pas
montent en cloche puis se stabilisent mollement sur la surexploitation (`strain`), certains
depassant 4000 entites. Le seul frein dur est `crowding_half`, un plafond de densite
artificiel par case.

Une seconde ressource, la matiere structurelle dont sont faits les corps, en quantite finie
et conservee, produit-elle :

1. un **vrai plateau** de population a une capacite de charge previsible,
2. qui ne s'active que sur les mondes qui la depassent (pas de mal aux petits),
3. sans jamais faire s'eteindre un monde qui aurait survecu sans,
4. de facon **deterministe** et a **matiere exactement conservee** ?

Si le plateau ne se forme pas, ou s'il strangle des mondes viables, le modele est a revoir.

---

## Montage

Modele global, non spatial (V1). Le monde contient `matter_per_cell * cases` de matiere.
Un corps vivant immobilise `body_matter` ; le reste est `WorldState.free_matter`.

- **Division** (`sim.rs` phase 7) : batir un enfant prend `body_matter` du stock libre.
  Stock a sec -> `ReplicationFailed { reason: Materials }`, le parent patiente `retry_frac`
  de sa gestation (pas de tentative gachee, l'energie n'est pas ponctionnee). La ponction se
  fait une fois l'enfant sur (apres les tirages d'echec environnemental et de letalite).
- **Mort** (`sim.rs` phase 6) : `body_matter` retourne au stock libre.
- **Invariant** : `free_matter + population * body_matter` constant a chaque tick. Teste par
  `structural_matter_is_conserved` (`crates/genesis-core/tests/invariants.rs`).

Config `[bricks]` : `matter_per_cell = 0.14`, `body_matter = 1.0`, `retry_frac = 0.4`.
Sur une grille 128x128 : capacite de charge = `0.14 * 16384 / 1.0` ~ 2293.

Aucun nouveau tirage RNG : le flux est inchange, le determinisme tient.

## Mesures, tracees dans le temps

Nouvelles stats dans `WorldStats` : `free_matter`, `carrying_capacity`,
`matter_locked_fraction`, `repro_blocked_materials` (cumule). Affichees dans le lecteur,
panneau « Matiere ».

## Resultat, V1 (2026-09-01)

A/B a la meme graine, 12 graines, `ticks = 40000`. ON = defaut. OFF = `matter_per_cell`
tres grand (jamais limitant).

| Graine | ON, pop finale | OFF, pop finale | Lecture |
|---|---|---|---|
| 1 | **2293** | 2774 | plafonne exactement a la capacite |
| 3 | **2293** | 3116 | montait encore a OFF, plafonne a ON |
| 5 | **2293** | 4269 | fort emballement a OFF, coupe net |
| 6 | 308 | 308 | sous la capacite : aucun effet |
| 11 | 629 | 629 | sous la capacite : aucun effet |
| 2, 4, 7, 8, 9, 10, 12 | 0 | 0 | extinctions, identiques ON et OFF |

- **Plateau net** : les mondes qui atteignent la capacite s'y collent (pop = 2293 exact),
  avec un fort renouvellement (`w2`, graine 3, 60k ticks : 22498 naissances, 20207 morts,
  pop finale 2293). `repro_blocked_materials` grimpe vite au plateau (graine 1 : ~11000 sur
  les 8000 derniers ticks), `matter_locked_fraction` -> 1.00, `free_matter` -> ~0.
- **Innocuite** : aucun monde survivant a OFF ne s'eteint a ON. Les petits mondes (6, 11)
  sont bit-identiques ON et OFF. Les 7 extinctions sont la variance habituelle (T-14),
  identiques dans les deux bras.
- **Determinisme** : snapshots et journaux byte-identiques a 1 thread et a 8 threads. Rejeu
  `w2` : OK.
- **Conservation** : exacte, testee sur 40000 ticks.

Verdict : le modele fait ce qu'on lui demande. Plateau reel, previsible, borne, conserve,
sans dommage collateral. C'est une base honnete. La courbe logistique jusqu'a la capacite
est exactement ce que le « graphe d'evolution » de 0.0.2 doit montrer.

Non spatial : la geographie de la matiere est le travail des biomes.

## Resultat, V2, plateau adouci (2026-09-01)

Choix utilisateur : adoucir le plafond dur. Quand `free_matter` descend sous
`comfort_frac` (defaut 0.06) de la matiere totale, la division devient probabiliste :
chance = `(free_matter - body_matter) / (comfort_frac * matter_total)`, plafonnee a 1,
nulle des qu'il ne reste que la matiere d'un corps. Un echec probabiliste patiente deux
fois moins (`retry_frac / 2`). Un tirage RNG par candidat concerne, en phase 7 sequentielle.

Effet (graines 1, 3, 5, plateau sur les 8000 derniers ticks d'un run de 45000) :
- La population se stabilise **un peu sous** la capacite dure (2239 a 2274 au lieu de 2293
  pile) et **respire** : graine 3, creux a 1841 puis remontee (amplitude 439) ; graines 1 et
  5, ondulations plus fines (35 et 26). L'amplitude depend de la graine et de la synchronie
  des morts (lisses ici : famine et age etales). Une oscillation ample demanderait une
  mortalite correlee (famines, maladies), travail ulterieur.
- Determinisme : byte-identique 1 vs 8 threads. Conservation : toujours exacte.
- Verdict : plateau plus vivant, sans plafond rigide, sans cout de stabilite. On garde V2
  par defaut.

## Freins de natalite : mesurer avant de relacher

`crowding_half` (frein de densite pure) est maintenant partiellement redondant avec la
capacite de charge. Non touche pour l'instant (anti whack-a-mole, T-16). A relacher dans un
A/B dedie si les mesures montrent un monde sur-freine, et le consigner ici. `maturity_frac`
reste (c'est un mecanisme, pas un nombre).

## Livrable

`worlds/w2` (graine 3, 60k ticks) : le monde de reference de la tranche. A/B reproductible
par `[bricks] matter_per_cell = 1000` pour le bras OFF.
