# 03. Modèle de données minimal, Genesis 0.0.1

Statut : PROPOSED. Périmètre 0.0.1 uniquement. Les valeurs de config sont des points de départ (tranchée 12).
Base : `Nodyx-Genesis_2` (CORE_DATA_MODEL), réduit au strict périmètre 0.0.1.

Périmètre 0.0.1 : deux entités, énergie, mouvement, reproduction, mutation, mort,
persistance, graine déterministe. Pas de mémoire, pas de LLM, pas de Nodyx, pas de culture.

Respecte les 10 invariants de l'audit, la tranchée 4 (temps), la tranchée 5 (déterminisme),
la tranchée 12 (des chiffres de départ), la tranchée 17 (construit pour durer).

---

## Identifiants

`UUIDv7` partout. Triable dans le temps, générable en local, compatible PostgreSQL, pas de
compteur global. Un identifiant n'est jamais réutilisé.

```
type WorldId  = Uuid
type EntityId = Uuid
type EventId  = Uuid
type GenomeId = Uuid
```

## Temps

```
struct GenesisTime { tick: u64 }
```

1 tick vaut une durée fixe de temps-monde, définie par ère. En 0.0.1 il y a une seule ère,
1 tick = 1 heure-monde. La compression consiste à sauter ou agréger des ticks, jamais à
changer ce qu'un tick vaut (tranchée 4). Les budgets s'expriment en secondes réelles, pas
en ticks. Le modèle complet (deux horloges, détecteur d'ère stable, pas grossier agrégé) est
dans `04_SIMULATION.md`.

## WorldState

Racine logique d'un monde. C'est la vérité objective. Seuls les systèmes de simulation
écrivent dedans.

```
struct WorldState {
    world_id:        WorldId,
    tick:            u64,
    seed:            u64,
    schema_version:  u32,       // tranchée 17 : migrations
    engine_version:  String,    // tranchée 17 : un monde sait quel moteur l'a fait naître

    space:           Space,
    resources:       ResourceField,
    entities:        Vec<Entity>,   // trié par id croissant, dense (schema v3)
    next_entity_id:  EntityId,      // monotone : un nouveau-né a toujours l'id le plus haut

    rng:             DeterministicRng,   // l'état du RNG fait partie de l'état du monde
    next_event_seq:  u64,   // (schema v6) prochain seq d'événement, incrémenté à la création dans tick()

    free_matter:              f32,   // (0.0.2) matière structurelle libre, voir Matière ci-dessous
    repro_blocked_materials:  u64,   // (0.0.2) divisions calées faute de matière, cumulé

    cells:            Vec<Cell>,     // (0.0.2, tranche 2) cellules vivantes, triées par id
    next_cell_id:     u32,
}

struct Watch {                       // état des veilleurs (phase 8b), voir 07_EVENTS.md
    // ... paliers, historique de population, lignées, séries d'espèces, cell_pending ...
    deaths_since_check:          Vec<u64>,          // (v6) seq des EntityDied depuis le dernier contrôle, plafonné à 128
    last_death_seq_by_lineage:   BTreeMap<u16,u64>, // (v6) dernière mort connue par lignée fondatrice
}

struct Cell {                        // un amas cohérent de parents, reconnu comme unité
    id:            u32,
    formed_tick:   u64,
    position:      Position,          // centroïde des membres
    radius:        f32,
    member_count:  u32,
    genome_key:    u16,               // signature quantifiée, comme l'espèce
    mean_traits:   [f32; 7],
}
```

## Cellules (0.0.2, tranche 2)

Étape 1, « membrane » : un amas d'entités proches, génétiquement parentes, cohésives et
persistant devient une `Cell`. Les membres **restent** dans `WorldState.entities`, taggués
`Entity.cell_id: Option<u32>`. Détecteur en phase 5b de `sim.rs`, sur le modèle du détecteur
d'espèce (union-find + persistance via `Watch.cell_pending`). Être en cellule apporte le
partage d'énergie (chaque membre tend vers la moyenne du groupe) et une reproduction
protégée (`birth_loss` multiplié par `1 - cell_birth_relief`). Réversible : dissolution si
la cellule se disperse, rétrécit sous `min_members`, ou voit sa cohésion moyenne retomber.

L'étape 2 dé-simulera les membres (ils quittent `entities`, la cellule devient un bilan) :
`population()` et les invariants de conservation gagneront alors un terme cellule. En étape 1
les invariants sont **inchangés**. Config : bloc `[cells]`.

## Matière structurelle (briques, 0.0.2)

Seconde ressource, distincte de l'énergie. Le monde contient une quantité finie de matière,
`matter_per_cell * nombre de cases`. Un corps vivant en immobilise `body_matter` ; le reste
est `WorldState.free_matter`. Une division prend `body_matter` du stock libre pour bâtir
l'enfant ; si le stock est à sec, la division cale (`ReplicationFail::Materials`) et le
parent patiente `retry_frac` de sa gestation. La mort rend `body_matter` au stock.

Invariant : `free_matter + population * body_matter` est constant, à chaque tick (testé,
`structural_matter_is_conserved`). C'est le vrai frein de capacité : la population plafonne
autour de `matter_per_cell * cases / body_matter`.

V2 (plateau adouci) : quand `free_matter` descend sous un coussin (`comfort_frac` de la
matière totale), la division devient probabiliste, sa chance décroît linéairement jusqu'à 0
quand il ne reste que la matière d'un corps. La population respire dans cette bande au lieu
d'être épinglée au plafond. Un tirage RNG par candidat concerné, dans l'ordre des `EntityId`
(phase 7 séquentielle) : le déterminisme tient.

Non spatial en 0.0.2 (une géographie de la matière viendra avec les biomes). Config : bloc
`[bricks]`.

## Espace

En 0.0.1, une grille bornée : des bords, pas de repli toroïdal. Un monde a des limites,
et la géographie des ressources compte (les coins, le centre).

Le troupeau tend à se concentrer là où la fertilité est la plus riche (souvent un bord,
selon la géographie propre à la graine). Une répulsion des bords (`edge_correct`) a été
essayée en 0.0.3 puis retirée : elle faussait la capacité de charge. Voir `00_INDEX.md`.

```
struct Space { width: u32, height: u32 }

struct ResourceField {
    cell:        Vec<f32>,   // longueur width*height, énergie disponible sur la case
    regen_rate:  f32,        // fraction de max_per_cell régénérée par tick
    max_per_cell: f32,
}
```

## Entité

En 0.0.1, une entité est un organisme sans cognition. À partir de 0.0.3 elle peut porter un
esprit (`mind`) : elle devient alors un agent.

```
struct Entity {
    id:         EntityId,
    genome_id:  GenomeId,
    generation: u32,

    position:   Position,   // coordonnées grille, en f32 (mouvement sous-case)
    energy:     f32,        // <= starve_at signifie mort. Source de vérité de l'énergie, ici.
    age_ticks:  u64,

    cell_id:    Option<u32>,        // (0.0.2) cellule d'appartenance, None = molécule libre
    mind:       Option<Box<Mind>>,  // (0.0.3, schema v7) esprit d'agent, None pour la quasi-totalité
    last_shock: Option<Shock>,      // (0.0.3) dernier choc marquant, graine d'un souvenir
}

struct Position { x: f32, y: f32 }
```

Réponse à un manque de l'audit : l'énergie appartient à `WorldState.entities[id].energy`.
Seul le système Métabolisme l'écrit.

## Cognition (0.0.3, tranche 1)

Une entité s'éveille en agent (phase 5c de `sim.rs`) quand elle perçoit assez, a assez vécu
et vient de subir un choc. Elle gagne un `Mind`. Réversible : sans souvenir depuis
`lapse_ticks`, elle retombe. Détails et paramètres : `05_COGNITION.md`, config `[cognition]`.

```
struct Mind {
    awoke_tick: u64,
    episodic:   Vec<Memory>,   // borné à max_memories, le plus faible cède la place
    needs:      Needs,         // (0.0.3 tranche 4) faim, peur, solitude ; pondèrent le comportement
    mode:       BehaviorMode,  // (0.0.3 tranche 6) forage | flee | join | seek_bounty | wander
    social:     Vec<SocialTie>,// (0.0.3 tranche 7) relations vers d'autres agents, borné à 8
}

struct Needs { hunger: f32, fear: f32, solitude: f32 }   // chacune dans [0, 1]

struct SocialTie {
    other:       EntityId,
    familiarity: f32,   // (0, 1] : monte quand l'agent est proche de `other`, décroît lentement
    valence:     f32,   // [-1, 1] : + si l'agent se sentait bien près de `other`, - sinon
}

struct Memory {
    formed_tick: u64,
    place:       Position,
    kind:        MemoryKind,     // Peril | Bounty | Witnessed (mort d'un proche vue, 0.0.3 tranche 3)
    event_seq:   Option<u64>,    // lien vers le fait objectif (Event.seq). Invariant 5 : la
                                 // mémoire subjective ne réécrit jamais l'histoire objective,
                                 // la divergence se mesure, elle ne se corrige pas.
                                 // None pour Peril (famine subjective) ; Some(seq de l'EntityDied) pour Witnessed.
    strength:    f32,            // (0, 1], décroît * memory_decay par tick, oubli sous memory_eps
}

struct Shock { tick: u64, place: Position, peril: bool }   // écrit pour toutes les entités
```

La mémoire biaise le déplacement de l'agent (phase 2/3, hors des lieux de péril et de mort
vue, vers les lieux d'aubaine). En tranche 3, un agent témoin d'un `EntityDied` proche (un
membre de sa lignée) en garde un souvenir `Witnessed` ancré sur ce `seq` : c'est le premier
souvenir vérifiable au sens fort. En tranche 4, les trois besoins (mis à jour en phase 5c)
pondèrent cette même cible : la faim pousse à foncer manger, la peur fait fuir le danger
même affamé, la solitude fait dériver vers les siens. En tranche 6, `Mind.mode` retient
laquelle de ces forces a **dominé** la décision (une lecture, pas un changement de
comportement) : la biographie peut alors dire « elle a fui plutôt que de manger ».
La dé-simulation de la biologie sous l'agent est une tranche suivante.

## Génome

```
struct Genome {
    id:        GenomeId,
    traits:    GenomeTraits,
    parent_a:  Option<EntityId>,
    parent_b:  Option<EntityId>,
}

struct GenomeTraits {   // 9 traits, tous normalisés 0..1 (genome.rs::N_TRAITS)
    // -- 7 traits de corps (genome.rs::SPECIES_TRAITS ; seuls eux forment la signature d'espèce) :
    metabolism:  f32,   // énergie brûlée par tick
    speed:       f32,   // distance de déplacement max par tick
    perception:  f32,   // rayon de détection des ressources
    efficiency:  f32,   // énergie gagnée par unité de ressource mangée
    fertility:   f32,   // rythme de réplication : gestation = base * (1.5 - fertility)
    lifespan:    f32,   // âge auquel la probabilité de mort monte
    cohesion:    f32,   // retenue sur les communs quand l'entité est entourée de parents
    // -- 2 traits de personnalité (0.0.3 tranche 5, hérités, ne comptent pas dans l'espèce) :
    caution:     f32,   // haut = l'agent évite plus fort ses souvenirs de danger
    curiosity:   f32,   // haut = l'agent est plus attiré par ses souvenirs d'abondance
}
```

Reproduction asexuée en 0.0.1, stade molécule : une entité accumule de l'énergie
(`energy_threshold`), se scinde, chaque moitié copie le génome trait par trait avec
micro-mutation gaussienne. Une mutation peut être létale (`lethal_mutation_rate`) : pas
d'enfant. La division échoue aussi sans infrastructure (`birth_loss_base`) et sur une case
surpeuplée (`crowding_half`). L'enfant se détache d'environ une case du parent, l'énergie
est partagée en deux. La reproduction sexuée arrive à 0.0.2 et ne s'impose jamais, voir
`00_INDEX.md` « émergence du sexué ». Code : `genome.rs::divide`, `sim.rs` phase 7.

## Événement

Journal append-only. Un événement validé est immuable. Une correction est un nouvel
événement (INV-005).

```
struct Event {
    seq:            u64,          // clé d'ordre, monotone par monde. Attribué à la création dans tick() (schema v6), plus à l'écriture
    tick:           u64,
    kind:           EventKind,    // enum à données, pas de payload séparé en 0.0.1
    salience:       u8,           // 0 bruit .. 255 genèse. base_salience(kind), peut monter
    causes:         Vec<u64>,     // seq des événements causes. Peuplé (0.0.2) pour PopulationCrash et LineageExtinct ; vide ailleurs jusqu'à 0.0.6
    cascade_depth:  u16,          // reste 0 jusqu'à 0.0.6 ; le garde-fou actif est max_events_per_tick
}

enum EventKind {
    WorldCreated,
    EntitySpawned,       // dont les deux entités initiales
    EntityAte,           // défini, non émis en 0.0.1 (trop bruyant)
    EntityDivided,       // scission : parent -> enfant
    ReplicationFailed,   // division sans enfant. reason : LethalMutation | Environment | Materials (0.0.2)
    EntityDied,          // cause : Starvation | Age
    SnapshotTaken,
    // veilleurs (sim.rs phase 8b), saillance haute :
    LineageExtinct { lineage: u16 },
    SpeciesEmerged { species: u32, size: u32 },
    PopulationMilestone { level: u32 },
    PopulationCrash { from: u32, to: u32 },
    CellFormed { cell: u32, size: u32 },   // (0.0.2, tranche 2)
    CellDissolved { cell: u32 },
    AgentAwoke { entity: EntityId },        // (0.0.3, tranche 1) une entité s'éveille en agent
    AgentLapsed { entity: EntityId },       // ... et retombe entité de fond. Réversible.
}
```

Décision de longévité (tranchée 17) : le mouvement n'est **pas** un événement. Logger chaque
déplacement de chaque entité à chaque tick ferait exploser le journal. La position est de
l'état courant, reconstructible depuis les instantanés. Le journal ne porte que le squelette
causal : naissance, repas, reproduction, mort.

Traçabilité causale (0.0.2, tranche 3b) : les `seq` sont attribués à la création (dans
`tick()`, via `WorldState.next_event_seq`), pas à l'écriture, pour qu'un événement de veille
puisse citer les événements qui l'ont causé. Deux liens sûrs et bon marché sont câblés :
`PopulationCrash` cite la vague de `EntityDied` sur la fenêtre (`Watch.deaths_since_check`),
`LineageExtinct` cite la dernière mort d'un membre de cette lignée
(`Watch.last_death_seq_by_lineage`). Les autres `EventKind` gardent `causes` vide : le graphe
causal complet, l'autopsie « trois candidats de bascule », c'est 0.0.6 (tranchée 15).

## Persistance

Trois mécanismes, comme dans `Nodyx-Genesis_2`, dimensionnés pour durer.

```
WorldState  ->  ligne(s) courante(s) dans PostgreSQL, un monde logique
Event Log   ->  table append-only, partitionnée par plage de ticks,
                vieilles partitions compressées puis archivées en froid
Snapshot    ->  WorldState sérialisé entier (postcard/bincode) tous les N ticks,
                les anciens éclaircis (on garde 1 sur 10, puis 1 sur 100)
```

Récupération = dernier instantané + rejeu des événements depuis. L'instantané inclut l'état
du RNG, sinon le rejeu n'est pas déterministe.

## Série temporelle de stats (0.0.2, tranche 3a)

`series.jsonl` : une ligne (`genesis-view::SeriesRow`) tous les `[persistence] series_every`
ticks (500). Fonction pure de `(WorldState, config)` comme le ViewState (tranchée 3). Porte :
tick, année, population, naissances et morts (par cause), génération génomique (moyenne,
écart-type, max), diversité génétique, cellules, capacité de charge, et par trait :
`trait_mean`, `trait_spread`, et les quantiles `trait_p10` / `trait_p50` / `trait_p90`. Une
distribution, pas seulement moyenne plus écart-type : une bande p10-p90 qui se scinde, c'est
une spéciation. `series.html` en fait le graphe d'évolution (le moment public de 0.0.2).

## Contrat de déterminisme (INV-9, tranchée 5)

- Un seul RNG, graine dérivée de la graine du monde. Son état est dans `WorldState`, donc
  dans les instantanés.
- Tout tirage aléatoire (mutation, mort par âge, bruit de régénération) puise dans ce RNG
  dans un ordre fixe par tick.
- Ordre du pipeline de tick fixe (ci-dessous).
- Aucune horloge murale, aucune non-déterminisme de threads dans le cœur. Le parallélisme
  n'est autorisé que là où l'ordre n'importe pas, avec fusion déterministe des résultats.
- Résultat : même graine + même config + même version de moteur = même monde, image par
  image. Le LLM cassera ça à partir de 0.0.5, traité par rejeu des sorties enregistrées,
  hors sujet en 0.0.1.

## Pipeline de tick 0.0.1, ordre fixe

```
1. Environnement   régénération des ressources
2. Perception      chaque entité détecte les ressources dans son rayon (lecture seule)
3. Décision        chaque entité choisit une cible de déplacement
                   (règle : vers la meilleure ressource perçue, sinon errance via RNG)
4. Mouvement       application des déplacements, clamp aux bords
5. Métabolisme     dépense d'énergie (trait metabolism + coût du mouvement) ; repas si sur une case
6. Cycle de vie    age_ticks++ ; test de mort (energy <= starve_at -> Starvation ;
                   age vs lifespan -> tirage RNG)
7. Réplication     entités éligibles (énergie + gestation) -> scission : copie de génome
                   avec mutation, chance de létalité, d'échec environnemental, d'échec par
                   surpopulation ; énergie partagée en deux ; cooldown de gestation
8. Journal         écriture des événements du tick
9. Instantané      si tick % snapshot_interval == 0
```

Chaque phase se termine avant la suivante. Dans une phase, l'itération sur les entités suit
l'ordre des `EntityId` (stable), ce qui rend le parallélisme sûr.

## Config de départ (tranchée 12)

**Le fichier qui fait autorité est `BIBLE/genesis.starter.toml`,** repris à l'identique par
`crates/genesis-core/src/config.rs`. Ne pas recopier les valeurs ici : ce bloc a déjà dérivé
une fois. Ci-dessous, seulement les leviers critiques de 0.0.1, tenus synchrones avec le
fichier de référence.

Toutes ces valeurs sont des points de départ, à mesurer et ajuster. Le principe est de
partir avec des nombres, pas avec "configurable".

```toml
[time]
tick_duration_seconds        = 3600   # 1 tick = 1 heure-monde en 0.0.1
target_ticks_per_real_second = 60     # débit de calcul visé (ticks par seconde réelle),
                                      # pas un ratio de compression : voir 04_SIMULATION.md

[resources]
regen_rate    = 0.015  # fraction du plafond régénérée par passage de régen
max_per_cell  = 10.0
initial_fill  = 0.5
regen_every   = 4      # la régén ne tourne qu'un tick sur N (taux multiplié par N)

[bricks]                # matière structurelle (0.0.2), voir « Matière structurelle » ci-dessus
matter_per_cell = 0.14  # matière totale = ceci x nombre de cases ; capacité ~ total / body_matter
body_matter     = 1.0   # matière immobilisée par un corps vivant
comfort_frac    = 0.06  # V2 : coussin sous lequel la division devient probabiliste
retry_frac      = 0.4   # patience après un échec de division faute de matière

[cells]                  # cellules (0.0.2, tranche 2), voir « Cellules » ci-dessus
check_every       = 200
link_dist         = 2.0
kin_dist          = 0.7
min_cohesion      = 0.45
min_members       = 12
max_spread        = 6.0
dissolve_members  = 6      # V2, hystérésis de dissolution
dissolve_spread   = 11.0
grace_ticks       = 600
persist_checks    = 4
leave_factor      = 1.9
energy_share      = 0.15
cell_birth_relief = 0.4

[lifecycle]
starve_at            = 0.0
lifespan_ticks_mean  = 20000   # environ 2,3 années-monde, à régler
age_death_curve      = 4.0

[reproduction]
mode                 = "asexual"  # stade molécule ; le sexué arrive à 0.0.2+
energy_threshold     = 8.0
energy_cost          = 1.5
mutation_rate        = 0.05       # par trait
mutation_scale       = 0.1
lethal_mutation_rate = 0.06
gestation_ticks_base = 700        # cooldown = base * (1.5 - fertilité)
birth_loss_base      = 0.30       # échec de division sans infrastructure
crowding_half        = 1.8        # surplus sur une case pour un échec quasi certain
maturity_frac        = 0.05       # fraction de l'espérance de vie avant la 1re division

[persistence]
snapshot_interval_ticks      = 5000
event_log_partition_ticks    = 100000
series_every                 = 500    # une ligne de série temporelle de stats tous les N ticks
```

## Tests d'invariants à écrire avant le code (tranchée 13)

```
test_ids_are_uuidv7_and_unique
test_same_seed_same_config_same_world_frame_by_frame
test_event_log_is_append_only
test_correction_is_a_new_event_not_an_edit
test_dead_entity_cannot_move_eat_or_reproduce
test_snapshot_plus_replay_equals_live_state
test_rng_state_survives_snapshot_roundtrip
test_energy_only_written_by_metabolism
test_world_state_size_stays_bounded_over_1M_ticks
```

## Ce qui n'est volontairement pas là

Mémoire, personnalité, relations, cognition, LLM, culture, mèmes, institutions, Nodyx,
histoire subjective, le Voile. Tout ça arrive à partir de 0.0.3 et se conçoit dans les
specs suivantes.
