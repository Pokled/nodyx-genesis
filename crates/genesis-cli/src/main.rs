//! genesis, ligne de commande.
//!
//!   genesis run --seed <N> --ticks <T> [--out worlds/<nom>] [--config <fichier>] [--frame-every <n>]
//!   genesis replay <dossier-monde>
//!
//! `run` fait naitre un monde, le fait tourner, ecrit les instantanes et le journal,
//! puis genere `view.html` : le premier replay deterministe, a ouvrir dans un navigateur.
//!
//! `replay` rejoue le monde depuis sa graine et verifie qu'il retombe exactement sur le
//! meme etat final. C'est le moment public de 0.0.1.

mod series_html;
mod view_html;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use genesis_core::event::{Event, EventKind};
use genesis_core::{tick, SimConfig, WorldDir, WorldState};
use genesis_core::persist::WorldMeta;
use genesis_view::{project, series_row, SeriesRow, ViewFrame};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("{}", USAGE);
        return ExitCode::FAILURE;
    }
    let cmd = args[0].as_str();
    let flags = parse_flags(&args[1..]);

    let result = match cmd {
        "run" => cmd_run(&flags),
        "replay" => cmd_replay(&flags, args.get(1).cloned()),
        "-h" | "--help" | "help" => {
            println!("{}", USAGE);
            Ok(())
        }
        other => {
            eprintln!("commande inconnue : {other}\n\n{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "\
genesis, Nodyx Genesis 0.0.1

  genesis run --seed <N> --ticks <T> [--out worlds/<nom>] [--config <fichier.toml>] [--frame-every <n>]
      Fait naitre un monde et le fait tourner. Ecrit dans le dossier de sortie :
        config.toml, meta.json, snapshots/, events.jsonl, frames.jsonl, view.html

  genesis replay <dossier-monde>
      Rejoue le monde depuis sa graine et verifie l'etat final. Deterministe : OK ou DIFF.
";

// ---------------------------------------------------------------------------

fn cmd_run(flags: &HashMap<String, String>) -> std::io::Result<()> {
    let seed: u64 = flags
        .get("seed")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| err("--seed <N> est requis"))?;
    let ticks: u64 = flags
        .get("ticks")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| err("--ticks <T> est requis"))?;
    let frame_every: u64 = flags
        .get("frame-every")
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(12);
    let out: PathBuf = flags
        .get("out")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("worlds/w{seed}")));

    if WorldDir::exists(&out) {
        return Err(err(&format!(
            "le monde {} existe deja, choisis un autre --out ou supprime le dossier",
            out.display()
        )));
    }

    let cfg = match flags.get("config") {
        Some(p) => SimConfig::load(Path::new(p))?,
        None => SimConfig::default(),
    };
    let tps = cfg.time.target_ticks_per_real_second;

    let wdir = WorldDir::open_or_create(&out)?;
    wdir.write_config(&cfg)?;

    let mut world = WorldState::new(seed, &cfg);

    // Evenement fondateur et naissance des deux entites. Les `seq` 0..N sont poses ici a la
    // main (avant le premier tick) ; le compteur du monde reprend a la suite.
    let mut founding: Vec<Event> = vec![Event::now(0, EventKind::WorldCreated)];
    for id in world.entities.iter().map(|e| e.id).collect::<Vec<_>>() {
        founding.push(Event::now(0, EventKind::EntitySpawned { entity: id }));
    }
    for (n, e) in founding.iter_mut().enumerate() {
        e.seq = n as u64;
    }
    world.next_event_seq = founding.len() as u64;
    wdir.append_events(&founding)?;

    wdir.write_snapshot(&world)?;

    let mut frames: Vec<ViewFrame> = Vec::new();
    // La premiere frame porte les evenements fondateurs : la genese est un chapitre.
    frames.push(project(&world, &cfg, tps, &founding));

    // Serie temporelle de stats : une ligne au depart, puis tous les `series_every` ticks,
    // puis une a la fin. Le materiau du graphe d'evolution.
    let series_every = cfg.persistence.series_every.max(1);
    let mut series: Vec<SeriesRow> = vec![series_row(&world, &cfg)];

    let mut since_frame: Vec<Event> = Vec::new();
    let mut notable: Vec<Event> = Vec::new();
    let mut extinct_at: Option<u64> = None;

    // Seuil de saillance pour le journal des choses qui comptent (chapitres).
    const NOTABLE: u8 = 150;
    notable.extend(founding.iter().filter(|e| e.salience >= NOTABLE).cloned());

    // Fenetre "genese" : on echantillonne quatre fois plus fin sur les premiers ticks,
    // la ou deux entites deviennent une lignee. C'est court en temps-monde, ca merite
    // d'etre regarde au ralenti.
    let genesis_window: u64 = 4000.min(ticks);
    let dense_every: u64 = (frame_every / 4).max(1);

    for _ in 0..ticks {
        let ev = tick(&mut world, &cfg);
        since_frame.extend(ev.iter().cloned());
        wdir.append_events(&ev)?;
        notable.extend(ev.iter().filter(|e| e.salience >= NOTABLE).cloned());

        let interval = if world.tick <= genesis_window { dense_every } else { frame_every };
        if world.tick % interval == 0 {
            frames.push(project(&world, &cfg, tps, &since_frame));
            since_frame.clear();
        }
        if world.tick % cfg.persistence.snapshot_interval_ticks == 0 {
            wdir.write_snapshot(&world)?;
        }
        if world.tick % series_every == 0 {
            series.push(series_row(&world, &cfg));
        }
        if world.entities.is_empty() {
            extinct_at = Some(world.tick);
            break;
        }
    }

    // Instantane final exactement au dernier tick, pour que replay puisse comparer.
    wdir.write_snapshot(&world)?;
    frames.push(project(&world, &cfg, tps, &since_frame));
    if series.last().map_or(true, |r| r.tick != world.tick) {
        series.push(series_row(&world, &cfg));
    }

    let meta = WorldMeta {
        seed,
        engine_version: genesis_core::ENGINE_VERSION.to_string(),
        schema_version: genesis_core::SCHEMA_VERSION,
        ticks_played: world.tick,
        last_event_seq: world.next_event_seq,
    };
    wdir.write_meta(&meta)?;

    write_frames_jsonl(&wdir.root, &frames)?;
    write_jsonl(&wdir.root.join("notable.jsonl"), &notable)?;
    write_jsonl(&wdir.root.join("series.jsonl"), &series)?;
    let html = view_html::render(&meta, &cfg, &frames);
    std::fs::write(wdir.root.join("view.html"), html)?;
    let series_html = series_html::render(&meta, &cfg, &series);
    std::fs::write(wdir.root.join("series.html"), series_html)?;

    println!("Monde w{seed}");
    println!("  ticks joues       {}", world.tick);
    println!("  population finale  {}", world.population());
    println!("  naissances        {}", world.births_total);
    println!("  morts             {}", world.deaths_total);
    println!("  diversite genetique {:.3}", world.genetic_diversity());
    if let Some(t) = extinct_at {
        println!("  extinction au tick {t}");
    }
    println!("  frames            {}", frames.len());
    println!("  generation max    {}", series.last().map(|r| r.max_generation).unwrap_or(0));
    println!();
    println!("Ouvre {}", wdir.root.join("view.html").display());
    println!("      {}", wdir.root.join("series.html").display());
    genesis_core::profile_dump();
    Ok(())
}

// ---------------------------------------------------------------------------

fn cmd_replay(_flags: &HashMap<String, String>, positional: Option<String>) -> std::io::Result<()> {
    let dir = positional.ok_or_else(|| err("genesis replay <dossier-monde>"))?;
    let dir = PathBuf::from(dir);
    let wdir = WorldDir::open_or_create(&dir)?;
    let meta = wdir.read_meta()?;
    let cfg = wdir.read_config()?;

    let recorded = wdir
        .latest_snapshot()?
        .ok_or_else(|| err("aucun instantane a comparer"))?;
    let recorded_json = serde_json::to_string(&recorded).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

    let mut world = WorldState::new(meta.seed, &cfg);
    // Les evenements fondateurs (WorldCreated + un EntitySpawned par entite initiale) portent
    // les `seq` 0..N ; `run` avance le compteur d'autant avant le premier tick, on refait
    // pareil ici pour que la numerotation rejouee colle.
    world.next_event_seq = 1 + world.entities.len() as u64;
    for _ in 0..meta.ticks_played {
        let _ = tick(&mut world, &cfg);
        if world.entities.is_empty() {
            break;
        }
    }
    let replayed_json = serde_json::to_string(&world).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

    println!("Monde        {}", dir.display());
    println!("graine       {}", meta.seed);
    println!("moteur       enregistre {} / actuel {}", meta.engine_version, genesis_core::ENGINE_VERSION);
    println!("ticks        {}", meta.ticks_played);

    if recorded_json == replayed_json {
        println!();
        println!("deterministe : OK. Meme graine, meme monde, jusqu'au dernier tick.");
        Ok(())
    } else {
        println!();
        println!("deterministe : DIFF. L'etat final rejoue ne correspond pas a l'enregistre.");
        // aide au diagnostic : premiere position qui diverge
        let ra = recorded_json.as_bytes();
        let rb = replayed_json.as_bytes();
        let mut at = ra.len().min(rb.len());
        for i in 0..ra.len().min(rb.len()) {
            if ra[i] != rb[i] {
                at = i;
                break;
            }
        }
        let lo = at.saturating_sub(50);
        println!("  divergence vers l'octet {at}");
        if let (Some(x), Some(y)) = (
            recorded_json.get(lo..(at + 50).min(recorded_json.len())),
            replayed_json.get(lo..(at + 50).min(replayed_json.len())),
        ) {
            println!("  enregistre : ...{x}");
            println!("  rejoue     : ...{y}");
        }
        Err(err("replay non deterministe"))
    }
}

// ---------------------------------------------------------------------------

fn write_frames_jsonl(root: &Path, frames: &[ViewFrame]) -> std::io::Result<()> {
    write_jsonl(&root.join("frames.jsonl"), frames)
}

fn write_jsonl<T: serde::Serialize>(path: &Path, items: &[T]) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::File::create(path)?;
    for it in items {
        let line = serde_json::to_string(it)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        writeln!(f, "{}", line)?;
    }
    Ok(())
}

fn parse_flags(args: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if let Some(name) = a.strip_prefix("--") {
            let val = args.get(i + 1).cloned().unwrap_or_default();
            map.insert(name.to_string(), val);
            i += 2;
        } else {
            i += 1;
        }
    }
    map
}

fn err(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg.to_string())
}
