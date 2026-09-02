//! Genere `lives.html` : la premiere biographie auto-generee (0.0.3, tranche 2).
//!
//! Fichier unique, aucune dependance, s'ouvre par double-clic. Les vies d'agents sont
//! embarquees. Style editorial (comme `series.html`), sans emoji, sans cadratin. Aucun LLM :
//! la prose est faite de gabarits.

use genesis_core::persist::WorldMeta;
use genesis_core::SimConfig;

use crate::AgentLife;

const TEMPLATE: &str = include_str!("lives_template.html");
const COMMON_JS: &str = include_str!("reader_common.js");

/// `all` est deja trie (vies les plus longues d'abord). Seules les `embed` premieres sont
/// serialisees dans la page ; le reste ne sert qu'aux totaux (issues, plus longue vie).
pub fn render(
    name: &str,
    meta: &WorldMeta,
    cfg: &SimConfig,
    all: &[AgentLife],
    embed: usize,
    feature: usize,
) -> String {
    let head = &all[..all.len().min(embed)];
    let lives_json = serde_json::to_string(head)
        .unwrap_or_else(|_| "[]".to_string())
        .replace("</", "<\\/");

    let (mut mort, mut sommeil, mut vivant) = (0usize, 0usize, 0usize);
    for l in all {
        match l.ended {
            "mort" => mort += 1,
            "sommeil" => sommeil += 1,
            _ => vivant += 1,
        }
    }
    let longest = all
        .iter()
        .map(|l| l.ended_tick.unwrap_or(meta.ticks_played).saturating_sub(l.awoke_tick))
        .max()
        .unwrap_or(0);

    let meta_json = serde_json::json!({
        "seed": meta.seed,
        "engine_version": meta.engine_version,
        "ticks_played": meta.ticks_played,
        "tick_seconds": cfg.time.tick_duration_seconds,
        "grid": [cfg.world.grid_width, cfg.world.grid_height],
        "energy_threshold": cfg.reproduction.energy_threshold,
        "feature": feature,
        "awoke_total": all.len(),
        "outcomes": { "mort": mort, "sommeil": sommeil, "vivant": vivant },
        "longest_span": longest,
    })
    .to_string()
    .replace("</", "<\\/");

    TEMPLATE
        .replace("__COMMON_JS__", COMMON_JS)
        .replace("__WNAME__", name)
        .replace("__SEED__", &meta.seed.to_string())
        .replace("__ENGINE__", &meta.engine_version)
        .replace("__TICKS__", &meta.ticks_played.to_string())
        .replace("__AWOKE__", &all.len().to_string())
        .replace("__META__", &meta_json)
        .replace("__LIVES__", &lives_json)
}
