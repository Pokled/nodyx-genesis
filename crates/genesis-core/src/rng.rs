//! Generateur pseudo-aleatoire deterministe, ecrit a la main.
//!
//! On ne prend pas de dependance externe pour ca : un monde doit pouvoir tourner des
//! decennies et survivre aux montees de version (tranchee 17). L'algorithme est
//! SplitMix64, connu, simple, sans etat cache. L'etat tient dans un `u64` et il est
//! serialise avec le World State.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn from_seed(seed: u64) -> Self {
        Rng { state: seed }
    }

    /// Prochain u64 du flux. SplitMix64.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// f32 dans [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        // 24 bits de mantisse
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Entier dans [0, n).
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() % n as u64) as usize
    }

    /// Vrai avec probabilite p.
    pub fn chance(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }

    /// Tirage gaussien centre, ecart-type sd. Box-Muller.
    pub fn gaussian(&mut self, sd: f32) -> f32 {
        let u1 = self.next_f32().max(1e-7);
        let u2 = self.next_f32();
        (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos() * sd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_stream() {
        let mut a = Rng::from_seed(42);
        let mut b = Rng::from_seed(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn survives_serde_roundtrip() {
        let mut a = Rng::from_seed(7);
        for _ in 0..123 {
            a.next_u64();
        }
        let json = serde_json::to_string(&a).unwrap();
        let mut b: Rng = serde_json::from_str(&json).unwrap();
        assert_eq!(a.next_u64(), b.next_u64());
    }
}
