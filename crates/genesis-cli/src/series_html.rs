//! Genere `series.html` : le graphe d'evolution genetique du monde.
//!
//! Fichier unique, aucune dependance, s'ouvre par double-clic. La serie temporelle est
//! embarquee. Style editorial (comme les pages d'experience), sans emoji, sans cadratin.

use genesis_core::persist::WorldMeta;
use genesis_core::SimConfig;
use genesis_view::SeriesRow;

const TEMPLATE: &str = include_str!("series_template.html");
const COMMON_JS: &str = include_str!("reader_common.js");

pub fn render(name: &str, meta: &WorldMeta, cfg: &SimConfig, series: &[SeriesRow]) -> String {
    let series_json = serde_json::to_string(series)
        .unwrap_or_else(|_| "[]".to_string())
        .replace("</", "<\\/");
    let last_gen = series.last().map(|r| r.max_generation).unwrap_or(0);
    let meta_json = serde_json::json!({
        "seed": meta.seed,
        "engine_version": meta.engine_version,
        "ticks_played": meta.ticks_played,
        "tick_seconds": cfg.time.tick_duration_seconds,
        "max_generation": last_gen,
    })
    .to_string()
    .replace("</", "<\\/");

    TEMPLATE
        .replace("__COMMON_JS__", COMMON_JS)
        .replace("__WNAME__", name)
        .replace("__SEED__", &meta.seed.to_string())
        .replace("__ENGINE__", &meta.engine_version)
        .replace("__TICKS__", &meta.ticks_played.to_string())
        .replace("__GENMAX__", &last_gen.to_string())
        .replace("__META__", &meta_json)
        .replace("__SERIES__", &series_json)
}
