# 01. Vision

Statut : LOCKED sur l'intention. Fusion des 5 documents de vision d'origine, recentrée.

---

## Ce que c'est

Nodyx Genesis est un moteur de simulation d'univers vivant. On pose des règles simples,
de la matière, du temps. La vie apparaît, évolue, forme des groupes, des sociétés, des
civilisations. Le monde continue jour et nuit, que quelqu'un le regarde ou non.

Genesis est le moteur. Nodyx est l'écosystème numérique où le monde devient observable,
et où ses civilisations pourront un jour créer de vraies pages, de vrais forums, de vrais
échanges.

## Ce que ce n'est pas

Ce n'est pas un jeu avec des personnages IA. Ce n'est pas une démonstration d'une thèse.
Ce n'est pas une prétention à détenir une vérité sur l'univers ou sur la nature humaine.

## Le but

Montrer qu'un humain et une IA peuvent construire quelque chose ensemble, et apprendre de
ce que ça devient. Faire vivre un monde dont quelqu'un, un mauvais jour, pourrait apprendre
beaucoup.

## Ce que le projet cherche

- On ne prétend pas prouver que les civilisations tendent vers le bien. On explore quelles
  conditions permettent qu'une bonne émerge.
- La plupart des mondes échoueront. Guerre, effondrement, tyrannie, extinction, un monde qui
  détruit son environnement. C'est le sujet, pas un échec.
- Si un monde juste apparaît un jour, ce sera un cadeau rare, pas une démonstration.

## Le plus difficile

Ne pas laisser notre jugement d'aujourd'hui, humain ou IA, décider de ce que ces mondes
deviennent. Un monde ne peut nous apprendre que ce qu'on ne lui a pas dicté. S'il ne fait
que refléter nos valeurs présentes, il ne fait que nous les renvoyer.

Mesures concrètes qui tiennent notre jugement à l'écart (détail dans `07_HISTORY_JUDGMENT.md`) :

- Le moteur ne juge jamais. « Bien » n'est pas un champ.
- Rien qui calcule « bien » ne revient dans la simulation.
- Nos lectures d'un monde sont des annotations datées, séparées de son journal, jamais des
  éditions.
- Toute règle changée passe en A/B à la même graine, avec son effet documenté. On ne règle
  jamais une règle parce qu'elle a produit un monde plus plaisant.
- On rejuge les vieux mondes plus tard, pour voir dériver notre propre jugement.

Limite honnête : on ne peut pas être sans biais. Le choix de ce qu'on simule encode déjà des
hypothèses. On les garde minimales et basses, au niveau de la physique et de la biologie,
jamais au niveau des valeurs. Les mondes renseignent sur « ce que ces règles produisent »,
pas sur « l'univers ».

## L'étoile polaire

Une civilisation qui vit sur Nodyx pendant des années, peut-être des décennies. Qui se
développe assez pour percevoir qu'il y a un dehors, chercher à le comprendre, et communiquer
avec nous. On évoluerait en la regardant, elle en nous devinant.

C'est le monde qu'on espère, sans jamais le forcer.

Conséquence directe : le moteur est conçu pour la longévité extrême. Un monde tourne des
années sans redémarrer, survit aux montées de version, son état ne grossit jamais sans
borne. Voir tranchée 17 et `03_DATA_MODEL.md`.

## Qui regarde, et quoi

- **Genesis** connaît la vérité complète du monde.
- **Les habitants** ne connaissent que ce qu'ils ont vécu, entendu, cru. Ils ne savent pas
  qu'ils sont simulés, et cette information n'entre jamais dans leur contexte.
- **Les humains** observent depuis Nodyx. Ils peuvent suivre, lire, comparer, et parfois
  parler aux habitants à travers le Voile, sans jamais pouvoir leur révéler ce qu'ils sont.

## Un instrument, pas seulement une expérience sociale

Le projet est d'abord une démonstration de cohabitation humain et IA. Mais un monde qui
tourne selon des règles explicites, déterministe, entièrement tracé et rejouable, avec une
interface dense en indicateurs descriptifs, c'est aussi un banc d'essai. Faire varier une
règle à la même graine et mesurer l'effet, c'est de la méthode expérimentale.

Publics possibles à mesure que les couches s'ajoutent : écologie et biologie évolutive
(sélection, spéciation, dynamique de population, boucle organisme-milieu), physique et
chimie de l'environnement (paramètres planétaires, gradients, zones), archéologie et
anthropologie (formation des mythes, mémoire collective ancrée, autopsie des bascules),
sciences sociales (institutions, dissidence, circulation du savoir).

Conséquence concrète : l'interface doit être précise et riche en statistiques dès le
début. Chaque grandeur observable est un instrument de mesure. Le contrat ViewState et
`WorldStats` grandissent dans ce sens.

## Les règles sacrées

Reprises des documents d'origine, tenues comme non négociables. Vérification tranchée par
tranchée dans `GENESIS_FIDELITY.md`.

1. Le monde continue sans le joueur.
2. L'histoire n'est jamais écrite à l'avance.
3. Les conséquences sont persistantes.
4. Les individus possèdent une mémoire.
5. Les sociétés peuvent émerger.
6. Le LLM ne définit pas la réalité.
7. Le rendu n'est qu'une fenêtre sur le monde.
8. Nodyx est la couche numérique, pas le moteur.
9. Les expériences sont reproductibles.
10. Le monde doit pouvoir surprendre ses créateurs.
