# 007. Experience, la fusion de cellules

Statut : **implementee et active** (2026-09-02). Jalon 0.0.2, tranche 2, etape 2b.
Gabarit de `001_emergence.md`. Suite de `006_cell_transition.md`.

Position : l'etape 1 (membrane) a donne aux amas coherents de parents une identite et un
avantage de groupe. L'etape 2b prolonge cette identite : deux cellules deja formees peuvent
n'en faire plus qu'une. C'est le premier pas vers le pluricellulaire, avant l'etape 2 (la
vraie de-simulation des membres, toujours deferee a 0.0.6).

---

## Question

1. Deux cellules stables aux membranes qui se chevauchent et aux genomes proches
   **fusionnent-elles**, de facon mecanisee (jamais un `if` qui la nomme, T-7) ?
2. La fusion **change-t-elle le monde** de facon mesurable, et dans quel sens ?
3. De facon **deterministe** et sans casser l'invariant d'effectif des cellules ?

## Montage

Nouvelle etape 3b dans `cell_phase` (`sim.rs`), entre l'entretien (3) et la detection de
nouvelles cellules (4). Sequentielle, sans RNG.

- Pour chaque paire de cellules `(a, b)` avec `a.id < b.id` (ordre stable, les cellules sont
  triees par id), toutes deux plus vieilles que `grace_ticks` :
  - **chevauchement** : `distance(centres) < (radius_a + radius_b) * fuse_overlap` (0,5 :
    les membranes doivent vraiment s'interpenetrer, pas juste se toucher).
  - **parente** : `L1(mean_traits_a, mean_traits_b) <= fuse_kin` (0,9). Aux valeurs
    actuelles ce test n'est jamais contraignant : deux cellules qui se chevauchent sont
    deja proches (elles se sont formees du meme stock regional). Il reste comme garde-fou
    d'une config extreme.
  - Si les deux passent : la plus grosse (a effectif egal, le plus petit id, donc la plus
    ancienne) garde son `id`, son `formed_tick`, son histoire ; l'autre est marquee
    absorbee. Une fusion par cellule et par tick.
- Application : on retagge les `cell_id` des membres de la cellule absorbee vers la
  survivante, on retire l'absorbee de `world.cells`, on ajoute `gone_count` a
  `member_count` de la survivante, on emet `CellsMerged { cell, absorbed, size, at }`.

Deterministe : ordre de paires fixe (indices tries par id), decisions collectees puis
appliquees, aucun tirage RNG. L'invariant `somme(member_count) == entites taguees` tient
aussi les ticks de fusion (l'absorbee perd `gone_count`, la survivante le gagne).

Schema v15 : `WorldState.cells_merged_total`, `EventKind::CellsMerged` (saillance 232, entre
`SpeciesEmerged` et `CellFormed`). Bloc `[cells]` : `fuse` (bouton d'A/B), `fuse_overlap`,
`fuse_kin`.

## Resultats

Graine 1, 60 000 ticks, `fuse = true` contre `fuse = false`.

| mesure | fusion ON | fusion OFF | effet |
|---|---|---|---|
| population finale | 2291 | 2292 | inchangee (plateau de capacite) |
| naissances totales | 9582 | 9105 | +5 % |
| morts par famine | 4973 | 4475 | +11 % |
| morts par age | 2320 | 2340 | inchangees |
| generation moyenne | 9,3 | 9,4 | -1 % |
| diversite genetique finale | 0,052 | 0,059 | **-12 %** |
| pic de diversite | 0,147 | 0,147 | inchange |
| cellules formees | 298 | 267 | +12 % |
| cellules dissoutes | 150 | 233 | **-36 %** |
| fusions | 120 | 0 | (la mecanique) |

Lecture : la fusion **rend les cellules beaucoup plus tenaces**. Au lieu de se dissoudre, une
petite cellule en peine rejoint une voisine et survit : les dissolutions chutent d'un tiers.
Les cellules deviennent plus grosses et plus stables. Le partage d'energie s'exerce alors
sur plus de membres : quand la cellule traverse une famine locale, plus de membres frolent
le peril ensemble, d'ou les +11 % de morts par famine, compenses par plus de naissances
(reproduction protegee sur des cellules qui durent). La population, deja bornee par la
capacite, ne bouge pas.

La diversite genetique finale baisse de 12 %. La fusion agrege des cellules genetiquement
similaires en super-cellules plus uniformes ; ces unites persistantes brassent moins les
lignees. **La fusion est une force de consolidation**, exactement a l'inverse de l'alarme de
la Voix (`00_INDEX.md`, tranche Voix), qui disperse et diversifie (+37 %). Un monde a les
deux.

`fuse_kin` 0,8 / 1,0 / 1,4 donnent des resultats identiques : le facteur limitant est le
chevauchement geometrique et `grace_ticks`, pas la distance genetique. Valeur retenue 0,9.

Determinisme byte-identique 1 vs 8 threads (graines 1 et 42). `replay` OK. Test
`cells_merge_when_membranes_overlap` : il s'en produit (graine 1, ~1 toutes les 500 ticks),
le compteur cumule colle, l'invariant d'effectif tient, `fuse = false` en produit zero.

Sur les quatre mondes de demonstration : w2 120 fusions, w4 83, w1 62, w3 0 (il meurt avant
que des cellules se forment).

## Ce qui reste

- Etape 2 : la vraie de-simulation. Les membres quittent `world.entities`, la cellule
  devient un bilan (molecule, energie, matiere). Retravaille les invariants de conservation.
  Deferee a 0.0.6.
- Une cellule fusionnee pourrait garder une trace de ses deux origines (vers la memoire
  collective, T-8).
