# 010. Experience 00X, la croyance sur le vrai moteur

Statut : **plan**. Rien d'implemente. Gabarit de `001_emergence.md`. C'est l'experience 00X
que `001` appelle explicitement : "a refaire sur le vrai moteur (0.0.3+), avec de vrais
agents, une vraie memoire, un vrai graphe social".

Position : le pont entre 0.0.4 (Voix) et 0.0.5 (Societe). C'est **le** test du pari du projet
(T-7, T-8) : un mythe se detache-t-il d'un fait par une regle qui consomme memoire, signaux
et contact, jamais par un `if` qui le nomme ? Si ce pari ne tient pas ici, il faut le savoir
avant de batir la couche sociale.

Requiert un redemarrage propre du moteur (bump de schema, regeneration des mondes). A ne pas
lancer pendant qu'un monde tourne en direct.

---

## Ce que le moteur a deja

- `Mind.episodic: Vec<Memory>` : souvenirs de lieux, chacun `Peril | Bounty | Witnessed`,
  avec une `strength` qui decroit et, pour les ancres, un `event_seq` qui pointe le fait
  objectif (invariant 5).
- `Mind.social: Vec<SocialTie>` : qui l'agent reconnait, avec `familiarity` et `valence`.
- `WorldState.signals` : le canal de la Voix. Aujourd'hui un seul signal (l'alarme de
  famine), sans genre, sans contenu.
- Le renouvellement des generations est reel : les agents meurent, de nouveaux s'eveillent
  sans rien savoir.

Il manque : une **croyance** qui n'est pas un souvenir de premiere main, qui circule, qui se
deforme, et qui peut resister a la correction.

## Ce qu'on ajoute

### 1. La croyance de lieu (`Mind`)

```
struct PlaceBelief {
    cell: (u16, u16),      // case de la grille, quantifiee (comme les souvenirs)
    danger: f32,           // 0..1, la croyance : "cet endroit tue"
    conviction: f32,       // 0..1, a quel point elle est tenue
    heard_count: u16,      // combien de fois entendue (poids de repetition)
    source: Option<EntityId>, // de qui on la tient (None si premiere main)
}
```

`Mind.beliefs: Vec<PlaceBelief>`, bornee (la moins convaincue cede la place). Distincte de
`episodic` : un souvenir est un fait vecu, une croyance est un dire.

### 2. La mise a jour (phase 5c, cognition, sequentielle sans RNG sauf le tirage explicite)

Chaque tick, pour chaque agent eveille :

- **Depuis l'experience** (poids fort) : s'il est sur une case et y prend un choc de peril
  ou une aubaine, sa croyance sur cette case bouge vers l'observation, **attenuee par
  `(1 - conviction)`**. Un agent tres convaincu ne corrige presque plus. Cette resistance
  n'est pas une regle qui dit "defends ton mythe" : elle sort de `conviction`.
- **Depuis les signaux** (poids faible, deformant) : les agents a portee d'un signal recu
  ce tick (la Voix, etendue) absorbent la croyance portee, vers une moyenne ponderee par la
  `conviction` de l'emetteur et leur familiarite avec lui (`Mind.social`). Chaque absorption
  ajoute un petit bruit fixe : **la transmission deforme**.
- **Repetition -> conviction** : entendre la meme direction (danger monte / danger baisse)
  incremente `heard_count` et fait monter `conviction`. C'est le mecanisme du mythe qui
  s'installe (V2 du prototype `001`).
- **Ancre qui se degrade** : `conviction` d'une croyance de premiere main decroit avec le
  temps ecoule depuis l'observation. Passe un seuil, l'agent ne distingue plus sa croyance
  d'un dire. Les temoins meurent, le souvenir devient legende.

### 3. Le signal etendu (`WorldState.signals`, la Voix)

`Signal` gagne un genre et une charge :

```
enum SignalKind { Alarm, PlaceDanger, PlaceSafe }
struct Signal { pos, born, kind: SignalKind, payload_cell: Option<(u16,u16)>, from: EntityId }
```

Un agent qui tient une croyance forte sur une case proche l'emet (un `PlaceDanger` ou
`PlaceSafe`), pas seulement quand il a faim. Aucun lexique : `PlaceDanger` et `PlaceSafe`
sont deux entrees fixes, comme un cri d'alarme et un cri d'appel chez un animal. La
**cartographie** (quelle situation declenche quel signal) reste fixe en 00X ; la faire
evoluer (chaque lignee sa version) est la suite, `06_EMERGENCE.md`.

### 4. L'histoire de la cause (pour l'institution, sans verite a comparer)

Chaque croyance porte un axe supplementaire `story: f32` dans [0, 1] : un bout "naturel"
(l'endroit est juste pauvre), l'autre "engage" (l'endroit est marque, il faut l'eviter par
principe). **Cet axe n'a aucune verite objective dans le monde.** Il circule avec la
croyance, se deforme, se renforce par repetition. Un sous-groupe qui converge etroitement
sur un `story` engage, y resiste, et pese sur les autres, c'est une institution non
declaree.

## Les mesures (calculees depuis l'etat du monde, jamais depuis une regle)

```
divergence(t)  = moyenne sur (agent, case) de |belief.danger - danger_reel(case, t)|
consensus(t)   = fraction d'agents dont la croyance sur une case donnee est a moins d'eps
                 du mode de la population, moyenne sur les cases connues
detachement    = consensus reste haut apres qu'une case a change d'etat objectif
                 (le danger reel a disparu, la croyance persiste). Un mythe.
institution    = existe-t-il un sous-groupe stable qui :
                   a) partage un belief.story de facon serree,
                   b) a une conviction moyenne qui ne baisse pas quand l'experience contredit,
                   c) emet plus de signaux qu'il n'en absorbe (asymetrie d'influence positive).
turnover_myth  = un mythe survit-il au remplacement complet de la generation qui l'a cree.
```

Un evenement de veille `MythTook { cell, generation }` peut marquer, pour la chronique et
l'overlay, le tick ou le detachement franchit un seuil et tient. Detecte, jamais devine.

## A/B, meme graine (T-16)

| Variante | Ce qu'on coupe | Attendu |
|---|---|---|
| V0 | pas de signaux de croyance, pas de story | divergence colle au fait, aucun mythe |
| V1 | + signaux, transmission fidele (bruit = 0) | consensus plus vite, un retard apres bascule |
| V2 | + transmission deformante + repetition->conviction | mythes qui survivent au fait |
| V3 | + effet fondateur (bonus d'emission durable aux premiers convaincus) | noyaux type institution |

Bouton d'A/B global : `[belief] enabled = false` rend la tranche inerte (aucune croyance,
aucun signal de croyance). Un monde par defaut alors identique a v15.

## Critere de reussite

Comme `001` : **V2 ou V3 produit du detachement et un noyau institution, sans aucune regle
qui nomme le mythe ou l'institution.** Si ni V2 ni V3 n'y arrivent sur le vrai moteur, avec
de vrais agents, une vraie memoire ancree et un vrai graphe social, alors le pari de
l'emergence doit etre repense avant 0.0.5. C'est exactement ce qu'on veut savoir tot.

Verification standard : `cargo build` propre, determinisme byte-identique 1 vs 8 threads (la
phase 5c est sequentielle, un seul tirage RNG explicite par emission), `replay` OK, le monde
par defaut inchange, un test d'invariant (`belief_stays_bounded`, `myth_can_detach`).

## Impact schema

`Mind.beliefs`, `Signal.kind` / `payload_cell` / `from` : bump de schema (v16). `[belief]`
nouveau bloc de config. `EventKind::MythTook`. Regeneration des quatre mondes de
demonstration. Aucun autre invariant touche (la population, la matiere, l'energie ne
bougent pas : une croyance ne coute rien, elle biaise seulement le deplacement, comme le
souvenir depuis 0.0.3).

## Ce que ca ouvre

Si le detachement tient : la couche 0.0.5 (memoire collective, consensus, premier LLM en
lecture du Voile) a un vrai substrat. Le Voile, cote `genesis.nodyx.org`, pourrait alors
afficher, pour un monde, la carte de ses mythes : quelles cases sont crues dangereuses,
depuis quand, par quelle lignee, et si c'est vrai. Un humain lit ce qu'un monde a decide de
croire pendant qu'il ne regardait pas.
