//! Genere `view.html` : un lecteur autonome des frames du monde.
//!
//! Fichier unique, aucune dependance, s'ouvre par double-clic. Les frames sont embarquees.
//! Style : sombre, propre, sans emoji, sans tiret cadratin.
//!
//! On assemble par substitution de marqueurs, pas avec `format!`, pour ne pas avoir a
//! echapper les accolades du CSS et du JS.

use genesis_core::persist::WorldMeta;
use genesis_core::SimConfig;
use genesis_view::ViewFrame;

const TEMPLATE: &str = include_str!("view_template.html");

pub fn render(meta: &WorldMeta, cfg: &SimConfig, frames: &[ViewFrame]) -> String {
    // La fertilite du sol est statique : on l'extrait une fois et on la retire de chaque
    // frame. Sur un long run c'est la moitie du poids du fichier en moins.
    let mut value = serde_json::to_value(frames).unwrap_or_else(|_| serde_json::json!([]));
    let mut terrain = serde_json::json!([]);
    if let Some(arr) = value.as_array_mut() {
        for (i, fr) in arr.iter_mut().enumerate() {
            if let Some(fert) = fr.pointer_mut("/resources/fertility") {
                if i == 0 {
                    terrain = fert.clone();
                }
                *fert = serde_json::json!([]);
            }
        }
    }
    let frames_json = serde_json::to_string(&value)
        .unwrap_or_else(|_| "[]".to_string())
        .replace("</", "<\\/");
    let terrain_json = serde_json::to_string(&terrain)
        .unwrap_or_else(|_| "[]".to_string())
        .replace("</", "<\\/");
    let meta_json = serde_json::json!({
        "seed": meta.seed,
        "engine_version": meta.engine_version,
        "ticks_played": meta.ticks_played,
        "grid": [cfg.world.grid_width, cfg.world.grid_height],
        "tick_seconds": cfg.time.tick_duration_seconds,
    })
    .to_string()
    .replace("</", "<\\/");

    TEMPLATE
        .replace("__SEED__", &meta.seed.to_string())
        .replace("__ENGINE__", &meta.engine_version)
        .replace("__GW__", &cfg.world.grid_width.to_string())
        .replace("__GH__", &cfg.world.grid_height.to_string())
        .replace("__TSEC__", &cfg.time.tick_duration_seconds.to_string())
        .replace("__TEMP__", &format!("{:.0}", cfg.planet.temperature_c))
        .replace("__MEDIUM__", &cfg.planet.medium)
        .replace("__GRAV__", &format!("{:.2}", cfg.planet.gravity))
        .replace("__PRESS__", &format!("{:.2}", cfg.planet.pressure_atm))
        .replace("__TERRAIN__", &terrain_json)
        .replace("__META__", &meta_json)
        .replace("__FRAMES__", &frames_json)
}
