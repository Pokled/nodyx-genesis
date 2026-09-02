# 10. Feuille de route

Statut : LOCKED sur les 7 jalons et la cible probante. PROPOSED sur le détail de chaque
jalon. Base : tranchée T-11, section 07 de `direction/genesis-direction.html`, décisions
utilisateur du 2026-09-01.

Dernière révision : 2026-09-01.

---

## Le cap

Le projet doit produire un résultat probant sur une forme de vie intelligente et évolutive.
Pas la civilisation qui devine le dehors (l'étoile polaire, horizon long non daté), mais son
premier étage, atteignable et vérifiable.

**Cible probante minimale : un individu qui se souvient (jalon 0.0.3).** Une entité dotée
d'une mémoire, de besoins, d'une personnalité, dont le comportement dépend visiblement de ce
qu'elle a vécu. Une biographie qu'on peut lire : naissance, vie, mort, ce dont elle se
souvenait. Aucun LLM. Tout jalon antérieur (0.0.1, 0.0.2) est un moyen d'y arriver, pas une
fin.

Ce document existe parce que la route vers ce résultat n'était écrite nulle part. Les 7
jalons sont fixés depuis le document de direction, mais leurs specs (`04_SIMULATION.md`,
`05_COGNITION.md`, `06_EMERGENCE.md`) étaient des souches, et le travail réel de 0.0.1 a
dérivé vers la biologie fine et l'instrumentation. Utile, voulu par la VISION, mais c'est le
chemin de moindre résistance. Deux murs restaient non chiffrés et non conçus : le temps
évolutif (voir `04_SIMULATION.md`) et le pari d'émergence (voir plus bas).

---

## Les 7 jalons

Repris de `direction/genesis-direction.html` section 07. 0.0.1 vers 0.1.0 visé en environ
six mois. Chaque jalon a un moment public : une observation d'émergence, jamais un événement
scripté (T-7, T-11).

| Version | Nom | Ce que ça ajoute | Moment public | Question ouverte |
|---|---|---|---|---|
| 0.0.1 | Deux | 2 entités, énergie, mouvement, reproduction asexuée, mutation, mort, persistance, graine, ViewState, rendu web | Le premier rejeu déterministe, à la frame près | Le stade molécule tient-il assez de générations pour qu'une sélection se voie ? |
| 0.0.2 | Vie | 100 entités et plus, génome complet, sélection naturelle, statistiques, traçabilité causale, bascule molécule vers cellule. **Fait :** capacité de charge par la matière (`experiments/005`) ; bascule cellule étape 1 (`experiments/006`) ; sélection mesurée, série temporelle `series.jsonl` + graphe `series.html` ; traçabilité causale, liens de base câblés (`Event.causes` peuplé pour `PopulationCrash` et `LineageExtinct`, `seq` attribué à la création). Graphe causal complet à 0.0.6 | Un graphe d'évolution génétique sur 10 000 générations, en direct | Le graphe existe (`series.html`) ; 10 000 générations demandent le modèle de temps à deux horloges (`04_SIMULATION.md`), non construit : un run de 60 000 ticks donne ~30 générations |
| 0.0.3 | Individus | Mémoire, personnalité, besoins, objectifs, relations, comportement. **Cible probante.** Toujours sans LLM. **Démarré :** t1 le premier souvenir (mémoire qui biaise le déplacement) ; t2 `lives.html`, la biographie auto-générée (gabarits, aucun LLM) ; t3 le souvenir ancré (`Memory.event_seq` pointe l'`EntityDied`) ; t4 les besoins (faim, peur, solitude pondèrent le comportement) ; t5 la personnalité héritée (`caution`, `curiosity` au génome, 9 traits, schéma v10). t6 le mode de comportement lisible (`Mind.mode` : lecture de la force qui a dominé chaque décision, zéro coût, la biographie dit « a fui plutôt que de manger ») ; t7 les souvenirs sociaux (`Mind.social` : un agent reconnaît les autres agents qu'il croise souvent, familiarité et valence ; premier pas vers les groupes) ; t8 la santé (`Entity.health` : la biologie devient un scalaire de fond qui intègre famines et vieillesse, un corps usé se traîne et meurt plus tôt ; la cognition passe au premier plan). **État consolidé (graine 1, v11) : la cognition divise les morts par famine par six (25 800 -> 4 300), la population meurt de vieillesse et non plus de faim, la perception se détend de 0,95 à 0,75.** Dé-simulation partielle faite (santé) ; l'escalier étape 2 (bilan molécule d'une cellule) déféré à 0.0.6. Cible probante atteinte | La première biographie auto-générée (`lives.html` : souvenirs tracés, jauges, tempérament, mode de décision, relations) | Le comportement dépend-il vraiment du souvenir, ou est-ce cosmétique ? Substrat cognitif semé ou évolutif ? (voir `05_COGNITION.md`) |
| 0.0.4 | Voix | Signaux, langage émergent, groupes, transmission | Deux populations isolées qui divergent linguistiquement | Un système de signes émerge-t-il sans qu'on code un lexique ? |
| 0.0.5 | Société | Culture, mémoire collective, consensus, premier LLM (cloud), Voile en lecture | Le premier mythe daté, tracé jusqu'à son événement d'origine | Le pari d'émergence : un mythe se détache-t-il du fait par une règle, pas par un `if` ? |
| 0.0.6 | Civilisation | Villages, économie, institutions, politique, religion, toutes émergentes | La première guerre et ses quatre récits contradictoires | Les institutions naissent-elles d'un groupe, ou sont-elles déclarées ? |
| 0.1.0 | Le monde qui parle | Couche numérique Nodyx : wiki, forum, calendrier des civilisations, émissaire | On ouvre l'URL. La bibliothèque de mondes est publique | (lancement) |

À partir de 0.0.2, tout monde qui se termine reçoit son autopsie (T-15). Absent volontairement
jusqu'à 0.1.0 : le client Godot, le LLM local, l'économie financière complexe.

---

## L'escalier des échelles

La simulation ne garde pas la même unité tout du long. Elle monte un escalier : à chaque
marche, l'unité de simulation change, et la couche du dessous cesse d'être simulée en détail
pour devenir un substrat agrégé ou figé. C'est la simulation différentielle (invariant 6)
appliquée à l'échelle, pas seulement à l'importance.

L'escalier suit celui du vivant réel (référence : `Transcription/Taille.md`, ordres de
grandeur entre parenthèses ci-dessous). Toutes les marches biologiques ne deviennent pas une
unité de simulation distincte : tissu, organe et appareil sont la structure interne d'un
organisme, pas des niveaux qu'on pilote séparément. Elles comptent comme de la
différenciation à l'intérieur de la marche « organisme ».

| Marche | Unité simulée | Déclencheur de bascule (mécanisé, jamais marqué à la main) | Ce qu'on agrège de la couche du dessous | Jalon |
|---|---|---|---|---|
| Atome (~0,1 nm) | (pas une unité du moteur) | (substrat) | métadonnées de table périodique, CHNOPS | couche chimie, `experiments/002`, 0.1+ |
| Molécule (~1 nm) | l'entité, scission asexuée | (départ) | l'atome reste dans la couche chimie | 0.0.1 |
| Cellule (1 à 50 µm) | un amas cohésif borné (membrane, énergie mutualisée, reproduction protégée) | un groupe d'entités génétiquement proches, cohésion haute, spatialement serré et persistant, franchit un seuil (`experiments/006`, sur le modèle de `SpeciesEmerged`) | étape 1 : rien (les membres restent simulés, taggués `cell_id`) ; étape 2 : les molécules internes deviennent un bilan, plus simulées une à une | 0.0.2 |
| Organisme (tissu ~1 mm, organe ~1 cm, appareil ~1 dm, corps ~1 m) | un individu multicellulaire, cycle de vie complet ; tissus, organes et appareils sont sa structure interne | un agrégat de cellules différenciées et co-dépendantes tient sur plusieurs générations | les cellules internes deviennent des tissus puis des organes (fonctions, santé), les organes des appareils | 0.0.2 vers 0.0.3 |
| Individu, agent | l'agent : mémoire, besoins, personnalité, objectifs, relations | l'organisme atteint le seuil cognitif (perception, mémoire, apprentissage suffisants) ; promotion réversible | la biologie devient un état de fond (santé, âge, génome), la cognition passe au premier plan | 0.0.3 |
| Groupe | famille, bande, communauté : mémoire et objectifs collectifs | des agents en contact répété, coopération ou parenté, franchissent un seuil de cohésion sociale | les membres non saillants sont simulés statistiquement | 0.0.4 vers 0.0.5 |
| Civilisation | territoire, institutions, économie, culture | un réseau de groupes avec une mémoire collective ancrée et des institutions stables | les groupes deviennent des populations, des régions, des factions | 0.0.6 |

Règle commune à toutes les bascules : elles sont détectées, pas devinées (T-7). On définit
des seuils mesurables, le monde les franchit, le moteur émet un événement saillant, et le
niveau de détail se réorganise. Une bascule est réversible : une cellule qui perd sa
cohésion redevient des molécules libres, un groupe qui se disperse redevient des individus.

Dézoomer, c'est monter une marche. La couche quittée n'est pas jetée : elle est résumée,
comme la pyramide temporelle résume les vieilles années (`02_ARCHITECTURE.md`, "Un monde qui
ne s'arrête jamais").

---

## Le pont Entity vers Agent

C'est la marche qui mène à la cible probante. Détail dans `05_COGNITION.md`. En résumé :

- **Entity (0.0.1)** : un organisme sans cognition. `{ id, genome, position, energy, age }`
  plus 7 traits. Se déplace par chimiotaxie, mange, se scinde, meurt.
- **Agent (0.0.3)** : une entité promue. Ajoute un moteur de mémoire, des besoins, une
  personnalité (paramètres hérités), un modèle de comportement qui lit la mémoire. Pas de
  LLM avant 0.0.5.
- **La promotion** est un seuil mécanisé sur les capacités (perception, mémoire,
  apprentissage), pas un `if age > X`. Réversible : un agent dont les capacités retombent
  redevient une entité de fond.

Question laissée ouverte, à trancher à 0.0.3 avec des données : le substrat cognitif est-il
**semé** (moteur de mémoire et modèle de décision construits en dur, seuls leurs paramètres
et la culture au-dessus émergent) ou **cultivé** (la capacité de mémoire, la règle
d'apprentissage évoluent aussi) ? Les deux options et leur coût sont posés dans
`05_COGNITION.md`.

---

## La règle de subordination

Chaque version 0.0.x doit faire avancer le substrat vers la cognition, pas seulement ajouter
du réalisme biologique.

Test à appliquer à tout item de travail : est-ce que ça rapproche d'un agent qui se
souvient, ou est-ce du polish sur la molécule ? Un item de polish n'est pas interdit, mais
il ne passe pas devant un item qui construit le substrat.

Le travail d'instrumentation (densité de statistiques, rigueur A/B, veilleurs) reste
légitime : il sert à détecter l'émergence et à régler les pressions sans régler les
résultats (T-16). Il n'est pas le livrable. Voir la décision "L'instrument sert l'objectif"
dans `00_INDEX.md` section C.

---

## Le pari d'émergence

À partir de 0.0.5, la culture, les mythes, les institutions doivent naître de règles qui
consomment des événements, de la mémoire et du contact, jamais d'un `if` qui nomme le
résultat (T-7). Si ce pari ne se mécanise pas, ces pans arrivent plus tard, on ne les
déclare pas.

Testé une fois en prototype Python (`experiments/001_emergence.md`), verdict : mécanisme
plausible, pari ni levé ni cassé. L'audit indépendant dit que le projet est solide jusqu'à
0.0.3 et que 0.0.5 et 0.0.6 seront déclaratifs sans un modèle de données d'émergence concret.

**Expérience 004, planifiée avant 0.0.5** : le vrai test du pari sur le moteur (pas un jouet
numpy), avec de vrais agents, une vraie mémoire ancrée qui se dégrade, une transmission avec
perte, un renouvellement des générations. Mesure si un mythe engagé et un groupe qui le
défend émergent. Reprend l'intention déjà inscrite dans `GENESIS_FIDELITY.md`.

---

## Ce qui reste à écrire

- `04_SIMULATION.md` : écrit (le modèle de temps à deux horloges).
- `05_COGNITION.md` : squelette écrit (le pont Entity vers Agent, la question semé ou cultivé).
- `06_EMERGENCE.md` : à écrire avant 0.0.5, avec l'expérience 004.
- `07_HISTORY_JUDGMENT.md`, `08_WORLDS.md`, `09_NODYX_VOILE.md` : à écrire au fil des jalons.
