# 012. Experience, la predation

Statut : **premiere version sur le moteur** (2026-09-03), `[predation] enabled`, defaut
`false`. Gabarit de `001_emergence.md` et `009_organism.md`. Le prototype numpy
(`experiments/012-predation/`) a servi de degrossissage ; le vrai test se fait sur le moteur
comme les autres marches (bouton d'A/B). La decision d'allumer par defaut revient a
l'utilisateur.

Position : la roadmap (`10_ROADMAP.md`) fait monter Genesis d'echelle. Aujourd'hui les entites
broutent un champ de ressources : un seul niveau trophique, et les seules morts sont la faim
et l'age (`sim.rs`, `DeathCause`). Cette experience ajoute un **second niveau trophique** :
une entite peut se nourrir d'une autre entite.

Elle vient **avant** la marche organisme (`009`). Raison : le pluricellulaire sans predateur
n'a aucune raison de persister (une cellule coute, ne rapporte presque rien ; voir w2,
multicellulaire eteint sous saturation). Avec un predateur, une cellule devient un **refuge de
taille**, avantage selectif durable. C'est la lecon de l'explosion cambrienne (Vannier,
`Transcription/L-Explosion-cambrienne...pdf` ; `Transcription/analyse.md`, seconde passe) :
la montee de complexite est pilotee par la predation et sa cascade, pas par l'environnement.

Lien memoire : [[organism-path-predation-first]].

---

## Question

1. Une entite qui peut manger une autre entite fait-elle **emerger** une chaine alimentaire
   (un sous-ensemble de la population qui vit des autres), de facon mecanisee, jamais un `if`
   qui nomme « predateur » ?
2. La predation cree-t-elle une **pression selective nouvelle** : la taille, la vitesse de
   fuite, l'agregation, la prudence deviennent-elles adaptatives la ou elles ne l'etaient pas ?
3. Le trait `perception`, aujourd'hui presque inerte, devient-il utile (voir venir le danger) ?
4. De facon **deterministe**, reversible (`predation = false`), sans casser les invariants de
   conservation (l'energie et la matiere d'une proie mangee vont quelque part de precis) ?
5. L'ecosysteme **tient-il** ? Une predation trop efficace effondre le monde ; trop faible ne
   change rien. Ou est la fenetre ?

## Les tensions

- **Qui mange qui ?** Il faut un critere mecanise, pas un tag. Pistes : une entite mange une
  entite plus petite (masse / energie / age), assez proche, si son propre niveau d'energie est
  bas (on chasse quand on a faim). Le ratio de taille requis est un parametre.
- **Le cout de la chasse.** Manger une proie doit rapporter net (energie de la proie moins
  l'effort), sinon personne ne chasse. Mais un gain trop grand fait exploser les predateurs
  puis s'effondrer le monde (Lotka-Volterra). A/B tres soigneux.
- **La proie n'est pas passive.** Une entite qui percoit un predateur proche devrait fuir
  (deja : la Voix a une alarme, `cfg.voice.alarm_fear` ; la cognition a `fear_radius`). La
  predation branche enfin ces mecaniques sur un vrai danger.
- **Conservation.** Aujourd'hui une mort rend `body_matter` au stock et l'energie disparait.
  Une proie mangee : sa matiere retourne au stock (comme une mort normale), une fraction de
  son energie passe au predateur, le reste est perdu (chaleur). La fraction transferee est un
  parametre (l'efficacite trophique reelle est ~10 %, mais Genesis n'a pas besoin d'etre si
  dur).
- **La de-simulation.** Sans objet nouveau : la predation ne fait que deplacer de l'energie
  et declencher une mort. Un `DeathCause::Predation` (schema +1), un `EventKind` pour les
  chapitres (« une lignee de chasseurs apparait »).

## Trois pistes

### Piste A : la predation opportuniste (le plus simple)

Pas de trait « predateur ». En phase metabolisme ou juste apres, une entite affamee
(`energy < peril`) qui a dans sa case (ou a portee 1) une entite nettement plus petite
(`autre.energy < self.energy * ratio`, ratio ~0,6) la mange : la proie meurt
(`DeathCause::Predation`), le predateur gagne `proie.energy * transfer` (transfer ~0,5).
Determinisme : ordre d'id, une predation par predateur et par tick, decisions collectees puis
appliquees.

Emergent : aucune entite n'est « un predateur », mais une lignee dont le genome (grande taille
= metabolisme + longevite hauts, ou vitesse haute) rend la chasse rentable se repand. La
chaine alimentaire est un fait statistique, pas une categorie.

Cout : faible. Reutilise la grille spatiale existante. Risque : l'equilibre proie/predateur.

### Piste B : la predation + la fuite (le plus vivant)

Piste A, plus : une entite qui percoit (trait `perception`) un predateur potentiel a portee
change sa cible de deplacement pour s'en eloigner (au lieu de remonter le gradient de
ressources). La predation devient une pression sur `perception` et `vitesse`, et un predateur
doit etre plus rapide que sa proie pour l'attraper. On obtient une course aux armements.

Cout : moyen. Touche la phase de decision de deplacement (`decide` / `forage_target`).

### Piste C : attendre l'organisme

Faire d'abord la marche organisme (`009`) avec un avantage de pool de ressources, puis ajouter
la predation quand il y aura des « grosses choses » (organismes) a manger et des refuges
(cellules) ou se cacher. Defendable, mais c'est l'ordre inverse de ce que dit le Cambrien
(la predation vient avant, elle est *la cause* de la complexification).

## Resultats, premier passage sur le moteur (2026-09-03)

**Piste A implementee**, `[predation]` (`enabled` defaut false, `reach` 2.0, `hunt_below` 4.0,
`prey_frac` 0.5, `transfer` 0.55). Phase 5a, sequentielle, sans RNG, ordre des id, une prise
par predateur et par tick ; la proie meurt en phase 6 avec `DeathCause::Predation`, une entite
mangee consomme quand meme son tirage RNG (flux inchange). Le prototype numpy
(`experiments/012-predation/`) etait reste non concluant apres 3 iterations (feast, artefacts
de chimiotaxie) ; le test sur le vrai moteur, ou l'ecosysteme est deja cale, tranche mieux.

A/B graine 1, monde complet (grille 192, saisons), 60k ticks, `enabled = true` contre `false` :

| mesure | predation ON | OFF |
|---|---|---|
| population de plateau | 6 663 | 7 760 (**-14 %**) |
| population finale | 9 534 | 9 574 (a la capacite) |
| creux de croissance | 24 | 37 |
| morts totales | 99 731 | 75 449 |
| morts par famine | **13 075** | 74 137 |
| morts par predation | **86 338 (86,6 %)** | 0 |
| diversite genetique de plateau | **0,078** | 0,055 (**+42 %**) |
| cellules vivantes (moy.) | 51 | 46 |

Derive des traits (moyenne ON contre OFF) :

| trait | ON | OFF | delta |
|---|---|---|---|
| fecondite | 0,76 | 0,48 | **+0,28** |
| prudence | 0,72 | 0,42 | **+0,30** |
| metabolisme | 0,49 | 0,31 | **+0,17** |
| curiosite | 0,60 | 0,52 | +0,08 |
| perception | 0,92 | 0,87 | +0,05 |
| efficacite | 0,65 | 0,61 | +0,04 |

**Lecture.** Une chaine alimentaire massive emerge sans qu'aucune regle ne nomme un predateur :
86 % de la mortalite passe par la predation, et les morts par famine chutent d'un facteur 6
(on se fait manger avant de mourir de faim). L'ecosysteme **tient** : w2 finit a la capacite.
La population de plateau baisse de 14 % et le creux de croissance est un peu plus profond
(24 contre 37), mais pas d'effondrement.

Et surtout, la predation **diversifie** (+42 % de diversite genetique, a l'inverse de la fusion
et de `cell_burn_relief` qui consolident) et fait bouger les traits **exactement dans le sens
du Cambrien** : prudence (l'armure comportementale, +0,30), fecondite (pondre vite pour
devancer le predateur, +0,28), metabolisme (un corps plus gros, plus dur a manger, +0,17),
perception (voir venir le danger, +0,05). Le trait `perception` devient enfin nettement
adaptatif.

**Ce qui reste** : la piste B (la proie qui fuit ce qu'elle percoit) fermerait la boucle du
« Light Switch » et pourrait remonter la population. Le ratio de taille (`prey_frac`) et le
transfert meritent un balayage. La question « allumer par defaut » est a trancher :
`enabled = true` change les mondes de reference (population -14 %, diversite +42 %), c'est un
choix editorial comme l'a ete la fusion.

## Mesures attendues (pour les prochains passages)

```
chaine_alimentaire = fraction de l'energie ingeree par les entites qui vient d'autres entites
                     (contre : du champ de ressources)
pression_taille    = correlation entre taille (energie moy.) et succes reproductif, ON vs OFF
utilite_perception = correlation entre trait perception et survie, ON vs OFF
resilience         = temps de recuperation apres un choc, ON vs OFF (interdependance = fragilite,
                     cf. Vannier ; c'est le point C de analyse.md)
tenue              = le monde survit-il 200k ticks sans effondrement ?
```

## Lecture

`Transcription/L-Explosion-cambrienne...pdf` (Vannier, 2009) : la predation comme moteur, la
theorie du « Light Switch » (la vision declenche la predation active), l'interdependance
trophique = stabilite + fragilite. `Transcription/` : les radiations de groupes fossiles
(vertebres, tetrapodes...) pour caler plus tard les plans d'organisation des « grosses choses »
mangeables.
