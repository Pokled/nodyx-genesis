//! Index spatial : un decoupage de la grille en cases, pour trouver vite les entites
//! proches d'un point sans comparer toutes les paires.
//!
//! Construit par tri par comptage : O(nombre d'entites + nombre de cases), sans arbre ni
//! allocation par case. Il range des **indices** dans la tranche d'entites (le `Vec` est
//! trie par id, donc l'ordre des indices est l'ordre des id) : deterministe, et les
//! requetes renvoient directement des indices exploitables sans recherche.

use crate::entity::{Entity, Position};
use crate::world::Space;

pub struct SpatialHash {
    bin: f32,
    cols: i32,
    rows: i32,
    /// Pour la case lineaire `k` : les indices sont `items[start[k] .. start[k + 1]]`.
    start: Vec<u32>,
    items: Vec<u32>,
}

impl SpatialHash {
    /// Range chaque entite (par son indice dans `entities`) dans une case de cote `bin`.
    pub fn build(entities: &[Entity], space: &Space, bin: f32) -> Self {
        let bin = bin.max(0.5);
        let cols = ((space.width as f32 / bin).ceil() as i32).max(1);
        let rows = ((space.height as f32 / bin).ceil() as i32).max(1);
        let ncells = (cols as usize) * (rows as usize);

        let cell_index = |p: Position| -> usize {
            let cx = ((p.x / bin).floor() as i32).clamp(0, cols - 1);
            let cy = ((p.y / bin).floor() as i32).clamp(0, rows - 1);
            (cy as usize) * (cols as usize) + (cx as usize)
        };

        // 1. compter par case
        let mut count = vec![0u32; ncells + 1];
        for e in entities {
            count[cell_index(e.position) + 1] += 1;
        }
        // 2. somme prefixe -> offsets
        for i in 1..count.len() {
            count[i] += count[i - 1];
        }
        let start = count.clone();
        // 3. placer les indices, dans l'ordre du Vec (donc des id)
        let mut cursor = count;
        let mut items = vec![0u32; entities.len()];
        for (idx, e) in entities.iter().enumerate() {
            let k = cell_index(e.position);
            items[cursor[k] as usize] = idx as u32;
            cursor[k] += 1;
        }

        SpatialHash { bin, cols, rows, start, items }
    }

    fn cell_of(&self, p: Position) -> (i32, i32) {
        (
            ((p.x / self.bin).floor() as i32).clamp(0, self.cols - 1),
            ((p.y / self.bin).floor() as i32).clamp(0, self.rows - 1),
        )
    }

    fn cell_slice(&self, cx: i32, cy: i32) -> &[u32] {
        let k = (cy as usize) * (self.cols as usize) + (cx as usize);
        &self.items[self.start[k] as usize..self.start[k + 1] as usize]
    }

    /// Appelle `f` sur chaque indice d'entite du bloc de cases autour de `pos`, sans
    /// allouer. Ordre : cases puis indices dans la case, deterministe.
    pub fn for_each_neighbor(&self, pos: Position, radius: f32, mut f: impl FnMut(u32)) {
        let r = (radius / self.bin).ceil() as i32;
        let (cx, cy) = self.cell_of(pos);
        for gy in (cy - r).max(0)..=(cy + r).min(self.rows - 1) {
            for gx in (cx - r).max(0)..=(cx + r).min(self.cols - 1) {
                for &idx in self.cell_slice(gx, gy) {
                    f(idx);
                }
            }
        }
    }

    /// Nombre d'entites dans la meme case que `pos` (densite locale grossiere).
    pub fn count_in_cell(&self, pos: Position) -> u32 {
        let (cx, cy) = self.cell_of(pos);
        self.cell_slice(cx, cy).len() as u32
    }
}
