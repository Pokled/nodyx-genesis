# 009. Experience, la marche organisme (exploration, pas encore decidee)

Statut : **exploration de conception**. Rien n'est implemente. Gabarit de `001_emergence.md`.
Ce document pose les options pour la prochaine marche de l'escalier des echelles
(`10_ROADMAP.md`), avant de s'engager. La decision revient a l'utilisateur, comme pour la
de-simulation (tranche 8) et le decoupage en deux temps de la bascule cellule.

Position : la roadmap place cette marche a "0.0.2 vers 0.0.3". L'utilisateur a choisi de
sauter directement au pont vers l'agent (0.0.3) et de ne de-simuler la biologie que
partiellement (la sante, un scalaire de fond). La marche organisme est donc le morceau
deferre. La fusion de cellules (`007_cell_fusion.md`) est le premier pas dans sa direction :
des membranes qui s'agregent.

---

## Le vivant reel, en une phrase

Un organisme pluricellulaire, c'est un agregat de cellules qui ne peuvent plus vivre seules,
qui se sont **specialisees** (des tissus : nourrir, defendre, se reproduire, tenir), et qui
partagent un destin (une seule ligne germinale, un cycle de vie commun). Tissu, organe,
appareil sont sa structure interne, pas des niveaux qu'on pilote separement (`10_ROADMAP.md`).

Genesis a aujourd'hui : l'entite (molecule) qui se divise, la cellule (amas de parents
tague `cell_id`, energie mutualisee, reproduction protegee, fusion). Il manque la
specialisation et la co-dependance.

## Question

1. Un agregat de cellules **liees, differenciees et co-dependantes** peut-il **emerger** de
   facon mecanisee (jamais un `if`), a partir des cellules existantes ?
2. L'organisme donne-t-il un **avantage mesurable** que des cellules libres n'ont pas ?
3. Sans casser les invariants (etape 1), de facon deterministe et reversible ?
4. La **specialisation** (les tissus) nait-elle, ou faut-il la declarer ?

## Les tensions

- **La fusion agrege du semblable.** Deux cellules fusionnent parce que leurs genomes se
  ressemblent (`007`). Un organisme, lui, a besoin de cellules **differentes** qui cooperent.
  Il faut soit un autre mecanisme de liaison que la fusion, soit accepter que la
  differenciation vienne apres la liaison (des cellules identiques se lient, puis divergent
  selon leur role).
- **Qu'est-ce qu'un role ?** Une cellule ne "fait" rien de particulier aujourd'hui, elle est
  juste un groupe. Un role a besoin d'un effet concret : une cellule "nourriciere" recolte
  pour l'organisme, une cellule "germinale" est la seule a se reproduire, une cellule
  "structurelle" immobilise de la matiere pour tenir la forme.
- **La de-simulation.** Tot ou tard les entites internes doivent cesser d'etre simulees une a
  une (la cellule devient un bilan), sinon un monde de dix mille organismes de cent cellules
  chacun, c'est un million d'entites. C'est l'escalier etape 2, la vraie rupture d'invariant.

## Trois pistes

### Piste A : l'organisme colonial (le plus proche de l'existant)

Un organisme est un **amas persistant de cellules liees**, detecte comme les cellules sont
detectees des entites : proximite + parente + persistance, un cran au-dessus.

- Liaison : deux cellules dont les membranes se touchent longtemps sans fusionner (genomes
  trop distants pour `007`, mais assez proches pour cooperer) se lient. Un `organism_id` sur
  la `Cell`, un `Vec<Organism>` sur le `WorldState`.
- Avantage : **pool de ressources a l'echelle de l'organisme**. Les cellules de l'organisme
  partagent l'energie entre elles, pas seulement en interne. Une cellule dans une zone riche
  nourrit une cellule dans une zone pauvre.
- Specialisation emergente : au sein d'un organisme, la cellule la plus centrale (loin du
  bord) tend a devenir "germinale" (seule a alimenter la reproduction de l'organisme), les
  cellules peripheriques deviennent "nourricieres" (elles recoltent, elles ne se
  reproduisent plus). Le role suit la position, il n'est pas tire au sort.
- Reversible : un organisme qui perd sa cohesion se defait en cellules libres.

Cout : moyen. Reutilise l'infra cellule. Pas de rupture d'invariant (etape 1). Le risque :
que le pool de ressources trop genereux ecrase la selection locale.

### Piste B : la ligne germinale (le plus fidele a la biologie)

On saute la colonie et on va directement a l'idee-cle : **une seule cellule d'un groupe se
reproduit, les autres la servent et meurent avec elle**.

- Dans une cellule assez grande et assez vieille, une sous-population de membres devient
  "somatique" : ils ne se divisent plus (leur `birth_loss` monte a 1), ils recoltent et
  versent leur surplus a la cellule. Les autres restent "germinaux".
- La cellule entiere partage alors un cycle de vie : si les germinaux disparaissent, les
  somatiques meurent vite (co-dependance dure).
- Avantage : les somatiques, liberes de la reproduction, peuvent pousser des traits que la
  selection individuelle ne favoriserait jamais (perception extreme, immobilite econome).
  C'est le vrai gain du pluricellulaire.

Cout : plus faible en code, plus fort en consequences ecologiques (il faut A/B tres
soigneusement : couper la reproduction d'une part de la population peut effondrer un monde).
Pas de nouvel objet, juste un etat de role sur l'entite.

### Piste C : attendre, et d'abord la chimie

La differenciation cellulaire reelle repose sur un environnement interne (gradients de
molecules). Genesis n'a pas de couche chimie (`experiments/002`, 0.1+). Sans elle, toute
specialisation qu'on code est arbitraire (position, age, tirage). La piste C consiste a
**ne pas faire cette marche maintenant**, a finir 0.0.4 (Voix) et 0.0.5 (Societe), et a
revenir a l'organisme quand la chimie donnera un vrai substrat a la differenciation.

Cout : nul. Mais la roadmap avance dans l'ordre social avant l'ordre biologique, ce qui est
un choix a assumer.

## Recommandation pour discussion

La **piste B** est la plus interessante par ce qu'elle revele (le gain du somatique) et la
moins couteuse en code, mais la plus risquee pour l'equilibre des mondes. La **piste A** est
la plus sure et la plus continue avec la fusion. La **piste C** est defendable si on tient a
l'ordre de la roadmap.

Avant de choisir : un prototype autonome (comme `001`), une petite grille, des cellules avec
un role somatique/germinal, pour voir si le gain apparait et si l'ecosysteme tient. Aucun
engagement moteur tant que le prototype n'a pas parle.

## Lecture

`Transcription/` contient un fonds de paleontologie (Comptes Rendus Palevol 2009,
"de l'origine de la vie a la complexite actuelle", "explosion cambrienne", "emergence des
tetrapodes"). A depouiller pour caler les seuils de bascule sur ce que dit le vivant reel,
pas sur l'intuition, quand cette marche sera engagee.
