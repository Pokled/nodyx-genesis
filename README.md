# Nodyx Genesis

Un moteur de simulation d'univers vivant. On pose des regles simples, de la matiere, du
temps. La vie apparait, evolue, et le monde continue tout seul.

Ce que le projet cherche, et pourquoi : `BIBLE/01_VISION.md`.
Toutes les decisions, avec leur statut : `BIBLE/00_INDEX.md`.

## Etat

Genesis 0.0.1, "Deux", stade molecule. Deux entites sur une grille a fertilite variable :
elles bougent, mangent, se scindent (reproduction asexuee), mutent, meurent. Une mutation
peut etre letale. La division echoue sans "infrastructure" et sur une case surpeuplee.
Persistance et graine deterministe. Pas de LLM, pas de Nodyx, pas de memoire.

Chaque graine donne un monde different : certains restent une poignee de molecules,
d'autres montent a quelques centaines. Longueur de run conseillee : 20 000 a 40 000 ticks.

## Prerequis

Rust stable. Si tu ne l'as pas :

```
# Windows, Linux, macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# ou sur Windows : https://rustup.rs
```

Le fichier `rust-toolchain.toml` fixe la chaine a "stable", `rustup` la recupere tout seul.

## Lancer un monde

```
cargo run --release -p genesis-cli -- run --seed 1 --ticks 20000

# options
#   --out worlds/mon-monde     dossier de sortie (defaut worlds/w<seed>)
#   --config chemin.toml       config (defaut : les chiffres de BIBLE/genesis.starter.toml)
#   --frame-every 12           un instantane visuel tous les N ticks
```

Sortie dans `worlds/w1/` :

```
config.toml        la config utilisee
meta.json          graine, version du moteur, ticks joues
snapshots/         instantanes du World State
events.jsonl       le journal complet, un evenement par ligne
notable.jsonl      seulement les evenements saillants (les chapitres)
frames.jsonl       les frames du ViewState
series.jsonl       la serie temporelle de stats (une ligne tous les 500 ticks)
view.html          le lecteur du monde, a ouvrir dans un navigateur
series.html        le graphe d'evolution genetique du monde
```

Ouvre `worlds/w1/view.html` : la grille, les entites, la timeline, le compteur de
population. Tu peux rejouer, scruter, changer la vitesse. `series.html` montre la derive des
neuf traits du genome sur toute la duree du monde, trait par trait, distributions comprises.
`lives.html` (0.0.3) raconte la vie de quelques agents : leur memoire, leurs besoins, ce
qu'ils ont evite ou cherche.

## Verifier le determinisme

Le moment public de 0.0.1 : meme graine, meme monde, jusqu'au dernier tick.

```
cargo run -p genesis-cli -- replay worlds/w1
```

Affiche `deterministe : OK` ou `DIFF` avec la position de la divergence.

## Tests

```
cargo test
```

Les tests d'invariants sont dans `crates/genesis-core/tests/invariants.rs` : meme graine
meme monde, instantane plus rejeu egal etat vivant, ordre des evenements, etat borne sur
40 000 ticks.

## Structure du depot

```
Cargo.toml               workspace
rust-toolchain.toml
crates/
  genesis-core/          World State, tick, evenements, persistance. Ne depend d'aucun rendu.
  genesis-view/          contrat ViewState : projection du monde en flux observable.
  genesis-cli/           binaire `genesis` : run, replay, generation de view.html.
BIBLE/                   le corpus de reference (voir 00_INDEX.md)
experiments/             prototypes numeriques (voir BIBLE/experiments/)
audit/ direction/ presentation/   documents de contexte
```

Frontiere verrouillee (tranchee 3) : `genesis-core` ne connait pas le rendu.
`genesis-view` depend de `genesis-core`, jamais l'inverse.
