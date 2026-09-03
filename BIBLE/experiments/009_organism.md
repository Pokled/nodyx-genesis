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
tague `cell_id`, energie mutualisee, reproduction protegee, fusion, **et division** : schema
v19, une cellule grande, mure et etiree se pince en deux, la cellule est devenue une unite
qui se reproduit et sur laquelle la selection agit). Il manque la specialisation et la
co-dependance : c'est la marche organisme proprement dite, ci-dessous.

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

**Premier pas fait (2026-09-03) : l'adhesion / le tissu.** `[cells] tissue` (defaut false,
config seulement). Des cellules de genome proche (`trait_l1 <= tissue_kin`, plus strict que
`fuse_kin`) dont les membranes se touchent (`distance < (r1 + r2) * tissue_reach`) sans
fusionner adherent : un **tissu** = composante connexe de telles cellules, d'au moins
`tissue_min` cellules. Derive chaque tick (union-find sur les positions, ordre d'id, l'id du
tissu = le plus petit id de cellule du groupe), pas d'etat serialise en plus (`Cell.tissue`
porte l'id, `#[serde(default)]`, schema v19 inchange). `world.tissues_alive`, `CellView.tissue`
pour l'overlay. Phase 5b (3d), apres fusion/division, sequentiel sans RNG.

C'est la **liaison** de la piste A, sans l'`organism_id` persistant ni le pool de ressources
ni les roles : juste « ces cellules sont une meme etoffe ». Pas d'hysteresis pour l'instant, le
compte de tissus flotte un peu.

**Adhesion et ordre (meme jour).** `tissue_pull` : les cellules d'un meme tissu se rapprochent
doucement jusqu'au contact (poussee accumulee par membre, bornee, gardee dans la grille). C'est
cette traction qui fait **emerger le pavage**. Et un parametre d'ordre : `world.tissue_order` =
ordre orientationnel a 6 plis (psi6) des centroides de cellules, moyenne sur les cellules en
tissu a >= 3 voisines. `1` = pavage hexagonal parfait, `0` = desordre. Sur w2 avec la traction :
~65 %, un vrai signal hexatique (scenario KTHNY, la phase hexatique ; l'agitation cellulaire =
temperature effective, cf. l'apport de l'utilisateur sur la transition ordre-desordre des
monocouches, confirmee a Leiden 2022). Sans la traction l'ordre reste a 0 (les cellules
flottent, rien ne les pave). Overlay : ligne « tissus N · ordre X % » dans le panneau Cellules.

Ce qui manque, dans l'ordre : le rendu tesselle (membranes poussees jusqu'au contact a
l'ecran, le pavage visible), puis les **roles** (barriere / soutien / moteur / signal /
germinal, `Taille.md` : cellules specialisees + une meme fonction), d'ou emergeront les **types
de tissus** (epithelial / conjonctif / musculaire / nerveux) a nommer comme une espece.

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

### Piste D : le genome structurel (Karl Sims, 1994)

Les pistes A et B tirent le role d'une cellule de sa **position** ou de son **age** ; le doc
le reconnait lui-meme, c'est scripte, pas evolue. La piste C dit d'attendre la chimie pour
avoir un vrai substrat de differenciation. Karl Sims (`Transcription/siggraph94.pdf`, *Evolving
Virtual Creatures*) donne une quatrieme voie que ce doc n'avait pas.

Chez Sims, le genotype n'est pas un vecteur de scalaires, c'est un **graphe oriente** : des
noeuds (unites du corps) et des connexions (comment une unite s'attache a une autre : position,
orientation, echelle, limite de recursion). Le phenotype est *developpe* depuis ce graphe :
une sous-structure decrite une fois est **instanciee** partout ou on la reference (une patte
definie une fois, poussee quatre fois, chacune avec sa copie locale du circuit de controle).
La differenciation vient de la **topologie du graphe** (quel noeud je suis), pas d'un gradient
chimique. La mutation agit sur le graphe : ajouter/retirer un noeud, recabler une connexion,
changer une limite de recursion.

Genesis a un genome de 10 scalaires plats. `cellule -> tissu -> organe` est une progression
**structurelle** et il n'y a rien de structurel a faire evoluer. La piste D ajoute un **second
genome, structurel, heritable, distinct du genome de traits** (schema v19 -> v20, le genome de
traits reste) :

- **`adhesion`** (gene) : bande de parente a laquelle cette cellule *colle* sans fusionner
  (plus proche que `fuse_kin`, plus loin que la repulsion). Un **tissu** = composante connexe
  de cellules qui adherent. C'est la « connexion » de Sims.
- **carte de roles** (gene, ~4 entrees) : signal positionnel (distance au centroide, sur le
  bord, nombre de voisins) -> distribution de roles. Mutable (perturber les poids,
  ajouter/retirer une entree). C'est le « quel noeud je suis » de Sims, reduit a l'echelle
  Genesis : la position fournit le signal, mais la *carte* signal -> role evolue.
- **`role`** sur l'entite (germinal / somatique / structurel / nourricier), reevalue chaque
  tick en passant les signaux de la cellule dans la carte.
- **selection a l'echelle de l'organisme** : le complexe adherent se divise en entier, le
  genome structurel s'herite avec mutation. La selection agit sur l'organisme, comme Sims
  evalue la creature entiere.

Cout : le plus eleve des quatre. Schema v19 -> v20, cablage mutation/heredite d'un second
genome, le brin d'ADN de l'overlay passe de 10 a 11+ traits, tests d'invariants. Mais c'est
la seule piste ou la differenciation est **evoluee** et non declaree.

Lignee ALife de cette voie : Sims -> Sam Kriegman -> xenobots (`Transcription/url_video_et_documents.md`).
Les xenobots (tas de cellules souches qui s'auto-organise en unite fonctionnelle sans genome
de plan corporel) valident la piste A ; le genome structurel de Sims est ce qu'il faut pour
rendre le resultat **hereditaire et selectionnable**.

## Recommandation pour discussion

La **piste B** est la plus interessante par ce qu'elle revele (le gain du somatique) et la
moins couteuse en code, mais la plus risquee pour l'equilibre des mondes. La **piste A** est
la plus sure et la plus continue avec la fusion. La **piste C** est defendable si on tient a
l'ordre de la roadmap. La **piste D** est la seule ou la differenciation est evoluee, mais la
plus lourde.

**Mise a jour 2026-09-03 (apres lecture du dossier `Transcription/`).** Une marche manque
*avant* celle-ci : la **predation**. Le papier Vannier sur l'explosion cambrienne
(`Transcription/L-Explosion-cambrienne...pdf`) est categorique : la montee de complexite du
Cambrien n'est pas l'apparition des cellules/tissus (deja la) mais de la predation et de sa
cascade de retroactions (vision -> predation -> armure -> comportements -> niches -> niveaux
trophiques). Le multicellulaire sans predateur n'a aucune raison de persister : une cellule
coute et ne rapporte presque rien. Avec un predateur, une cellule devient un **refuge de
taille**, avantage selectif durable. w2 le confirme a l'envers (multicellulaire eteint sous
saturation, faute d'avantage). Ordre revise : `cell_burn_relief` (tampon anti-disette,
0.0.2) -> **predation** (`experiments/012_predation.md` a rediger) -> organisme (piste A) ->
genome structurel (piste D). Detail : [[organism-path-predation-first]] en memoire.

Avant de choisir une piste organisme : un prototype autonome (comme `001`), une petite grille,
des cellules avec un role somatique/germinal, pour voir si le gain apparait et si l'ecosysteme
tient. Aucun engagement moteur tant que le prototype n'a pas parle.

## Lecture

`Transcription/` : fonds de paleontologie (Comptes Rendus Palevol 2009, "de l'origine de la vie
a la complexite actuelle", "explosion cambrienne", "emergence des tetrapodes") pour caler les
seuils de bascule sur le vivant reel. Plus, depouilles le 2026-09-03 : `siggraph94.pdf` (Karl
Sims, le genome-graphe, cf. piste D), `url_video_et_documents.md` (lignee Sims -> Kriegman ->
xenobots), `Taille.md` (l'escalier atome -> molecule -> cellule -> tissu -> organe -> appareil
-> organisme, avec ses criteres). Analyse consignee dans `Transcription/analyse.md`.
