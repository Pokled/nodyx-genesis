# 02. Architecture

Statut : LOCKED sur le noyau et le ViewState. PROPOSED sur le détail des modules.
Base : audit des 22 documents, tranchées 2, 3, 5, 17.

---

## Les 10 invariants du noyau

Jamais contredits sur les 22 documents d'origine. Ce sont les frontières que toute évolution
future doit préserver.

1. **Le LLM propose, Genesis valide, le moteur applique.** Le LLM n'écrit jamais le World State.
2. **Nodyx ne mute jamais le World State.** Il retourne un résultat, Genesis confirme puis intègre.
3. **Genesis tourne sans LLM, sans Nodyx, sans réseau.**
4. **Toute mutation importante devient un événement.** Les événements sont immuables. Une correction est un nouvel événement.
5. **La mémoire subjective ne réécrit jamais l'histoire objective.** Ancrage `world_event_reference`.
6. **Simulation différentielle.** Tous les agents existent, pas au même niveau de détail. L'importance est dynamique et réversible, et dépend de l'agent, jamais du regard du joueur (tranchée 5).
7. **Protection anti-cascade.** `cascade_depth`, `MAX_EVENTS_PER_TICK`.
8. **Séparation vérité objective, croyance de l'agent, représentation publique.**
9. **Les appels LLM sont enregistrés pour rejeu.** Le cœur reste déterministe, les composants non déterministes sont isolés.
10. **Le pont vers Nodyx passe par un adaptateur.** Genesis ignore React, HTTP, SQL de Nodyx.

---

## Les modules et leurs frontières

| Module | Responsabilité | Possède | Écrit dans |
|---|---|---|---|
| Genesis Core | vérité du monde, temps, RNG déterministe, persistance | World State, Event Log, géographie, ressources, graine | World State, via les systèmes de simulation uniquement |
| Genesis Simulation | biologie, évolution, comportement algorithmique | rien de canonique | events |
| Agent System | état d'agent, importance, niveau de simulation | personnalité, émotions, objectifs, statut social | events, via validation |
| Memory | mémoire individuelle et collective, RAG | souvenirs, mémoire collective par entité sociale | souvenirs |
| Cognition / LLM | Model Router, Context Builder, sortie structurée, repli | rien (sans état) | un `Intent` seulement |
| Validation | Physique (bloque), Sociale (conséquences), Narrative (pression) | `ValidationResult` | consequences |
| Event System | Event Bus, Scheduler, protection de cascade, budgets | file de priorité | events |
| Culture | mèmes, transmission, consensus, langue | mémoire collective, traditions | mémoire collective |
| Civilization | territoire, gouvernement, économie, institutions | état de civilisation | events |
| Persistence | World State mutable, Event Log append-only, snapshots | fichiers, PostgreSQL | snapshots, journal |
| Nodyx Bridge | Gateway, registre d'outils, `Intent` vers exécution | `ArtifactReference` | events (retour Nodyx) |
| Veil / Human | `WorldProjectionService`, classification, isolation des secrets, pouvoirs du Gardien | projections | rien dans le World State |

Dépendance autorisée : `World -> Biology -> Behavior -> Cognition -> LLM`.
Interdit : `LLM -> World State`, `Nodyx -> World State`.

---

## Source of Truth

| Donnée | Source de vérité | Qui écrit | Statut |
|---|---|---|---|
| Position | World State, `entities[id].position` | Mouvement | LOCKED |
| Énergie | World State, `entities[id].energy` | Métabolisme seul | LOCKED |
| Santé, âge | World State, `entities[id]` | Biologie | PROPOSED |
| Génome | `Genome`, immuable après la naissance | Reproduction | PROPOSED |
| Besoins, personnalité, émotions | Agent | Comportement, plus sortie LLM validée pour les émotions | PROPOSED |
| Relation objective | World State, `RelationshipStore` | Validation sociale | PROPOSED |
| Souvenir individuel | Mémoire de l'agent | Memory Engine | LOCKED (principe) |
| Croyance | Agent, `beliefs` | sortie LLM validée, transmission | PROPOSED |
| Mémoire collective, mythe | Collective Memory, avec `origin_events[]` | transmission, consensus | LOCKED (ancrage, tranchée 8) |
| Événement historique objectif | Event Log, append-only | tout système via l'Event Bus | LOCKED |
| Représentation publique | dérivée, dans Nodyx | Event Processor | PROPOSED |
| Secrets, infra, prompts système | interne, jamais projeté | personne | LOCKED |
| Jugement humain ou IA sur un monde | annotation datée, séparée du journal | Contributeur, Gardien | LOCKED (tranchée 16) |

---

## Le contrat ViewState

Tranchée 3 : le moteur ne dessine rien, il émet un flux d'état observable. Les clients
(web, Godot, Nodyx, CLI) le consomment. Rien ne se code dans le moteur avant que ce contrat
existe, parce que sans lui il n'y a pas de retour visuel, donc pas de pilotage.

### Propriétés

1. **Lecture seule.** Un client ne peut jamais réécrire dans le monde par ce canal.
2. **Projection pure.** Un `ViewFrame` est une fonction pure de `(WorldState, bounds, lod)`.
   Deux clients avec le même instantané voient exactement la même chose. C'est ce qui fait
   marcher le rejeu et « deux personnes qui regardent le même monde ».
3. **Versionné.** `view_version` évolue indépendamment de `schema_version`. Un moteur récent
   sert un monde d'ancien schéma via une couche de compatibilité (tranchée 17).
4. **Borné.** Ce qui est loin de l'observateur ou agrégé est résumé. En 0.0.1, une frame
   porte toujours tout.
5. **Deux flux.** Frames complètes périodiques, plus deltas par tick ou par lots.
6. **Transport-agnostique.** Le contrat est le schéma, pas le fil. Transports de référence :
   WebSocket pour le web, IPC ou stdout pour Godot et la CLI, un point de collecte pour Nodyx.

### Schéma

```
struct ViewFrame {
    view_version:  u16,
    world_id:      WorldId,
    tick:          u64,
    world_clock:   WorldClock,   // année, jour, heure, lisibles
    speed:         f32,          // ticks par seconde réelle, informatif
    grid:          [u32; 2],
    lod:           "detail" | "region",  // "region" au delà de detail_max_entities
    resources:     ResourceView, // grille de densité sous-échantillonnée + fertilité + strain
    entities:      Vec<EntityView>,   // rempli en "detail"
    clusters:      Vec<ClusterView>,  // rempli en "region", entities vide
    events:        Vec<EventView>,   // événements notables depuis la frame précédente
    stats:         WorldStats,
}

struct EntityView {
    id:         EntityId,
    pos:        [f32; 2],
    energy_pct: f32,     // 0..1, jamais l'énergie brute
    age_pct:    f32,
    hue:        u16,     // teinte 0..359, calculée depuis le génome
    state:      "forage" | "eat" | "divide" | "dying",
}

struct ClusterView {
    pos:        [f32; 2],  // centroïde
    radius:     f32,       // dispersion des membres
    count:      u32,
    energy_pct: f32,
    hue:        u16,       // teinte moyenne
    state:      &str,      // action dominante
}

struct EventView {
    tick:      u64,
    kind:      String,       // "naissance", "mort", "reproduction"
    at:        [f32; 2],
    subjects:  Vec<EntityId>,
}

struct WorldStats {
    population:        u32,
    births_total:      u64,
    deaths_total:      u64,
    mean_age_ticks:    f64,
    genetic_diversity: f32,
    mean_energy_pct:   f32,
}
```

Le mouvement n'apparaît jamais comme `EventView`. Les positions vivent dans `entities`,
rafraîchies à chaque frame.

### L'inspecteur, séparé du flux

Regarder une entité en détail, ou demander « pourquoi X est arrivé », est une API
requête-réponse, pas un flux.

```
GET /world/{id}/entity/{eid}     -> EntityView complet plus l'historique de l'entité
GET /world/{id}/event/{seq}      -> l'événement plus sa chaîne causale (remontée de causes)
GET /world/{id}/snapshot/{tick}  -> pour le rejeu et le voyage dans le temps
GET /world/{id}/stats?from=&to=  -> séries temporelles pour les graphes
```

C'est là que vit la traçabilité causale. Le flux reste léger, l'inspecteur est riche à la
demande.

### Ce que le focus du joueur change, et ne change pas

Le joueur peut demander une `bounds` plus serrée, un `lod` plus fin, suivre une entité,
recevoir des notifications. Le focus ne change **jamais** ce qui est calculé dans le moteur
(tranchée 5), seulement la frame envoyée. Un agent observé et un agent non observé sont
simulés exactement pareil.

### Dézoomer : niveau de détail et saillance des événements

Quand un monde grandit, on ne peut ni tout montrer ni tout envoyer. Le regard doit pouvoir
se retirer sans que le moteur change quoi que ce soit (tranchée 5). Trois leviers, tous
côté projection.

1. **`bounds` plus la population décident du `lod`.** `Detail` : chaque entité est un
   `EntityView`. `Region` : au delà d'un seuil d'entités dans le cadre, on agrège en
   amas (`ClusterView` : centre, rayon, effectif, génome moyen, action dominante,
   énergie moyenne). `Macro` : plus que la grille de densité et les amas grossiers.
   Le client choisit `bounds` et un `lod` maximum, le moteur descend le `lod` tout seul
   si le volume l'exige. Deux clients au même cadre et au même zoom voient la même chose.

2. **Saillance des événements.** Tous les événements ne se valent pas. Chaque `EventView`
   porte un `salience: u8`. Le flux ne remonte que ce qui dépasse un seuil, fonction du
   `lod` : dézoomé, seuls les événements rares et structurants passent (extinction d'une
   lignée, premier de quelque chose, bascule démographique), une mort parmi mille est du
   bruit. La règle de score est mécanique et déterministe, jamais un jugement de valeur.

3. **Delta plutôt que frame complète.** Frames complètes espacées, puis deltas par tick
   ou par lots entre deux. C'est ce qui rend un monde observable sur des jours.

En 0.0.1 : `lod` toujours `Detail`, pas d'agrégation, `view.html` embarque des frames
complètes. C'est la limite connue. `Region` et `Macro`, la saillance et les deltas sont
le prochain chantier visuel, avant que les mondes ne durent assez pour la rendre
indispensable.

### Un monde qui ne s'arrête jamais : la pyramide

Un monde tourne pour des années. On ne peut pas garder chaque tick, ni montrer chaque
entité, ni conserver le journal complet à pleine granularité. Mais on doit pouvoir le
regarder vivre, remonter son histoire, et ne jamais perdre le squelette causal (tranchées
8 et 15). La réponse est une pyramide sur deux axes, le temps et l'espace, plus une colonne
vertébrale d'événements.

**1. Niveau de détail temporel : plus c'est vieux, plus c'est grossier.**
Échelle de rétention géométrique, comme on se souvient d'hier en détail et de 1995 en flou.
La dernière heure-monde : chaque tick (ou presque). Le dernier jour : au pas de la minute.
Le dernier mois : à l'heure. La dernière année : au jour. Au delà : au mois, puis à l'année.
Le stockage croît en logarithme du temps joué, pas linéairement. Un monde de 50 ans tient
en quelques Go.
Rétrograder une période vers un palier plus grossier n'est **pas** jeter des frames, c'est
**agréger** : l'instantané grossier d'une année, c'est des statistiques sur cette année
(fourchette de population, naissances et morts, lignées dominantes, territoire, événements
saillants). On perd le frémissement tick par tick, on garde la forme et les bascules.

**2. Niveau de détail spatial : dézoomer, c'est agréger en groupes.**
`Detail` : chaque entité. `Region` : au delà d'un seuil dans le cadre, des amas
(`ClusterView` : centre, rayon, effectif, génome moyen, comportement dominant, et un nom
dès que le groupe est reconnaissable). `Macro` : le champ de densité plus les gros amas
étiquetés. Calculé à la demande depuis le World State vivant, jamais stocké (sauf comme
partie des instantanés historiques grossiers).

**3. La colonne vertébrale d'événements : elle compresse, elle ne casse jamais.**
Récent : tous les événements. Plus ancien : seulement au dessus d'un seuil de saillance qui
monte avec l'âge. Très ancien : seulement les événements porteurs, ceux dont beaucoup
d'autres dépendent causalement, les premiers, les extinctions, les bascules.
La saillance d'un événement monte s'il est un premier, s'il est cité comme cause par
beaucoup d'événements plus tardifs, s'il est une extinction de lignée, une inflexion de
population, un déplacement de territoire, plus tard une guerre, une fondation, une découverte.

**4. Savoir quand dézoomer sur du nouveau : l'émergence est détectée, pas devinée (T-7).**
On ne marque pas « ceci est intéressant » à la main. On définit des seuils mesurables et on
laisse le monde les franchir. Exemple, une lignée devient un groupe distinct quand :
au moins N entités, profondeur d'au moins M générations depuis la souche, distance
génétique au dessus d'un seuil par rapport au stock parent, et occupation d'un territoire
propre. Quand c'est franchi, le moteur émet un événement saillant `LineageEmerged`. Le
lecteur en fait un chapitre sur la frise, un marqueur sur la carte en vue Macro, un endroit
où plonger. Plus tard, la même détection vaudra pour un premier composé chimique synthétisé,
une première institution, un premier signe tourné vers le dehors.

En pratique on découpe ce chantier en trois : la saillance des événements et les chapitres
d'abord (rend le lecteur navigable, et c'est le mécanisme de détection d'émergence), puis
l'agrégation spatiale en amas, puis la rétention temporelle géométrique côté stockage.

**Lot 1, fait (2026-09-01).** `Event.salience: u8`, copié de `kind.base_salience()`. Des
veilleurs mécanisés dans `sim.rs` (phase 8b, état dans `WorldState.watch`, réglages
`[watch]`) émettent des événements saillants : `PopulationMilestone`, `PopulationCrash`,
`LineageExtinct` (une lignée fondatrice sans descendant vivant), `SpeciesEmerged` (un
groupe de génome quantifié, assez nombreux et persistant, à distance L1 du stock dominant).
Le CLI écrit `notable.jsonl` (saillance au moins 150). Le lecteur en fait des chapitres :
liste cliquable dans le panneau gauche, marqueurs cliquables sur la frise. Le garde-fou
anti-cascade trie par saillance avant de tronquer, il garde donc les événements qui comptent.

**Lot 3, fait (2026-09-01).** `ViewFrame.lod` vaut "detail" ou "region". Au delà de
`[view] detail_max_entities` (500 par défaut), `project()` agrège les entités sur une
grille `cluster_grid x cluster_grid` : `ViewFrame.clusters: Vec<ClusterView>` (centroïde,
rayon, effectif, énergie moyenne, teinte moyenne par somme de vecteurs unités, action
dominante), `entities` est vide. Le poids d'une frame est alors plafonné (au plus
`cluster_grid^2` amas) quelle que soit la population. Le lecteur peint des amas au lieu
d'individus. Reste ouvert : le débit de calcul de la simulation elle-même, qui ne dépend
pas du lecteur. Deux leviers déjà tirés : parallélisme (`rayon`) et stockage des entités
en `Vec` trié par id (schema v3), qui ensemble ramènent 60 000 ticks de ~95 s à ~16 s.
Levier restant : LOD temporel qui saute des ticks dans les ères stables (tranchée 4).
Lot 4 : rétention temporelle géométrique côté stockage.

### Tests

```
test_viewframe_is_pure_function_of_worldstate
test_two_clients_same_snapshot_get_identical_frames
test_player_focus_does_not_change_simulation
test_view_version_compat_layer_serves_old_schema
test_stream_size_bounded_at_high_population_via_lod
```
