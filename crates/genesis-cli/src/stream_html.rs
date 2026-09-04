//! `stream.html` : l'overlay du direct 24/24 (source navigateur pour OBS, 1920x1080).
//!
//! Coquille statique qui relit `scene.json` a chaque pas serveur (~1/5 s) et `live.json` plus
//! rarement, et anime les changements. A servir par `genesis serve --port`, ou a ouvrir en `file://`
//! (l'overlay tente `fetch`, et retombe sur les balises JSON embarquees a la generation).

const TEMPLATE: &str = include_str!("stream_template.html");

/// `live_json` et `scene_json` sont embarques comme repli : l'overlay s'affiche meme sans
/// serveur (en `file://`), et se met a jour par `fetch` quand il est servi.
pub fn render(name: &str, seed: u64, live_json: &str, scene_json: &str) -> String {
    TEMPLATE
        .replace("__WNAME__", name)
        .replace("__SEED__", &seed.to_string())
        .replace("__LIVE__", &live_json.replace("</", "<\\/"))
        .replace("__SCENE__", &scene_json.replace("</", "<\\/"))
}

