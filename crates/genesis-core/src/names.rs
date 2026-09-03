//! Vocabulaire du monde : des noms prononcables et deterministes pour les lignees et les
//! especes. Aucun hasard de run : meme graine, meme monde, memes noms (invariant 9). Un
//! nom n'est qu'une etiquette lisible, il ne revient jamais dans la simulation.

const ONSET: [&str; 16] = [
    "k", "t", "s", "r", "n", "m", "v", "l", "d", "th", "br", "sh", "gr", "z", "kh", "dr",
];
// Voyelles simples surtout : des noms courts et prononcables.
const NUCLEUS: [&str; 10] = ["a", "e", "i", "o", "u", "a", "e", "i", "o", "ae"];
const CODA: [&str; 10] = ["n", "r", "s", "l", "th", "", "", "", "", ""];

fn mix(x: u64) -> u64 {
    // Un pas de SplitMix64 : melange fort, deterministe.
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Un nom invente a partir d'une graine. `salt` separe les espaces de noms (lignees,
/// especes) pour qu'ils ne se recouvrent pas.
pub fn coined(seed: u64, salt: u64) -> String {
    let mut h = mix(seed ^ salt.rotate_left(17));
    // 2 syllabes le plus souvent, 3 une fois sur quatre.
    let syllables = if h % 4 == 0 { 3 } else { 2 };
    let mut out = String::new();
    let mut prev_on = usize::MAX;
    for i in 0..syllables {
        h = mix(h);
        let mut oi = (h % ONSET.len() as u64) as usize;
        if oi == prev_on {
            oi = (oi + 1) % ONSET.len();
        }
        prev_on = oi;
        let on = ONSET[oi];
        h = mix(h);
        let nu = NUCLEUS[(h % NUCLEUS.len() as u64) as usize];
        h = mix(h);
        // coda seulement en fin de mot, et pas systematiquement : plus fluide
        let co = if i + 1 == syllables {
            CODA[(h % CODA.len() as u64) as usize]
        } else {
            ""
        };
        if i == 0 {
            let mut c = on.chars();
            if let Some(first) = c.next() {
                out.extend(first.to_uppercase());
                out.push_str(c.as_str());
            }
        } else {
            out.push_str(on);
        }
        out.push_str(nu);
        out.push_str(co);
    }
    out
}

/// Nom d'une lignee fondatrice (une par fondateur du monde).
pub fn lineage_name(lineage: u16) -> String {
    coined(lineage as u64, 0x9E37_79B9)
}

/// Nom d'une espece emergee (identifiant sequentiel attribue par le veilleur).
pub fn species_name(species: u32) -> String {
    coined(species as u64, 0xC2B2_AE35)
}

/// Nom d'un organisme reconnu (identifiant sequentiel, 0.0.2).
pub fn organism_name(organism: u32) -> String {
    coined(organism as u64, 0x0126_5EED_0000_0001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_stable_and_distinct() {
        assert_eq!(lineage_name(0), lineage_name(0));
        assert_ne!(lineage_name(0), lineage_name(1));
        assert_ne!(species_name(0), species_name(1));
        // les deux espaces ne se recouvrent pas pour un meme indice
        assert_ne!(lineage_name(3), species_name(3));
        for i in 0..200u32 {
            let n = species_name(i);
            assert!(n.len() >= 3 && n.len() <= 18, "nom hors bornes : {} ({})", n, n.len());
            assert!(n.chars().next().unwrap().is_uppercase());
        }
    }
}
