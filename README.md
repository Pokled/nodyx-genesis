# Nodyx Genesis

Nodyx Genesis est un moteur de simulation qui fait pousser un monde vivant a partir d'une
graine. On pose des regles simples, de la matiere, du temps, et on regarde ce qui arrive :
des molecules qui se divisent, des amas de parents qui se referment en cellules, deux
membranes qui n'en font plus qu'une, des individus qui gardent des souvenirs et changent de
comportement selon ce qu'ils ont vecu. Le monde continue tout seul, sans joueur, et rien de
ce qui s'y passe n'est ecrit d'avance. Aucun modele de langage n'intervient dans la
simulation.

Le projet fait partie de l'ecosysteme Nodyx. Ce qu'il cherche vraiment, et pourquoi, est
decrit dans `BIBLE/01_VISION.md`. Le registre complet des decisions, avec leur statut, est
dans `BIBLE/00_INDEX.md`.

![L'overlay du direct 24/24](docs/images/stream.png)

## Ce que fait le moteur aujourd'hui

Le jalon 0.0.3, "Individus", est atteint. Sa cible etait modeste et verifiable : un individu
qui se souvient, dont on peut lire la biographie. Une entite s'eveille en agent quand elle
percoit assez bien et a vecu assez longtemps ; elle gagne alors une memoire faite de lieux,
chacun marque par un peril, une aubaine ou la mort d'un proche, avec une force qui s'efface
avec le temps. Trois jauges internes, la faim, la peur et la solitude, pesent sur ses choix.
A chaque decision elle retient explicitement un mode : chercher a manger, fuir un lieu de
danger, suivre les siens, retourner a une aubaine, ou errer. Elle reconnait les autres
agents qu'elle croise souvent. Sous tout cela, un corps, avec une sante qui integre lentement
les famines repetees et la vieillesse.

En dessous des individus, il y a les cellules. Un groupe d'entites proches, genetiquement
parentes, cohesives et persistant devient une unite reconnue, une membrane qui partage
l'energie et protege la reproduction de ses membres. Depuis peu, deux cellules stables dont
les membranes se chevauchent et dont les genomes se ressemblent fusionnent : la plus grosse
garde son identite, la petite y disparait, le genome de l'ensemble est remanie. Personne ne
declenche une fusion. C'est une condition geometrique et genetique que le monde franchit
quand deux colonies parentes derivent l'une dans l'autre.

Le monde a aussi un climat. Sa temperature moyenne, sa gravite, sa pression sont fixees a la
creation. La temperature agit sur le cout du metabolisme : un monde loin de son optimum
thermique est plus dur a habiter, on y meurt plus de faim, la selection y est plus rude. La
gravite renchit le deplacement. A graine egale, un monde nettement plus froid porte deux fois
moins de monde ; meme graine, autre planete, autre vie.

Ce qui change au fil du temps, ce sont les saisons. La capacite nourriciere du sol oscille
lentement au fil de l'annee-monde : une saison d'abondance, la population deborde ; une saison
maigre, une famine synchrone la rabote de moitie. En decale d'un quart d'annee, la temperature
oscille elle aussi. Le monde de reference n'est plus une ligne plate collee a son plafond, il
respire entre cinq mille et neuf mille six cents, et chaque goulot de disette rebrasse le
centre genetique de la population. Un dixieme trait, la tolerance a la chaleur, laisse la
population s'adapter au climat. A graine egale, ajouter les saisons fait passer la population
d'une oscillation de quelques dizaines a plusieurs milliers, et leve la diversite genetique
des deux tiers.

Le jalon 0.0.4, "Voix", a commence. Un agent qui frole la mort par famine emet une alarme a
sa position ; les agents proches l'entendent et ont un sursaut de peur, sans qu'aucun souvenir
ne se forme. Un agent qui mange bien sur une case franchement riche lance a l'inverse un appel
"bon coin ici", et les agents proches qui decident ou aller inflechissent leur cible vers lui.
Deux genres de signal, fixes, aucun lexique code. A graine egale, l'alarme divise les morts
par famine d'environ un dixieme, l'appel augmente les naissances et la diversite genetique :
la panique disperse, l'appel rassemble.

Tout changement de regle passe par une comparaison a graine egale, avec son effet documente.
On ne regle jamais une regle parce qu'elle a produit un monde plus plaisant.

## Le pari du determinisme

Meme graine, meme configuration, meme version du moteur : le monde est identique a l'octet
pres, tick apres tick. C'est ce qui rend les experiences reproductibles, et c'est aussi ce
qui permet au monde de surprendre ses createurs, puisqu'on ne peut pas le corriger en douce.

```
genesis replay worlds/w2
```

La commande rejoue le monde depuis sa graine et affiche `deterministe : OK`, ou `DIFF` avec
la position exacte de la divergence. Le determinisme est verifie a un thread et a huit
threads, journaux et instantanes byte-identiques.

## Un monde en direct

Un monde n'a pas besoin de s'arreter. `genesis serve` reprend un monde depuis son dernier
instantane et le fait avancer sans fin, par petits pas paces, en refaisant ses pages a
mesure. Avec `--port`, il sert le monde en HTTP, et un tableau de bord de direct devient
accessible dans un navigateur ou une source OBS.

```
genesis serve worlds/w2 --port 8080 --rate 45
```

Ce tableau de bord, `stream.html`, est une vue publique des donnees que Genesis produit deja.
Une horloge de l'age du monde, le pouls du monde en cinq jauges, le genome dominant en double
helice avec la derive de chaque trait depuis la genese, le fil des evenements, la courbe de
population de toute la vie du monde avec les grands tournants poses dessus, les records du
monde. Quand deux membranes fusionnent, la scene s'assombrit une seconde, un projecteur sur
le point. Rien n'est invente : chaque chiffre affiche raconte quelque chose du monde.

Un monde qui tourne pendant des mois finit par manquer de memoire ou de disque. Les
biographies terminees sont oubliees au fil de l'eau, la serie temporelle est plafonnee, on
ne garde que les instantanes recents, et le journal ne recoit plus le bruit de plateau.
Avec `--restart`, quand le monde meurt, une nouvelle graine repart au meme endroit et les
records se transmettent de monde en monde.

## Regarder un monde

Chaque monde genere s'ouvre par sa page de garde, `index.html`. Un chapeau ecrit a partir de
ses chiffres, sans modele de langage, raconte l'essentiel : les annees vecues, les
naissances, la plus longue vie d'agent et sa lignee, les especes qui ont emerge, les lignees
qui se sont eteintes, de quoi on y meurt. Une courbe montre la population sur toute la vie du
monde. Trois portes menent aux lecteurs.

![La page de garde d'un monde](docs/images/world-cover.png)

**La scene**, `view.html`, montre le monde en mouvement, image par image. On peut lire,
rejouer, changer la vitesse, se mettre en plein cadre. Un clic sur un point ouvre la carte de
l'individu : sa lignee, son age, sa sante, et s'il se souvient, son eveil, ce qu'il fait, ses
jauges, ses souvenirs les plus forts, ses relations. Le bouton suivre cadre la camera sur lui
et le garde au centre pendant qu'il vit ; la molette zoome ou l'on veut.

![Suivre un individu dans la scene](docs/images/view-inspect.png)

**L'evolution**, `series.html`, trace la derive des dix traits du genome sur toute la duree
du monde. Pas seulement la moyenne : la distribution complete, du dixieme au quatre-vingt-
dixieme centile, parce que c'est une bande qui se scinde qui signale une speciation. La
selection naturelle est le seul moteur ici, personne ne la declare.

![L'evolution genetique du monde](docs/images/series.png)

**Les vies**, `lives.html`, sont des biographies engendrees a partir des donnees, sans aucun
modele de langage. Naissance, souvenirs traces jusqu'a leur fait d'origine, temperament,
mode de decision, relations, mort. C'est la qu'on lit, individu par individu, si le
comportement depend vraiment du souvenir.

![Une biographie d'agent](docs/images/lives.png)

## Les mondes de demonstration

Le depot definit quatre mondes de reference par leur graine. Ils ne sont pas versionnes,
trop gros, mais toujours regenerables a l'identique : `genesis run --seed <N> --ticks 60000
--out worlds/<nom>`. La grille par defaut est 192x192 ; le milieu suit des saisons, donc la
population ne se colle pas a un plafond : elle oscille entre cinq mille et neuf mille six cents
au fil de l'annee-monde. La commande `genesis gallery` reconstruit `worlds/index.html`, la
grille qui les rassemble.

![La bibliotheque des mondes](docs/images/gallery.png)

Le monde **w2**, graine 1, est le monde de reference du projet et celui tenu en direct. A
soixante mille ticks il a vecu pres de sept annees-monde et atteint vingt-neuf generations ;
une des deux lignees fondatrices s'eteint des la premiere annee, puis la population monte a
la capacite en trois ans. Ensuite chaque annee et demie une disette la rabote (jusque sous
six mille) et la saison grasse la ramene ; le genome dominant bascule une fois. On n'y meurt
presque plus de vieillesse, une disette vous prend avant. Depuis `cell_burn_relief` (0.0.2),
la vie multicellulaire **tient** : une centaine de cellules vivantes, pres de la moitie de la
population dans une membrane, et les lignees de cellules encaissent les disettes au lieu de
s'eteindre.

Le monde **w1**, graine 8, suit le meme rythme mais ses disettes sont les plus violentes (la
population descend sous deux mille cent) : c'est le monde a la plus haute diversite genetique
des quatre, une perception tres poussee.

Le monde **w4**, graine 13, est le plus foisonnant : trente generations, treize emergences
d'espece, une efficacite metabolique qui reste basse. Une lignee fondatrice s'y eteint.

Le monde **w3**, graine 4, ne prend pas. Les deux fondateurs se divisent une poignee de fois,
puis les deux lignees s'eteignent avant la premiere annee-monde. Toutes les graines ne
donnent pas un monde viable, et c'est voulu. Sa page de garde le dit sans detour.

## Installer et lancer

Le moteur est ecrit en Rust. Le fichier `rust-toolchain.toml` fixe la chaine a stable, et
`rustup` la recupere automatiquement.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sous Windows, l'installeur est sur `https://rustup.rs`.

Pour installer le binaire une fois, puis faire naitre un monde :

```
cargo install --path crates/genesis-cli
genesis run --seed 1 --ticks 60000 --out worlds/w2
```

Sans installation, `cargo run --release -p genesis-cli -- run --seed 1 --ticks 60000 --out
worlds/w2` fait la meme chose. Les options utiles sont `--out` pour le dossier de sortie,
`--config` pour pointer une configuration `.toml` (par defaut, les chiffres de
`BIBLE/genesis.starter.toml`), et `--frame-every` pour l'intervalle entre deux images
visuelles. Soixante mille ticks font environ sept annees-monde et une vingtaine de
generations.

Le dossier de sortie contient la configuration utilisee, les metadonnees du monde, les
instantanes du World State, le journal des evenements et sa version reduite a la chronique,
les frames du ViewState, la serie temporelle des statistiques, les pages HTML (`index.html`,
`view.html`, `series.html`, `lives.html`, `stream.html`), et le petit etat vivant relu par
l'overlay du direct.

## Tester

```
cargo test
```

Les tests d'invariants sont dans `crates/genesis-core/tests/invariants.rs` : meme graine
meme monde a la frame pres, instantane plus rejeu egal etat vivant, ordre des evenements,
memoire et besoins bornes sur quarante mille ticks, les alarmes et les appels emis et bornes,
les cellules coherentes y compris aux ticks de fusion, le climat et les saisons qui faconnent
le monde sans le casser, la personnalite et la sante dans leurs limites.

## Le corpus

`BIBLE/` rassemble la reference du projet. Le point d'entree est `00_INDEX.md`, qui liste
toutes les decisions avec leur statut. `01_VISION.md` dit ce que le projet cherche.
`02_ARCHITECTURE.md` pose les dix invariants et le contrat du ViewState. `03_DATA_MODEL.md`
decrit le modele de donnees. `04_SIMULATION.md` traite du temps. `05_COGNITION.md` suit le
pont de l'entite vers l'agent, tranche par tranche. `10_ROADMAP.md` tient les jalons et
l'escalier des echelles, de la molecule a la civilisation. `GENESIS_FIDELITY.md` verifie
l'ecart entre le projet voulu et le projet decide.

## Structure du depot

```
Cargo.toml               le workspace
rust-toolchain.toml
crates/
  genesis-core/          World State, tick, evenements, cognition, cellules, climat ; ne connait aucun rendu
  genesis-view/          le contrat ViewState : projection pure du monde en flux observable
  genesis-cli/           le binaire genesis : run, replay, continue, serve, gallery ; generation des pages et de l'overlay
BIBLE/                   le corpus de reference
experiments/             les prototypes numeriques
worlds/                  les mondes generes, regenerables par leur graine
docs/                    les captures de cette page
```

La frontiere est verrouillee : `genesis-core` ne connait pas le rendu, `genesis-view` depend
de `genesis-core` et jamais l'inverse.

## Licence

MIT. Voir `LICENSE`.
