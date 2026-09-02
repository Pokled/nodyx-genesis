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

mod index_html;
mod lives_html;
mod series_html;
mod view_html;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use genesis_core::event::{Event, EventKind};
use genesis_core::{tick, EntityId, Memory, SimConfig, WorldDir, WorldState};
use genesis_core::persist::WorldMeta;
use genesis_view::{project, series_row, SeriesRow, ViewFrame};

/// Un souvenir compact, tel qu'il etait a un instant de la vie de l'agent.
#[derive(serde::Serialize, Clone)]
struct MemSnap {
    x: f32,
    y: f32,
    /// genre : 0 = peril (famine propre), 1 = aubaine, 2 = mort vue (ancree).
    k: u8,
    /// force du souvenir, (0, 1].
    s: f32,
    /// `seq` de l'evenement d'origine, pour les souvenirs ancres (mort vue).
    #[serde(skip_serializing_if = "Option::is_none")]
    e: Option<u64>,
}

fn mem_kind_code(k: genesis_core::MemoryKind) -> u8 {
    match k {
        genesis_core::MemoryKind::Peril => 0,
        genesis_core::MemoryKind::Bounty => 1,
        genesis_core::MemoryKind::Witnessed => 2,
    }
}

/// Un temps de vie d'agent (0.0.3, tranche 2) : sa position, son energie et sa memoire,
/// echantillonnes toutes les `BIO_SAMPLE` ticks. Le materiau des cartes de la page
/// biographie.
#[derive(serde::Serialize, Clone)]
struct LifeBeat {
    tick: u64,
    pos: [f32; 2],
    /// energie en pourcents du plafond, 0..100.
    energy: u8,
    /// sante en pourcents, 0..100 (biologie de fond, tranche 8).
    h: u8,
    /// jauges en pourcents : [faim, peur, solitude].
    n: [u8; 3],
    /// mode de comportement choisi : forage | flee | join | seek_bounty | wander.
    md: &'static str,
    /// la memoire episodique a cet instant (vide un temps sur deux, pour borner le poids).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mem: Vec<MemSnap>,
}

/// Vie d'un agent (0.0.3). Une ligne de `lives.jsonl` : quand il s'est eveille, comment sa
/// vie d'agent s'est terminee, sa memoire de fin, sa trajectoire (`beats`) et les evenements
/// du monde qui le nomment. Les `beats` et `events` ne sont gardes en detail que pour les
/// vies mises en vedette (voir `BIO_KEEP_DETAIL`).
#[derive(serde::Serialize)]
struct AgentLife {
    id: EntityId,
    lineage: u16,
    /// nom prononcable de la lignee fondatrice, pour la prose.
    lineage_name: String,
    generation: u32,
    perception: f32,
    lifespan_trait: f32,
    speed_trait: f32,
    awoke_tick: u64,
    awoke_place: [f32; 2],
    ended_tick: Option<u64>,
    /// "vivant" | "mort" | "sommeil"
    ended: &'static str,
    /// sante au dernier echantillon, 0..100 (biologie de fond, tranche 8).
    ended_health: u8,
    memories: Vec<Memory>,
    /// Les relations sociales de fin de vie : (id de l'autre, familiarite, valence).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    ties: Vec<(EntityId, f32, f32)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    beats: Vec<LifeBeat>,
    /// (tick, genre) des evenements objectifs qui nomment cet agent.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    events: Vec<(u64, &'static str)>,
}

/// Toutes les `BIO_SAMPLE` ticks, on note un temps de vie pour chaque agent suivi.
const BIO_SAMPLE: u64 = 150;
/// Anneau borne de temps de vie par agent (les plus recents).
const BIO_MAX_BEATS: usize = 64;
/// Nombre de vies gardees avec leur trajectoire complete (les plus longues, plus celles qui
/// ont le plus retenu). Les autres gardent la ligne resumee.
const BIO_KEEP_DETAIL: usize = 80;
/// Nombre de vies embarquees dans `lives.html` (les plus longues ; le reste ne sert qu'aux
/// totaux). Borne le poids de la page.
const BIO_EMBED: usize = 300;
/// Nombre de vies mises en chapitre dans `lives.html`.
const BIO_FEATURE: usize = 24;

/// Ajoute un evenement objectif a une vie d'agent (borne a 16).
fn push_life_event(l: &mut AgentLife, tick: u64, kind: &'static str) {
    if l.events.len() < 16 {
        l.events.push((tick, kind));
    }
}

/// Amorce une vie d'agent depuis l'entite au moment de l'eveil.
fn new_life(e: &genesis_core::Entity, awoke_tick: u64) -> AgentLife {
    AgentLife {
        id: e.id,
        lineage: e.genome.lineage,
        lineage_name: genesis_core::names::lineage_name(e.genome.lineage),
        generation: e.genome.generation,
        perception: e.genome.traits.perception,
        lifespan_trait: e.genome.traits.lifespan,
        speed_trait: e.genome.traits.speed,
        awoke_tick,
        awoke_place: [e.position.x, e.position.y],
        ended_tick: None,
        ended: "vivant",
        ended_health: (e.health.clamp(0.0, 1.0) * 100.0) as u8,
        memories: Vec::new(),
        ties: Vec::new(),
        beats: Vec::new(),
        events: Vec::new(),
    }
}

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
    // Vies d'agents (0.0.3) : suivies via le journal, figees a la mort ou a la retombee.
    let mut lives: std::collections::HashMap<EntityId, AgentLife> = std::collections::HashMap::new();

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

        for e in ev.iter() {
            match &e.kind {
                EventKind::AgentAwoke { entity } => {
                    if let Some(x) = world.get(*entity) {
                        lives.entry(*entity).or_insert_with(|| new_life(x, world.tick));
                    }
                }
                EventKind::AgentLapsed { entity } => {
                    if let Some(l) = lives.get_mut(entity) {
                        if l.ended_tick.is_none() {
                            l.ended_tick = Some(world.tick);
                            l.ended = "sommeil";
                        }
                    }
                }
                EventKind::EntityDied { entity, .. } => {
                    if let Some(l) = lives.get_mut(entity) {
                        push_life_event(l, world.tick, "mort");
                        if l.ended_tick.is_none() {
                            l.ended_tick = Some(world.tick);
                            l.ended = "mort";
                        }
                    }
                }
                EventKind::EntitySpawned { entity } => {
                    if let Some(l) = lives.get_mut(entity) {
                        push_life_event(l, world.tick, "naissance");
                    }
                }
                EventKind::EntityDivided { parent, child } => {
                    if let Some(l) = lives.get_mut(parent) {
                        push_life_event(l, world.tick, "division");
                    }
                    if let Some(l) = lives.get_mut(child) {
                        push_life_event(l, world.tick, "naissance");
                    }
                }
                _ => {}
            }
        }

        // Echantillon de vie : position, energie, memoire de chaque agent suivi.
        if world.tick % BIO_SAMPLE == 0 {
            let ceiling = cfg.reproduction.energy_threshold * 2.0;
            for l in lives.values_mut() {
                if l.ended_tick.is_some() {
                    continue;
                }
                if let Some(x) = world.get(l.id) {
                    let snap: Vec<MemSnap> = x.mind.as_deref().map_or_else(Vec::new, |m| {
                        m.episodic
                            .iter()
                            .map(|s| MemSnap {
                                x: s.place.x,
                                y: s.place.y,
                                k: mem_kind_code(s.kind),
                                s: s.strength,
                                e: s.event_seq,
                            })
                            .collect()
                    });
                    // Un echantillon sur deux porte le detail memoire, pour ne pas gonfler
                    // lives.jsonl : la parite suit le compte de battements de cette vie.
                    let with_mem = l.beats.len() % 2 == 0;
                    let pct = |v: f32| (v.clamp(0.0, 1.0) * 100.0) as u8;
                    let (n, md) = x.mind.as_deref().map_or(([0, 0, 0], "forage"), |m| {
                        (
                            [pct(m.needs.hunger), pct(m.needs.fear), pct(m.needs.solitude)],
                            m.mode.as_str(),
                        )
                    });
                    l.ended_health = pct(x.health);
                    l.beats.push(LifeBeat {
                        tick: world.tick,
                        pos: [x.position.x, x.position.y],
                        energy: ((x.energy / ceiling).clamp(0.0, 1.0) * 100.0) as u8,
                        h: pct(x.health),
                        n,
                        md,
                        mem: if with_mem { snap } else { Vec::new() },
                    });
                    if l.beats.len() > BIO_MAX_BEATS {
                        // On garde la vie sur toute sa longueur : on retire une paire de
                        // battements sur deux (donc un battement avec detail memoire et un
                        // sans, dans chaque paire conservee). La resolution s'adapte a la duree.
                        let mut k = 0usize;
                        l.beats.retain(|_| {
                            let keep = k % 4 < 2;
                            k += 1;
                            keep
                        });
                    }
                    if let Some(m) = x.mind.as_deref() {
                        l.memories = m.episodic.clone();
                        l.ties = m
                            .social
                            .iter()
                            .map(|s| (s.other, s.familiarity, s.valence))
                            .collect();
                    }
                }
            }
        }

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

    // Agents encore vivants au terme : on capture leur memoire de fin.
    for e in world.entities.iter() {
        if let Some(m) = &e.mind {
            let l = lives.entry(e.id).or_insert_with(|| new_life(e, m.awoke_tick));
            l.memories = m.episodic.clone();
            l.ties = m
                .social
                .iter()
                .map(|s| (s.other, s.familiarity, s.valence))
                .collect();
        }
    }
    // Duree comme agent, pour trier les vies : les plus longues d'abord.
    let agent_span = |l: &AgentLife| l.ended_tick.unwrap_or(world.tick).saturating_sub(l.awoke_tick);
    let mut lives_vec: Vec<AgentLife> = lives.into_values().collect();
    lives_vec.sort_by(|a, b| {
        agent_span(b)
            .cmp(&agent_span(a))
            .then(b.memories.len().cmp(&a.memories.len()))
            .then(a.id.cmp(&b.id))
    });
    // Au-dela des vies gardees en detail, on ne conserve que la ligne resumee.
    for l in lives_vec.iter_mut().skip(BIO_KEEP_DETAIL) {
        l.beats.clear();
        l.events.clear();
    }
    // Pour les chapitres, on remonte en tete les vies dont la memoire a le plus pese : une
    // mort vue (souvenir ancre) d'abord, puis la plus grande memoire atteinte, puis la
    // memoire de fin, puis la duree d'agent. Le reste garde l'ordre par duree.
    let peak_mem = |l: &AgentLife| l.beats.iter().map(|b| b.mem.len()).max().unwrap_or(0);
    let saw_death = |l: &AgentLife| {
        l.memories.iter().any(|m| matches!(m.kind, genesis_core::MemoryKind::Witnessed))
            || l.beats.iter().any(|b| b.mem.iter().any(|s| s.k == 2))
    };
    let head = lives_vec.len().min(BIO_KEEP_DETAIL);
    lives_vec[..head].sort_by(|a, b| {
        saw_death(b)
            .cmp(&saw_death(a))
            .then(peak_mem(b).cmp(&peak_mem(a)))
            .then(b.memories.len().cmp(&a.memories.len()))
            .then(agent_span(b).cmp(&agent_span(a)))
    });

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
    write_jsonl(&wdir.root.join("lives.jsonl"), &lives_vec)?;
    let html = view_html::render(&meta, &cfg, &frames);
    std::fs::write(wdir.root.join("view.html"), html)?;
    let series_html = series_html::render(&meta, &cfg, &series);
    std::fs::write(wdir.root.join("series.html"), series_html)?;
    let lives_html = lives_html::render(&meta, &cfg, &lives_vec, BIO_EMBED, BIO_FEATURE);
    std::fs::write(wdir.root.join("lives.html"), lives_html)?;

    // --- Page de garde du monde ---
    {
        use genesis_core::names;
        let sec_year = (cfg.time.tick_duration_seconds as f64) / (3600.0 * 24.0 * 365.0);
        let years = (world.tick as f64 * sec_year) as u64;
        let (agents_alive, mean_mem) = world.agent_stats();
        // La plus longue vie d'agent (lives_vec est re-trie pour les chapitres, pas par duree).
        let longest = lives_vec.iter().max_by_key(|l| agent_span(l));
        let mut species: Vec<String> = Vec::new();
        let mut extinct: Vec<String> = Vec::new();
        let mut chronicle: Vec<(u64, String)> = Vec::new();
        for e in notable.iter() {
            match &e.kind {
                EventKind::WorldCreated => chronicle.push((e.tick, "Genese du monde".to_string())),
                EventKind::SpeciesEmerged { species: s, size } => {
                    let n = names::species_name(*s);
                    chronicle.push((e.tick, format!("{n} apparait, {size} individus")));
                    species.push(n);
                }
                EventKind::LineageExtinct { lineage } => {
                    let n = names::lineage_name(*lineage);
                    chronicle.push((e.tick, format!("La lignee {n} s'eteint")));
                    extinct.push(n);
                }
                EventKind::PopulationCrash { from, to } => {
                    chronicle.push((e.tick, format!("Effondrement : {from} vers {to}")));
                }
                EventKind::PopulationMilestone { level } if *level >= 100 => {
                    chronicle.push((e.tick, format!("La population franchit {level}")));
                }
                _ => {}
            }
        }
        let digest = index_html::Digest {
            seed,
            engine: genesis_core::ENGINE_VERSION.to_string(),
            ticks: world.tick,
            years,
            pop_final: world.population(),
            carrying_capacity: series.last().map(|r| r.carrying_capacity).unwrap_or(0),
            births: world.births_total,
            deaths_starv: world.deaths_starvation,
            deaths_age: world.deaths_age,
            max_gen: series.last().map(|r| r.max_generation).unwrap_or(0),
            div_start: series.first().map(|r| r.genetic_diversity).unwrap_or(0.0),
            div_end: series.last().map(|r| r.genetic_diversity).unwrap_or(0.0),
            agents_awoke: lives_vec.len(),
            agents_alive,
            mean_mem,
            longest_span: longest.map(agent_span).unwrap_or(0),
            longest_lineage: longest
                .map(|l| names::lineage_name(l.lineage))
                .unwrap_or_default(),
            dominant_lineage: names::lineage_name(world.dominant_lineage()),
            dominant_share: world.lineage_stats().1,
            species,
            extinct,
            pop_series: series.iter().map(|r| r.population).collect(),
            cap_series: series.iter().map(|r| r.carrying_capacity).collect(),
            tick_series: series.iter().map(|r| r.tick).collect(),
            chronicle,
        };
        std::fs::write(wdir.root.join("index.html"), index_html::render(&digest))?;
    }

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
    {
        let (agents_alive, mean_mem) = world.agent_stats();
        let awoke = lives_vec.len();
        println!("  agents eveilles   {awoke}");
        println!("  dont vivants      {agents_alive}");
        println!("  souvenirs (moy.)  {mean_mem:.1}");
    }
    println!();
    println!("Ouvre {}", wdir.root.join("index.html").display());
    println!("      {}", wdir.root.join("view.html").display());
    println!("      {}", wdir.root.join("series.html").display());
    println!("      {}", wdir.root.join("lives.html").display());
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
