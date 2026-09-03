# Analyse du dossier Transcription

Date : 2026-09-01. Auteur : passe de lecture pour Genesis.

## Ce que contient le dossier

Deux transcriptions de vidéos de vulgarisation, en français, sur la vie artificielle.

| Fichier | Sujet | Origine probable |
|---|---|---|
| `jeu_de_vie_caunway.md` | Le Jeu de la vie de Conway, de la soupe primordiale jusqu'à un ordinateur Turing complet, puis ouverture vers les variantes et Lenia | Chaîne de vulgarisation, format « voyage » à dos de planeur, sponsor Odoo |
| `Lenia.MD` | Lenia, généralisation continue du Jeu de la vie, et exploration de son espace de paramètres par apprentissage automatique | ScienceEtonnante, échanges avec l'équipe Flowers de l'INRIA |

Les deux se lisent comme un diptyque : le premier pose l'automate cellulaire discret et
montre jusqu'où il monte, le second lève une à une ses contraintes pour se rapprocher du
vivant.

---

## Résumé, Jeu de la vie de Conway

Automate cellulaire, grille 2D théoriquement infinie, cellules à deux états, huit voisines.
Deux règles (naissance à 3 voisines, survie à 2 ou 3). Jeu à zéro joueur : il tourne seul,
l'humain explore et expérimente avec les conditions initiales.

Progression de la vidéo, du simple au complexe :

- Natures mortes (bloc, ruche), oscillateurs (blinker, pulsar), vaisseaux (planeur à c/4).
- Vaisseaux naturels : 54 recensés, triés par fréquence, le planeur représente 99,8 % des
  apparitions spontanées. Le reste tombe très vite dans l'improbable.
- Corderships, propulsés par des « switch engines » chaotiques qu'on cumule pour annuler
  leurs débris. Découverte historique : réduire le nombre de moteurs, pas l'augmenter.
- Géniteurs (glider guns) : croissance infinie, prime mise par Conway, trouvés par des
  étudiants du MIT. Usines qui assemblent des vaisseaux par collisions programmées de flux.
- Puffers, rakes, breeders. Le Breeder 1 : première croissance quadratique connue, un puffer
  dont les débris sont des usines à gliders.
- Mathusalems : petites configurations qui mettent des dizaines de milliers de générations
  à se stabiliser (52 513 pour celui montré).
- Turing complétude : portes logiques ET, OU, NON construites avec des flux de gliders,
  additionneurs, une unité arithmétique et logique 8 bits, une mémoire à bascules RS, un
  ordinateur programmable en Python avec un écran. Conway l'avait pressenti dès la découverte
  du planeur, « potentiel de transmission de données ».
- Ouverture : HighLife et ses réplicateurs, Wireworld, la fourmi de Langton, les turmites,
  puis Lenia. Clôture philosophique sur la soupe primordiale et l'apparition de la vie sur
  Terre il y a 3 à 4 milliards d'années.

Anecdote utile : Conway avait engagé son ami Richard Guy comme « blinker watcher », chargé
à la main du suivi des petites configurations périodiques.

---

## Résumé, Lenia

Lenia généralise le Jeu de la vie en rendant continu ce qui était discret, sur trois axes :

1. **État des cellules** : une valeur réelle dans [0, 1] au lieu de 0 ou 1. Lecture possible
   comme un niveau de vie, ou comme la concentration d'une substance.
2. **Temps** : la courbe de règle devient un taux de croissance, intégré sur un pas de temps
   aussi fin qu'on veut, au lieu d'un passage de génération à génération.
3. **Espace** : le comptage des huit voisines devient une convolution par un noyau (filtre),
   typiquement un anneau, ou plusieurs anneaux d'intensités différentes. La notion de voisine
   immédiate disparaît au profit d'une notion de proximité pondérée.

Règles = choisir un noyau, choisir une fonction de croissance (souvent gaussienne), simuler.
Décrit par Bert Chan en 2019. Zoologie : Orbium (déplacement linéaire, symétrie bilatérale),
Hydrogenium (noyau multi-anneaux, déplacement qui louvoie), créatures multi-filtres.

Améliorations décisives :

- **Multi-canaux** : plusieurs valeurs par case, une par « espèce chimique », chaque canal
  sent les autres via un filtre dédié et y réagit par sa fonction de croissance. Fait émerger
  des comportements différenciés, dont une quasi division cellulaire (Aquarium).
- **Exploration par apprentissage automatique** : l'espace des paramètres (noyaux, fonctions
  de croissance, nombre de canaux, configurations initiales) est gigantesque. L'équipe
  Flowers de l'INRIA adapte des méthodes d'apprentissage inspirées de la curiosité et de la
  motivation intrinsèque pour le fouiller efficacement.

Résultats obtenus par cette exploration dirigée : des motifs qui contournent des obstacles
et parcourent un labyrinthe, qui se reconstituent après une attaque (bruit, « balles »),
qui font de la chimiotaxie en suivant un signal sur un canal, qui se reproduisent (deux
individus en engendrent un troisième), et qu'on met en compétition dans un écosystème
virtuel où apparaissent des premiers éléments de sélection naturelle.

Le filtre de perception est interprété explicitement comme une faculté sensorielle : chaque
point sent son voisinage à une certaine distance et évolue en fonction.

---

## Le fil commun

- **Vie artificielle.** Les deux vidéos se réclament du même domaine de recherche, l'ALife,
  dont l'objectif est de créer des systèmes artificiels aux comportements proches du vivant.
- **Émergence, jamais scénarisée.** Le refrain des deux : à aucun moment le code ne dit de
  déplacer une structure. Le mouvement du planeur, celui de l'Orbium, émergent de règles
  purement locales. Le macroscopique n'est pas écrit.
- **Zéro joueur, mais pas zéro humain.** Le système tourne seul. L'humain explore, teste des
  conditions initiales, nomme, met en commun. Le Jeu de la vie n'a décollé que grâce à une
  communauté qui a construit une zoologie et un wiki.
- **Soupe primordiale.** Cadre partagé, presque mot pour mot : un milieu simple d'où la vie
  aurait émergé, qu'on peut recréer et observer.
- **Déterminisme.** Les deux systèmes sont déterministes. C'est ce qui rend les motifs
  reproductibles et partageables entre passionnés.
- **Explosion de l'espace des paramètres.** Problème central de Lenia dans sa deuxième
  moitié : trop de règles possibles, la recherche à la main est fastidieuse. Réponse :
  l'exploration dirigée par la curiosité.
- **Fragilité contre robustesse.** Les premiers motifs de Lenia, comme ceux du Jeu de la
  vie, cassent à la moindre perturbation. Les motifs récents résistent et se réparent. La
  robustesse est devenue un objet d'étude et une propriété qu'on sait sélectionner.

---

## Similitudes avec Genesis

Genesis appartient à la même famille intellectuelle. Les correspondances sont nombreuses et
plusieurs valident des choix déjà pris.

1. **Même lignée.** Conway, HighLife, Lenia et Genesis sont tous de la vie artificielle.
   Genesis se distingue en étant centré agents et non centré cellules (l'état vit dans les
   entités, avec un génome et une hérédité), mais son milieu, `ResourceField`, est bien une
   grille de type automate cellulaire avec régénération et tension.

2. **Doctrine de l'émergence.** « L'histoire n'est jamais écrite à l'avance » (règle sacrée
   2), le sexué ne doit jamais être un `if` mais émerger (tranchée 7, `00_INDEX.md`),
   l'émergence sociale doit naître d'une règle qui consomme des événements (`001_emergence`).
   C'est exactement le principe que les deux vidéos martèlent.

3. **La chimiotaxie, déjà là.** Le mouvement des entités de Genesis émerge de
   `sim.rs::forage_target` : remontée d'un gradient de concentration de ressources. C'est
   très précisément la lecture que la vidéo Lenia fait du filtre de perception, et la
   chimiotaxie bactérienne qu'elle cite comme un jalon récent. Genesis l'a de base.

4. **Stade molécule.** La vidéo Conway se termine où Genesis 0.0.1 commence : de petites
   créatures dans les océans il y a 3 à 4 milliards d'années. Genesis 0.0.1 est explicitement
   le stade molécule, reproduction asexuée par scission. Le cadrage est partagé.

5. **Déterminisme et rejeu.** Tranchée 5, verrouillée, rejeu byte-identique. Les vidéos
   montrent à quoi ça sert : reproduire une configuration, la partager par sa graine.

6. **Détecteur d'intérêt.** Les veilleurs mécanisés de la phase 8b (`SpeciesEmerged`,
   `PopulationCrash`, `LineageExtinct`) avec leur `salience: u8` sont déjà un détecteur de
   nouveauté. C'est l'ébauche maison de ce que l'équipe Flowers formalise pour Lenia. Rime
   historique : Genesis automatise le rôle que Conway confiait à la main à Richard Guy.

7. **Le problème du réglage.** Le « whack-a-mole » signalé plusieurs fois pendant le
   développement, la forte variance entre graines, les quarante paramètres de
   `genesis.starter.toml` : c'est le même mur que Lenia. La vidéo donne le nom de la sortie
   possible.

8. **Multi-canaux et couche chimie.** L'amélioration la plus riche de Lenia, un canal par
   espèce chimique avec des filtres croisés, est presque exactement la spec différée
   `experiments/002_pseudo_chemistry.md` (automate pseudo-chimique CHNOPS). Lenia en donne
   une formulation mathématique éprouvée.

9. **Robustesse comme mesure.** Le backlog de Genesis (prédation, maladie, météo,
   catastrophes) est une liste de sources de perturbation. Lenia montre qu'il faut en faire
   une grandeur observable, pas seulement un danger.

---

## Différences, pour ne pas trop emprunter

- **Genesis a une hérédité, pas Lenia.** Les motifs de Lenia n'ont pas de génome. Leur
  « évolution » est une recherche de paramètres faite par l'humain ou l'apprentissage, pas
  une descendance avec modification. Genesis peut faire de l'évolution ouverte que Lenia ne
  peut pas. Sur ce point Genesis est plus proche de Tierra, Avida, Polyworld, Framsticks.
- **La continuité de Lenia est un choix esthétique** pour des morphologies organiques.
  Genesis est hybride : grille discrète pour le milieu, position continue pour les entités.
  Inutile de courir après la continuité totale. Un point cheap et cohérent serait une
  diffusion sur le `ResourceField`.
- **La Turing complétude du Jeu de la vie est une curiosité,** pas un objectif Genesis.
- **Genesis a une couche narration et observateur** (chapitres, Voile, annotations datées)
  qu'aucun des deux systèmes n'a. C'est une part de son identité, pas un emprunt.

---

## Ce qu'on peut en tirer

Classé par rapport valeur sur effort.

### A. Curation automatique de mondes à partir du signal de saillance

Répond directement à la douleur du réglage. Prior art : IMGEP et exploration par curiosité
de l'équipe Flowers.

Version minimale : une sous-commande CLI qui lance N graines sur K ticks et les classe par
richesse de la liste de chapitres (nombre et diversité des événements notables), puis rapporte
les meilleurs mondes. Les veilleurs et `notable.jsonl` existent déjà, le signal est prêt.
Aucune modification du moteur, juste un pilote au dessus. Effort faible.

Version suivante : au lieu de graines au hasard, faire varier quelques paramètres de config
dans des plages, garder ceux qui produisent des mondes riches et surprenants, sans jamais
sélectionner sur « plaisant » (tranchée 16, règle du VISION). Effort moyen.

À consigner dans un `experiments/004_curation.md` sur le gabarit de `001_emergence.md`.

### B. Cadrer la couche pseudo-chimie sur le formalisme multi-canaux de Lenia

`experiments/002_pseudo_chemistry.md` gagnerait un squelette rigoureux : un canal par
composé, une convolution par paire de canaux, une fonction de croissance, intégration sur
dt. Modèle de calcul déterministe et parallélisable (convolution directe ou FFT), cohérent
avec le travail de perf récent. Effort : une passe de rédaction sur la spec, pas de code.

### C. Ajouter une grandeur de résilience

Perturber un monde stable (retirer X pour cent des entités, injecter de la tension, une
catastrophe ponctuelle et déterministe) et mesurer le temps de récupération. Transforme
l'item « catastrophes » du backlog en mesure. S'inscrit dans `WorldStats` et le contrat
ViewState que le VISION veut voir grandir. Effort faible à moyen.

### D. Situer Genesis dans la taxonomie ALife, dans le VISION

Une courte section « parenté » : automates cellulaires (Conway, Lenia) contre modèles
centrés individu (Tierra, Avida, Polyworld, Framsticks) contre Genesis. Honnête sur ce que
Genesis emprunte (doctrine de l'émergence, cadre soupe primordiale, déterminisme) et sur ce
qui le distingue (hérédité, évolution ouverte, couche narration). Utile pour la crédibilité
du positionnement « instrument scientifique ». Effort : une passe de rédaction.

### E. Perception multi-distances

Note de moyen terme pour le modèle de perception. Au lieu d'un seul centroïde de ressources,
sonder ressources, parents et danger à deux ou trois rayons. Lenia montre que ça produit un
mouvement non linéaire, plus vivant (le louvoiement d'Hydrogenium). Cheap en calcul. À
garder pour quand la perception sera retravaillée.

### F. Catalogue partagé d'espèces et de mondes

Le modèle du wiki du Jeu de la vie. Les chapitres et les clés d'espèce sont la graine de ça.
Colle à l'angle hub communautaire de Nodyx. Long terme.

### G. Générateur de noms

Les vidéos montrent combien les noms comptent pour une communauté (french kiss, mini
cocotte-minute, géniteurs). Les espèces de Genesis sont des clés de génome numériques. Un
générateur de noms déterministe, semé par la clé de génome, rendrait les mondes mémorables
et partageables. Petit, forte valeur d'usage.

---

## Références externes utiles

- Wiki du Jeu de la vie (LifeWiki) : zoologie, catégories, historique.
- Bert Chan, articles fondateurs de Lenia (2019) et chaîne associée.
- Équipe Flowers, INRIA Bordeaux : exploration par curiosité et motivation intrinsèque,
  appliquée à Lenia (mots clés : IMGEP, intrinsically motivated goal exploration).
- Famille centrée individu, pour comparaison : Tierra (Ray), Avida, Polyworld (Yaeger),
  Framsticks.
- Piste ouverte plus large : POET et l'auto-génération de curriculums d'environnements.

---

## Seconde passe (2026-09-03) : Sims, xenobots, Cambrien — pour `cellule -> tissu -> organe`

De nouveaux PDF ont été ajoutés au dossier. Beaucoup sont de la paléontologie de radiation
(échinodermes, tétrapodes, oiseaux, cétacés...) : utiles plus tard pour les plans
d'organisation, pas maintenant. Un rapport de politique du CNRS (section 29) : sans intérêt
pour le modèle. Trois sources parlent directement à la marche organisme.

### `siggraph94.pdf` — Karl Sims, *Evolving Virtual Creatures* (1994)

Le génotype est un **graphe orienté** : noeuds = unités du corps, connexions = attache
(position, orientation, échelle, limite de récursion). Le phénotype est *développé* depuis le
graphe : une sous-structure décrite une fois est **instanciée** partout où on la référence
(une patte définie une fois, poussée quatre fois, chacune avec sa copie locale du circuit de
contrôle). La différenciation vient de la topologie du graphe, pas d'un gradient chimique. La
mutation agit sur le graphe (ajouter/retirer un noeud, recâbler une connexion). Sims utilise
une fitness explicite (nager, marcher) ; pour l'ouvert, ses propres refs pointent Tierra,
Polyworld, et les L-systems (Lindenmayer) pour la grammaire de développement.

Ce que ça donne pour Genesis : le génome de 10 scalaires plats ne peut jamais exprimer « quelle
forme, quel bout fait quoi ». Il faut un **second génome, structurel** (graphe/règles),
distinct du génome de traits. Consigné comme « Piste D » dans `experiments/009_organism.md`.

### `url_video_et_documents.md` — la lignée Sims -> Kriegman -> xenobots

Toute la filiation moderne des créatures évoluées : Sam Kriegman (2017), puis les **xenobots**
(2020-2025), robots vivants faits de cellules souches de grenouille, conçus par algo évolutif.
Un tas de cellules s'auto-organise en unité multicellulaire fonctionnelle (déplacement,
navigation, transport collectif, auto-réparation, **réplication cinématique** : elles
rassemblent des cellules libres en tas qui deviennent de nouveaux xenobots) **sans génome de
plan corporel**. Ça valide la piste A du 009 (l'organisme colonial émerge de l'agrégation).
Refs à récupérer : Kauffman *Origins of Order* (auto-organisation + sélection), Prigogine
(structures dissipatives), Schrödinger *What is Life?*, PNAS 2023 (algo évolutif guidé par IA
= bien plus rapide — répond au point A ci-dessus).

### `L-Explosion-cambrienne...pdf` — Vannier (2009), le point le plus important

L'explosion cambrienne n'est **pas** l'apparition des cellules ou des tissus (déjà là) : c'est
l'apparition de la **prédation** et de sa cascade de rétroactions. Rien dans le registre
précambrien ne montre d'interactions animales fortes ni de chaîne trophique complexe. La
chaîne d'innovations : système nerveux + **vision** (« Light Switch » de Parker : voir
déclenche la prédation active) -> nouvelle pression sélective -> armure / biominéralisation ->
comportements anti-prédateur -> colonisation de niches -> niveaux trophiques. Climat et
oxygène : préparent le terrain, **négligeables pour le basculement lui-même**, qui est piloté
par le biotique. Et : l'interdépendance du réseau trophique rend l'écosystème plus stable en
marche normale **mais plus fragile au choc** (un choc à un niveau se propage) — c'est
exactement la grandeur de résilience du point C plus haut.

Conséquence pour Genesis : le multicellulaire sans prédateur n'a aucune raison de persister
(cf. w2, multicellulaire éteint sous saturation). Avec un prédateur, une cellule devient un
**refuge de taille** = avantage sélectif durable. Genesis n'a aucune prédation (morts = faim +
âge, un seul niveau trophique). **La prédation est la marche qui manque avant l'organisme.**
Ordre décidé : `cell_burn_relief` (tampon anti-disette, 0.0.2) -> prédation
(`experiments/012_predation.md` à rédiger) -> organisme (piste A) -> génome structurel
(piste D).

### `Taille.md`

L'escalier canonique : atome -> molécule -> **cellule -> tissu -> organe** -> appareil ->
organisme. Critères qui définissent le saut : tissu = cellules *spécialisées* + une *même
fonction* ; organe = *plusieurs tissus* qui *collaborent*. C'est la spec de ce que le 009
appelle « la spécialisation et la co-dépendance manquantes ».
