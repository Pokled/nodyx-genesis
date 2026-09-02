//! Genere `index.html` : la page de garde d'un monde.
//!
//! La porte d'entree. Un chapeau ecrit a partir des chiffres du monde (aucun LLM), trois
//! portes vers `view.html`, `series.html`, `lives.html`, une courbe de population sur toute
//! la vie du monde, et la chronique des grands tournants. Style : bioluminescence des
//! profondeurs, comme `view.html`.

use serde::Serialize;

const TEMPLATE: &str = include_str!("index_template.html");

/// Le condense d'un monde, calcule par la CLI et injecte dans le gabarit.
#[derive(Serialize)]
pub struct Digest {
    /// nom du dossier du monde, ex. "w2".
    pub name: String,
    pub seed: u64,
    pub engine: String,
    pub ticks: u64,
    pub years: u64,
    pub pop_final: u32,
    pub carrying_capacity: u32,
    pub births: u64,
    pub deaths_starv: u64,
    pub deaths_age: u64,
    pub max_gen: u32,
    pub div_start: f32,
    pub div_end: f32,
    pub agents_awoke: usize,
    pub agents_alive: u32,
    pub mean_mem: f32,
    pub longest_span: u64,
    pub longest_lineage: String,
    pub dominant_lineage: String,
    pub dominant_share: f32,
    /// Noms des especes emergees, dans l'ordre d'apparition.
    pub species: Vec<String>,
    /// Noms des lignees fondatrices eteintes.
    pub extinct: Vec<String>,
    /// Population et capacite de charge echantillonnees (la courbe de garde).
    pub pop_series: Vec<u32>,
    pub cap_series: Vec<u32>,
    pub tick_series: Vec<u64>,
    /// Les grands tournants : (tick, phrase).
    pub chronicle: Vec<(u64, String)>,
}

pub fn render(digest: &Digest) -> String {
    let json = serde_json::to_string(digest)
        .unwrap_or_else(|_| "{}".to_string())
        .replace("</", "<\\/");
    TEMPLATE
        .replace("__WNAME__", &digest.name)
        .replace("__SEED__", &digest.seed.to_string())
        .replace("__DIGEST__", &json)
}
