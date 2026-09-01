# 002. Experience, chimie pseudo-cellulaire

Statut : brouillon de travail. Consigne le 2026-09-01 a partir d'une proposition de
l'utilisateur. Non implemente. Meme gabarit que `001_emergence.md` (une question, un
montage, des mesures, des variantes A/B, un critere, un livrable).

Position dans la roadmap : couche tres lointaine, 0.1+ au plus tot. Non bloquante pour les
jalons 0.0.x. Ne change pas la sequence en cours : Dezoomer (voir `02_ARCHITECTURE.md`,
« Un monde qui ne s'arrete jamais »), puis `[planet]` branche sur la simulation, puis
biomes. On y revient apres.

Aujourd'hui, la matiere dans Genesis est un seul scalaire abstrait (ressource / energie)
plus la fertilite du sol. Cette experience teste si on peut poser une vraie chimie
en dessous, sans se noyer dans la complexite.

---

## Question, le pari

Simuler la vraie chimie (cinetique, orbitales, barrieres energetiques) est hors de portee
pour un jeu. L'astuce : des regles **abstraites**, ou pseudo-chimiques, tirees des
metadonnees d'une table periodique (electronegativite, etats d'oxydation, bloc, etat
standard, couleur CPK) au lieu de la formule exacte.

Des regles simples de ce genre, sur un automate cellulaire dans l'esprit du Jeu de la Vie,
produisent-elles :

1. des structures **emergentes reconnaissables** qui **tiennent** dans le temps (eau, chaines carbonees),
2. des **reactions** simples et lisibles (combustion),
3. de facon **deterministe** et a etat **borne**,
4. assez peu couteuses pour tourner **a cote** de la simulation principale,

sans coder a la main chaque reaction, et sans aucune regle qui nomme "eau" ou "combustion"
dans sa logique ?

Si rien de stable n'emerge, le modele abstrait est a repenser avant d'y investir.

---

## Montage

Automate cellulaire sur une grille. Chaque case est :

```rust
enum Cell {
    Empty,
    Atom(ElementBiogene),
    Molecule(MoleculeType), // eau, chaine, ... optionnel pour demarrer
}
```

Boucle de tick d'automate : l'etat suivant d'une case ne depend que de son etat et de ses
voisins (8 voisins, ordre stable). Comme le tick du moteur, chaque phase se termine avant
la suivante.

**Source des donnees.** Crate `sandmor/periodic-table-on-an-enum` (MIT, enum des elements
plus 17 proprietes, codegen depuis `PubChemElements_all.json`). Reserve deja notee dans la
memoire `ref-periodic-table-crate` : peu maintenue, pas de release. On vendorise plutot le
JSON PubChem sous-jacent et on reprend le patron enum plus codegen.

**Pont vers la crate, esquisse fournie par l'utilisateur.** Un enum restreint aux six
elements biogenes, avec une fonction qui va chercher les vraies metadonnees dans la crate
complete. Un seul point de contact, facile a remplacer si on vendorise le JSON.

```rust
use periodic_table_on_an_enum::Element;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ElementBiogene {
    Hydrogen,   // H : base de l'eau
    Oxygen,     // O : eau et energie (respiration)
    Carbon,     // C : squelette de toutes les molecules organiques
    Nitrogen,   // N : ADN et proteines
    Phosphorus, // P : stockage d'energie (ATP), lie aux logs d'energie du moteur
    Sulfur,     // S : solidite des structures
}

impl ElementBiogene {
    pub fn to_core_element(&self) -> Element {
        match self {
            ElementBiogene::Hydrogen => Element::Hydrogen,
            ElementBiogene::Oxygen => Element::Oxygen,
            ElementBiogene::Carbon => Element::Carbon,
            ElementBiogene::Nitrogen => Element::Nitrogen,
            ElementBiogene::Phosphorus => Element::Phosphorus,
            ElementBiogene::Sulfur => Element::Sulfur,
        }
    }
}
```

**Jeu de depart : CHNOPS, les six elements de la vie.** Ils composent environ 98 pour cent
de la matiere vivante. On ne cherche pas les millions de combinaisons, on prend le sous
ensemble qui permet a des molecules biologiques d'emerger.

| Symbole | Element | Priorite | Role visé | Couleur CPK |
|---|---|---|---|---|
| H | Hydrogene | indispensable | eau, fluide | blanc |
| O | Oxygene | indispensable | eau, energie, oxydation | rouge |
| C | Carbone | indispensable | squelette des molecules, 4 liaisons | noir ou gris tres fonce |
| N | Azote | tres important | briques de la replication, ADN | bleu |
| P | Phosphore | essentiel ensuite | stockage et transfert d'energie, mouvement | orange |
| S | Soufre | essentiel ensuite | ponts, rigidite, solidite | jaune |

Note : ce set abandonne la demonstration "sel qui cristallise" (il fallait Sodium et
Chlore) au profit d'un set ou c'est la chimie du vivant qui peut apparaitre. C'est le bon
compromis si la chimie doit un jour porter des molecules vivantes plutot que juste des
cristaux.

---

## Les trois familles de regles

Formulees par l'utilisateur, gardees telles quelles.

### 1. Affinite et assemblage, l'attraction

Un atome cherche des voisins qui le completent. Le nombre de liaisons souhaite vient des
etats d'oxydation (`get_oxidation_states()`) : Hydrogene 1, Oxygene 2, Azote 3, Carbone 4,
Soufre 2, Phosphore 3 ou 5.

- Regle generale : un atome dont le voisinage satisfait son nombre de liaisons souhaite
  devient stable et cesse de diffuser (cellule liee, ou `Molecule`).
- Oxygene avec exactement 2 voisins Hydrogene : se stabilise (eau).
- Carbone avec 4 voisins lies (autres Carbones, Hydrogenes) : forme un noeud de chaine, un
  squelette.
- Soufre entre deux atomes deja lies : agit comme un pont, rigidifie la structure.
- Phosphore lie a plusieurs Oxygenes : forme un noeud qui peut porter une charge d'energie
  (voir combustion).

### 2. Loi des gaz, diffusion et structure

L'etat standard de l'element a temperature ambiante (via `get_group_block` et l'etat
physique) anime la grille.

- Une cellule gaz a tendance a diffuser vers une case vide voisine a chaque tick.
- Une cellule solide reste fixe et sert de structure ou de support.
- Un liquide, intermediaire, coule vers le bas et s'etale.

Le lien avec `[planet].temperature_c` est naturel : la temperature decide de l'etat
physique d'un element, donc de son comportement sur la grille.

### 3. Energie, la surpopulation chimique

Au lieu de mourir par nombre brut de voisins comme dans le Jeu de la Vie, on somme
l'electronegativite des voisins.

- Somme trop elevee, milieu tres reactif : l'atome est instable et redevient `Empty`, il
  reagit violemment et se disperse.
- Aucun voisin : un atome instable seul se dissipe.

Esquisse de la mise a jour d'une case, dans l'esprit voulu :

```rust
fn next_atom(atome: Element, voisins: &[Element]) -> Cell {
    let en_totale: f32 = voisins.iter()
        .map(|v| v.electronegativity().unwrap_or(0.0))
        .sum();

    if en_totale > SEUIL_REACTIF {
        return Cell::Empty; // reaction violente
    }
    if voisins.is_empty() {
        return Cell::Empty; // dissipation
    }
    // affinite : voisinage qui satisfait le nombre de liaisons voulu -> stable, ne diffuse plus
    let voulu = atome.liaisons_voulues(); // derive des etats d'oxydation
    if voisins.len() >= voulu {
        return Cell::Atom(atome); // liee
    }
    Cell::Atom(atome) // survit par defaut, continue de chercher
}
```

---

## Comportements emergents vises, le critere concret

A la meme graine, on veut voir apparaitre, sans regle qui les nomme :

- **Eau.** 2 Hydrogenes plus 1 Oxygene adjacents s'arretent de bouger, bloc stable.
- **Chaine carbonee.** Des Carbones qui se lient en file ou en reseau, un squelette qui
  tient et sert de support aux autres atomes.
- **Combustion.** Le Carbone s'etend s'il y a de l'Oxygene a cote, en liberant de
  l'energie et en detruisant les Hydrogenes qu'il rencontre.
- **Pont soufre.** Du Soufre qui relie deux fragments de chaine et les rigidifie.
- **Noeud d'energie.** Un motif Phosphore plus Oxygenes qui accumule une charge quand il y
  a de l'energie disponible et la relache ailleurs. L'embryon d'un porteur d'energie.

---

## Affichage

Couleurs CPK par element (`get_cpk_color`) : la grille se colore toute seule. Hydrogene
blanc, Oxygene rouge, Carbone noir ou gris tres fonce, Azote bleu, Phosphore orange,
Soufre jaune. Le lecteur reutilise le meme mecanisme que les couches actuelles (ressource,
fertilite, surexploitation) : une couche de plus dans le selecteur.

---

## Variantes, en A/B a la meme graine (tranchee 16)

| Variante | Ce qu'on ajoute | Attendu |
|---|---|---|
| V0 | diffusion seule | un nuage de gaz qui se disperse, rien de stable |
| V1 | plus affinite et liaisons | premiers blocs eau, premieres chaines carbonees |
| V2 | plus energie et destruction | structures qui se defont dans les zones tres reactives, combustion |
| V3 | plus `Molecule(MoleculeType)` explicite | molecules stables durables, ponts soufre, noeuds d'energie |

---

## Critere de reussite

V1 ou V2 produit de l'eau et des chaines carbonees reconnaissables et stables, la
combustion detruit bien les Hydrogenes, le tout reste deterministe et a etat borne, et
aucune regle ne nomme le resultat. Si ni V1 ni V2 n'y arrivent, le modele abstrait est a
repenser avant d'aller plus loin, et c'est exactement ce qu'on voulait savoir tot.

---

## La question architecturale, ouverte

La pseudo-chimie est-elle le futur substrat de la matiere, ou une couche optionnelle
separee ? On ne tranche pas maintenant. Les deux options, avec leurs consequences :

**Substrat.** A terme, la matiere et l'energie de Genesis deviennent a base d'elements. Les
entites-blobs actuelles seraient refondees dessus.
- Pour : "molecule" devient litteral ; le genome a 6 traits pourrait devenir emergent de la
  chimie ; une seule verite physique pour tout le monde.
- Contre : reecriture du champ de ressources, croissance du World State, gros chantier ; le
  determinisme doit tenir sur l'automate complet ; risque de tout casser.

**Couche separee.** Le moteur abstrait actuel reste. La chimie est un module que certains
mondes activent, a cote.
- Pour : additif, plus petit, testable en isole ; on peut l'abandonner sans degats.
- Contre : deux modeles de matiere a reconcilier ; un pont necessaire, par exemple la
  "ressource" que mangent les entites serait produite par la couche chimie.

Decision repoussee : on y revient quand Dezoomer et le branchement `[planet]` sont faits,
et qu'on a de vrais mondes sous les yeux pour juger.

---

## Livrable, quand on y sera

Un prototype isole dans `experiments/002-chemistry/` : un binaire Rust autonome, une
graine, la grille CPK rendue dans le temps, un paragraphe de verdict. Meme forme que
l'experience 001. Ne touche pas au moteur principal tant que la question architecturale
n'est pas tranchee.
