# 012. Experience, la predation (exploration, pas encore decidee)

Statut : **exploration de conception**. Rien n'est implemente. Gabarit de `001_emergence.md`
et `009_organism.md`. Ce document pose les options avant de s'engager. La decision revient a
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

## Recommandation pour discussion

**Piste B** est la plus fidele au Cambrien (predation + reponse anti-predateur = la cascade)
et rend enfin `perception` adaptatif. **Piste A** est le prototype a faire d'abord, isole,
pour trouver la fenetre d'equilibre proie/predateur avant de brancher la fuite.

Avant tout engagement moteur : un prototype autonome (comme `001`), petite grille, entites
avec predation opportuniste, pour voir si (a) une chaine alimentaire emerge, (b) l'ecosysteme
tient, (c) la taille / la vitesse deviennent adaptatives. Aucune ligne dans `sim.rs` tant que
le prototype n'a pas parle.

## Mesures attendues

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
