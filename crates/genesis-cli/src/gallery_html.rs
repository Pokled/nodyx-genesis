//! Genere la page de garde de la bibliotheque : `<dossier>/index.html`, une grille de tous
//! les mondes du dossier. Reconstruite a chaque `run`, ou a la demande (`genesis gallery`).

use serde::Serialize;

const TEMPLATE: &str = include_str!("gallery_template.html");

/// Une carte de monde dans la grille. Tout vient des fichiers deja ecrits (`meta.json`,
/// derniere ligne de `series.jsonl`, `lives.jsonl`).
#[derive(Serialize)]
pub struct Card {
    /// nom du dossier, ex. "w2".
    pub name: String,
    pub seed: u64,
    pub schema: u32,
    pub ticks: u64,
    pub years: u64,
    pub pop: u32,
    pub carrying: u32,
    pub generations: u32,
    pub diversity: f32,
    pub agents_alive: u32,
    pub agents_awoke: usize,
    /// nom prononcable de la lignee dominante.
    pub dominant: String,
    /// population echantillonnee, pour une mini-courbe.
    pub pop_series: Vec<u32>,
    /// `true` si le monde s'est eteint (population finale nulle).
    pub extinct: bool,
}

pub fn render(cards: &[Card]) -> String {
    let json = serde_json::to_string(cards)
        .unwrap_or_else(|_| "[]".to_string())
        .replace("</", "<\\/");
    TEMPLATE.replace("__CARDS__", &json)
}
