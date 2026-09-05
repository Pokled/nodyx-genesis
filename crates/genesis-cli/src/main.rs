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

mod atlas_serve;
mod gallery_html;
mod http_serve;
mod index_html;
mod live;
mod lives_html;
mod series_html;
mod stream_html;
mod view_html;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use genesis_core::event::{Event, EventKind};
use genesis_core::{tick, EntityId, Memory, SimConfig, WorldDir, WorldState};
use genesis_core::persist::WorldMeta;
use genesis_view::{project, series_row, SeriesRow, ViewFrame};

/// Un souvenir compact, tel qu'il etait a un instant de la vie de l'agent.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct MemSnap {
    x: f32,
    y: f32,
    /// genre : 0 = peril (famine propre), 1 = aubaine, 2 = mort vue (ancree).
    k: u8,
    /// force du souvenir, (0, 1].
    s: f32,
    /// `seq` de l'evenement d'origine, pour les souvenirs ancres (mort vue).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    e: Option<u64>,
}

fn mem_kind_code(k: genesis_core::MemoryKind) -> u8 {
    match k {
        genesis_core::MemoryKind::Peril => 0,
        genesis_core::MemoryKind::Bounty => 1,
        genesis_core::MemoryKind::Witnessed => 2,
    }
}

/// Issue d'une vie d'agent. Serialise en `"vivant"` / `"mort"` / `"sommeil"` (le lecteur
/// compare ces chaines).
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum LifeEnd {
    Vivant,
    Mort,
    Sommeil,
}

/// Genre d'evenement objectif qui nomme un agent, pour sa biographie.
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum LifeEventKind {
    Naissance,
    Mort,
    Division,
}

/// Un temps de vie d'agent (0.0.3, tranche 2) : sa position, son energie et sa memoire,
/// echantillonnes toutes les `BIO_SAMPLE` ticks. Le materiau des cartes de la page
/// biographie. `Deserialize` pour que `genesis continue` reprenne les vies en cours.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct LifeBeat {
    tick: u64,
    pos: [f32; 2],
    /// energie en pourcents du plafond, 0..100.
    energy: u8,
    /// sante en pourcents, 0..100 (biologie de fond, tranche 8).
    h: u8,
    /// jauges en pourcents : [faim, peur, solitude].
    n: [u8; 3],
    /// mode de comportement choisi (serialise en forage | flee | join | seek_bounty | wander).
    md: genesis_core::BehaviorMode,
    /// la memoire episodique a cet instant (vide un temps sur deux, pour borner le poids).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    mem: Vec<MemSnap>,
}

/// Vie d'un agent (0.0.3). Une ligne de `lives.jsonl` : quand il s'est eveille, comment sa
/// vie d'agent s'est terminee, sa memoire de fin, sa trajectoire (`beats`) et les evenements
/// du monde qui le nomment. Les `beats` et `events` ne sont gardes en detail que pour les
/// vies mises en vedette (voir `BIO_KEEP_DETAIL`).
#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
    #[serde(default)]
    ended_tick: Option<u64>,
    ended: LifeEnd,
    /// sante au dernier echantillon, 0..100 (biologie de fond, tranche 8).
    #[serde(default)]
    ended_health: u8,
    #[serde(default)]
    memories: Vec<Memory>,
    /// Les relations sociales de fin de vie : (id de l'autre, familiarite, valence).
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    ties: Vec<(EntityId, f32, f32)>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    beats: Vec<LifeBeat>,
    /// (tick, genre) des evenements objectifs qui nomment cet agent.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    events: Vec<(u64, LifeEventKind)>,
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
fn push_life_event(l: &mut AgentLife, tick: u64, kind: LifeEventKind) {
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
        ended: LifeEnd::Vivant,
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
        "continue" => cmd_continue(&flags, args.get(1).cloned()),
        "serve" => cmd_serve(&flags, args.get(1).cloned()),
        "replay" => cmd_replay(&flags, args.get(1).cloned()),
        "gallery" => cmd_gallery(args.get(1).cloned().unwrap_or_else(|| "worlds".to_string())),
        "atlas" => cmd_atlas(&flags),
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

  genesis continue <dossier-monde> --ticks <T> [--frame-every <n>]
      Reprend un monde depuis son dernier instantane et le fait avancer de T ticks.
      Le journal et la serie temporelle continuent ; la scene montre le monde recent.

  genesis serve <dossier-monde> [--rate <ticks/s>] [--port <P>] [--restart] [--max-years <Y>]
      Fait tourner un monde en continu, en avancant par petits pas paces (--rate,
      defaut 30 ticks/s reels ; 0 = a fond). Avec --port, sert le monde en http :
      l'overlay du direct est sur http://localhost:<P>/stream.html (source OBS).
      Avec --restart, une nouvelle graine repart ici quand le monde meurt (ou passe
      --max-years annees-monde) ; records.json se transmet. Ctrl-C pour arreter.

  genesis replay <dossier-monde>
      Rejoue le monde depuis sa graine et verifie l'etat final. Deterministe : OK ou DIFF.

  genesis gallery [dossier]
      Reconstruit <dossier>/index.html : la grille de tous les mondes du dossier
      (defaut : worlds). Reconstruite aussi a chaque run.

  genesis atlas [--dir <dossier>] [--port <P>]
      Sert l'atlas du projet : l'arbre d'evolution (statuts, nœuds a inserer entre deux
      etapes), une todo, et une boite a idees (notes + fichiers deposes, pdf/html/md/xlsx/
      txt/xml...). Local, jamais publie. Defaut : --dir BIBLE/atlas --port 8090.
      Ouvre http://localhost:<P>/ dans un navigateur. Ctrl-C pour arreter.
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
    birth_world(&out, seed, &cfg, ticks, frame_every, true)
}

/// Les journaux et instantanes derives : effaces quand un monde renait au meme endroit
/// (`serve --restart`). `records.json` n'est PAS dans la liste : les records se transmettent
/// de monde en monde, c'est l'histoire sportive de la graine.
const DERIVED_FILES: &[&str] = &[
    "events.jsonl",
    "frames.jsonl",
    "series.jsonl",
    "notable.jsonl",
    "lives.jsonl",
    "live.json",
    "scene.json",
    "meta.json",
];

/// Fait naitre un monde dans `root` (le dossier peut deja exister : on repart a neuf) et le
/// fait tourner `ticks` ticks. Partage par `cmd_run` et la renaissance de `serve --restart`.
fn birth_world(
    root: &Path,
    seed: u64,
    cfg: &SimConfig,
    ticks: u64,
    frame_every: u64,
    verbose: bool,
) -> std::io::Result<()> {
    let tps = cfg.time.target_ticks_per_real_second;
    let wdir = WorldDir::open_or_create(root)?;
    wdir.write_config(cfg)?;

    // Table rase des derives d'un eventuel monde precedent (sauf records.json).
    for f in DERIVED_FILES {
        let _ = std::fs::remove_file(wdir.root.join(f));
    }
    if let Ok(entries) = std::fs::read_dir(wdir.root.join("snapshots")) {
        for e in entries.flatten() {
            let _ = std::fs::remove_file(e.path());
        }
    }

    let mut world = WorldState::new(seed, cfg);

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

    let mut sim = Sim::new();
    // La premiere frame porte les evenements fondateurs : la genese est un chapitre.
    sim.frames.push(project(&world, cfg, tps, &founding));
    sim.series.push(series_row(&world, cfg));
    sim.notable
        .extend(founding.iter().filter(|e| is_chronicle_event(&e.kind)).cloned());

    // Fenetre "genese" : on echantillonne quatre fois plus fin sur les premiers ticks, la ou
    // deux entites deviennent une lignee. En `run` on garde toutes les frames.
    sim.run(&mut world, cfg, &wdir, ticks, frame_every, 4000.min(ticks), usize::MAX, true)?;

    write_world_pages(
        &world,
        cfg,
        &wdir,
        seed,
        sim.awoke_count,
        &sim.frames,
        &sim.series,
        &sim.notable,
        &sim.lives,
        sim.extinct_at,
        verbose,
    )
}

/// `true` si l'evenement merite une ligne dans la chronique du monde (`notable.jsonl`, la
/// page de garde). Les eveils d'agents et les cellules sont trop frequents : ils vivent dans
/// le fil de `view.html`, pas dans la chronique.
fn is_chronicle_event(kind: &EventKind) -> bool {
    kind.is_chapter()
}

/// `true` si l'evenement va au journal append-only (`events.jsonl`). On laisse tomber les
/// `ReplicationFailed` : au plateau de capacite c'est l'ecrasante majorite du volume et ca
/// ne dit rien de plus que la stat `repro_blocked_materials`. La simulation ne relit jamais
/// le journal, donc le monde est identique ; seul l'artefact `events.jsonl` maigrit.
fn journal_worthy(e: &Event) -> bool {
    !matches!(e.kind, EventKind::ReplicationFailed { .. })
}

/// Les accumulateurs d'un segment de simulation : ce qu'on garde en memoire pour engendrer
/// les pages a la fin. Partage par `run` (depart) et `continue` (reprise).
struct Sim {
    frames: Vec<ViewFrame>,
    series: Vec<SeriesRow>,
    notable: Vec<Event>,
    lives: std::collections::HashMap<EntityId, AgentLife>,
    /// Eveils comptes sur ce segment. Un compteur, pas `lives.len()` : `serve` oublie les
    /// vieilles biographies pour ne pas gonfler sans fin, `lives.len()` sous-estimerait.
    awoke_count: u64,
    extinct_at: Option<u64>,
    /// Vrai quand la derniere frame poussee est une frame "provisoire" de fin de pas `serve`
    /// (hors frontiere `frame_every`). Le pas suivant la remplace au lieu de l'empiler : sinon
    /// un `serve` a petit pas ferait grossir `frames` cinq fois plus vite et la fenetre
    /// d'historique de `view.html` fondrait d'autant.
    last_frame_provisional: bool,
}

impl Sim {
    fn new() -> Self {
        Sim {
            frames: Vec::new(),
            series: Vec::new(),
            notable: Vec::new(),
            lives: std::collections::HashMap::new(),
            awoke_count: 0,
            extinct_at: None,
            last_frame_provisional: false,
        }
    }

    /// Oublie les biographies terminees les moins longues au-dela de `keep`. Les vies en
    /// cours sont toujours gardees. Pour qu'un `serve` sans fin ne fuie pas la memoire.
    fn prune_ended_lives(&mut self, now: u64, keep: usize) {
        let span =
            |l: &AgentLife| l.ended_tick.unwrap_or(now).saturating_sub(l.awoke_tick);
        let mut ended: Vec<(EntityId, u64)> = self
            .lives
            .iter()
            .filter(|(_, l)| l.ended_tick.is_some())
            .map(|(id, l)| (*id, span(l)))
            .collect();
        if ended.len() <= keep {
            return;
        }
        ended.sort_by_key(|(_, s)| std::cmp::Reverse(*s));
        for (id, _) in ended.into_iter().skip(keep) {
            self.lives.remove(&id);
        }
    }

    /// Borne `series` : recent au pas plein, ancien decime. Un monde 24/24 ne peut pas
    /// garder chaque ligne pour toujours ; l'overlay et la page de garde reechantillonnent.
    fn thin_series(&mut self, max: usize) {
        if self.series.len() <= max {
            return;
        }
        let cut = self.series.len() / 4; // on decime le quart le plus ancien
        let mut i = 0;
        self.series.retain(|_| {
            let keep = i >= cut || i % 4 == 0;
            i += 1;
            keep
        });
    }

    /// Fait tourner `ticks` ticks, en appendant au journal et en ecrivant les instantanes.
    /// `max_frames` borne le nombre de frames gardees (une fenetre glissante pour `continue`,
    /// `usize::MAX` pour `run`). `checkpoint` : ecrire un instantane exactement au dernier tick
    /// (vrai pour `run` / `continue` ; faux quand `serve` avance par petits pas, ou l'on ne
    /// veut pas un fichier d'instantane toutes les secondes, seulement ceux de l'intervalle).
    #[allow(clippy::too_many_arguments)]
    fn run(
        &mut self,
        world: &mut WorldState,
        cfg: &SimConfig,
        wdir: &WorldDir,
        ticks: u64,
        frame_every: u64,
        genesis_window: u64,
        max_frames: usize,
        checkpoint: bool,
    ) -> std::io::Result<()> {
        let tps = cfg.time.target_ticks_per_real_second;
        let series_every = cfg.persistence.series_every.max(1);
        let dense_every: u64 = (frame_every / 4).max(1);
        let mut since_frame: Vec<Event> = Vec::new();

        for _ in 0..ticks {
            let ev = tick(world, cfg);
            // Biographies et chronique : lisent tous les evenements.
            self.notable
                .extend(ev.iter().filter(|e| is_chronicle_event(&e.kind)).cloned());
            // Journal et frames : sans le bruit de plateau (`ReplicationFailed` = ~90 % du
            // volume au plafond de capacite, deja compte dans `repro_blocked_materials` ;
            // la simulation ne relit jamais le journal, le monde reste identique).
            let journal: Vec<Event> = ev.iter().filter(|e| journal_worthy(e)).cloned().collect();
            since_frame.extend(journal.iter().cloned());
            wdir.append_events(&journal)?;

            for e in ev.iter() {
                match &e.kind {
                    EventKind::AgentAwoke { entity } => {
                        if !self.lives.contains_key(entity) {
                            if let Some(x) = world.get(*entity) {
                                self.lives.insert(*entity, new_life(x, world.tick));
                                self.awoke_count += 1;
                            }
                        }
                    }
                    EventKind::AgentLapsed { entity } => {
                        if let Some(l) = self.lives.get_mut(entity) {
                            if l.ended_tick.is_none() {
                                l.ended_tick = Some(world.tick);
                                l.ended = LifeEnd::Sommeil;
                            }
                        }
                    }
                    EventKind::EntityDied { entity, .. } => {
                        if let Some(l) = self.lives.get_mut(entity) {
                            push_life_event(l, world.tick, LifeEventKind::Mort);
                            if l.ended_tick.is_none() {
                                l.ended_tick = Some(world.tick);
                                l.ended = LifeEnd::Mort;
                            }
                        }
                    }
                    EventKind::EntitySpawned { entity } => {
                        if let Some(l) = self.lives.get_mut(entity) {
                            push_life_event(l, world.tick, LifeEventKind::Naissance);
                        }
                    }
                    EventKind::EntityDivided { parent, child } => {
                        if let Some(l) = self.lives.get_mut(parent) {
                            push_life_event(l, world.tick, LifeEventKind::Division);
                        }
                        if let Some(l) = self.lives.get_mut(child) {
                            push_life_event(l, world.tick, LifeEventKind::Naissance);
                        }
                    }
                    _ => {}
                }
            }

            // Echantillon de vie : position, energie, memoire de chaque agent suivi.
            if world.tick % BIO_SAMPLE == 0 {
                let ceiling = cfg.reproduction.energy_threshold * 2.0;
                for l in self.lives.values_mut() {
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
                    let (n, md) =
                        x.mind.as_deref().map_or(([0, 0, 0], genesis_core::BehaviorMode::Forage), |m| {
                            (
                                [pct(m.needs.hunger), pct(m.needs.fear), pct(m.needs.solitude)],
                                m.mode,
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
                self.frames.push(project(world, cfg, tps, &since_frame));
                since_frame.clear();
                self.last_frame_provisional = false;
                while self.frames.len() > max_frames {
                    self.frames.remove(0);
                }
            }
            if world.tick % cfg.persistence.snapshot_interval_ticks == 0 {
                wdir.write_snapshot(world)?;
                wdir.prune_snapshots(SNAPSHOTS_KEPT)?;
            }
            if world.tick % series_every == 0 {
                self.series.push(series_row(world, cfg));
                self.thin_series(SERIES_MAX);
            }
            if world.entities.is_empty() {
                self.extinct_at = Some(world.tick);
                break;
            }
        }

        // Instantane final exactement au dernier tick, pour que replay puisse comparer.
        // En pas-a-pas (`serve`), on s'en remet aux instantanes de l'intervalle.
        if checkpoint {
            wdir.write_snapshot(world)?;
            wdir.prune_snapshots(SNAPSHOTS_KEPT)?;
        }
        // Frame de fin de pas.
        // - birth / replay (`checkpoint`) : il FAUT une frame pile au dernier tick pour la
        //   comparaison ; la boucle l'a peut-etre deja poussee sur une frontiere, ne pas
        //   dupliquer.
        // - `serve` (petits pas) : cette frame sert de scene "live" a chaque tour. On la marque
        //   provisoire et le tour suivant la remplace, sauf sur une frontiere `frame_every` ou
        //   la boucle vient de pousser la frame definitive. Sans ca, `frames` grossirait au
        //   rythme des pas et la fenetre d'historique de `view.html` fondrait d'autant.
        let on_boundary = frame_every > 0 && world.tick % frame_every == 0;
        if checkpoint {
            if self.frames.last().map_or(true, |f| f.tick != world.tick) {
                self.frames.push(project(world, cfg, tps, &since_frame));
            }
            self.last_frame_provisional = false;
        } else if !on_boundary {
            if self.last_frame_provisional {
                self.frames.pop();
            }
            self.frames.push(project(world, cfg, tps, &since_frame));
            self.last_frame_provisional = true;
        }
        while self.frames.len() > max_frames {
            self.frames.remove(0);
        }
        if self.series.last().map_or(true, |r| r.tick != world.tick) {
            self.series.push(series_row(world, cfg));
        }

        // Agents encore vivants au terme du segment : on capture leur memoire de fin.
        for e in world.entities.iter() {
            if let Some(m) = &e.mind {
                let l = self.lives.entry(e.id).or_insert_with(|| new_life(e, m.awoke_tick));
                l.memories = m.episodic.clone();
                l.ties = m
                    .social
                    .iter()
                    .map(|s| (s.other, s.familiarity, s.valence))
                    .collect();
            }
        }
        Ok(())
    }
}

/// Ecrit les cinq pages HTML et les journaux derives, puis la bibliotheque et le resume.
/// Partage par `run` et `continue`.
#[allow(clippy::too_many_arguments)]
fn write_world_pages(
    world: &WorldState,
    cfg: &SimConfig,
    wdir: &WorldDir,
    seed: u64,
    awoke_total: u64,
    frames: &[ViewFrame],
    series: &[SeriesRow],
    notable: &[Event],
    lives: &std::collections::HashMap<EntityId, AgentLife>,
    extinct_at: Option<u64>,
    verbose: bool,
) -> std::io::Result<()> {
    // Duree comme agent, pour trier les vies : les plus longues d'abord.
    let agent_span = |l: &AgentLife| l.ended_tick.unwrap_or(world.tick).saturating_sub(l.awoke_tick);
    let mut lives_vec: Vec<AgentLife> = lives.values().cloned().collect();
    lives_vec.sort_by(|a, b| {
        agent_span(b)
            .cmp(&agent_span(a))
            .then(b.memories.len().cmp(&a.memories.len()))
            .then(a.id.cmp(&b.id))
    });
    // Un monde qui ne s'arrete jamais ne peut pas garder toutes ses biographies : on borne
    // le fichier aux vies les plus longues. `awoke_total` (dans meta) garde le compte vrai.
    lives_vec.truncate(LIVES_FILE_CAP);
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

    // Un instantane exactement au tick courant : meta, journal et instantane restent
    // coherents, y compris quand `serve` avance par petits pas sans checkpoint. C'est aussi
    // ce que `replay` compare. (Pour `run`, c'est le meme fichier que celui deja ecrit.)
    wdir.write_snapshot(world)?;
    wdir.prune_snapshots(SNAPSHOTS_KEPT)?;

    let meta = WorldMeta {
        seed,
        engine_version: genesis_core::ENGINE_VERSION.to_string(),
        schema_version: genesis_core::SCHEMA_VERSION,
        ticks_played: world.tick,
        last_event_seq: world.next_event_seq,
        agents_awoke_total: awoke_total,
    };
    wdir.write_meta(&meta)?;

    // Nom d'affichage du monde : le dossier de sortie (ex. "w2"), pas la graine.
    let world_name = wdir
        .root
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("w{seed}"));

    write_frames_jsonl(&wdir.root, frames)?;
    write_jsonl(&wdir.root.join("notable.jsonl"), notable)?;
    write_jsonl(&wdir.root.join("series.jsonl"), series)?;
    write_jsonl(&wdir.root.join("lives.jsonl"), &lives_vec)?;
    let html = view_html::render(&world_name, &meta, cfg, frames);
    std::fs::write(wdir.root.join("view.html"), html)?;
    let series_html = series_html::render(&world_name, &meta, cfg, series);
    std::fs::write(wdir.root.join("series.html"), series_html)?;
    let lives_html = lives_html::render(&world_name, &meta, cfg, &lives_vec, BIO_EMBED, BIO_FEATURE);
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
                EventKind::GenomeShift { generation, .. } => {
                    chronicle.push((
                        e.tick,
                        format!("Le genome dominant bascule, generation {generation}"),
                    ));
                }
                _ => {}
            }
        }
        let digest = index_html::Digest {
            name: world_name.clone(),
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
            agents_awoke: awoke_total as usize,
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

    // L'overlay 24/24 : petit etat vivant, derniere image, records.
    let (live_json, scene_json) = live::write_live(
        wdir, world, cfg, seed, awoke_total, &world_name, frames, series, notable, &lives_vec,
    )?;
    let stream = stream_html::render(&world_name, seed, &live_json, &scene_json);
    std::fs::write(wdir.root.join("stream.html"), stream)?;

    // La bibliotheque : on reconstruit la page de garde du dossier parent (sans faille
    // fatale si ca echoue).
    if let Some(parent) = wdir.root.parent() {
        if let Err(e) = build_gallery(parent) {
            eprintln!("note : la bibliotheque n'a pas pu etre reconstruite ({e})");
        }
    }

    if verbose {
        println!("Monde {world_name}");
        println!("  ticks joues       {}", world.tick);
        println!("  population finale  {}", world.population());
        println!("  naissances        {}", world.births_total);
        println!("  morts             {}", world.deaths_total);
        println!("  diversite genetique {:.3}", world.genetic_diversity());
        if let Some(t) = extinct_at {
            println!("  extinction au tick {t}");
        }
        println!("  frames            {}", frames.len());
        println!(
            "  generation max    {}",
            series.last().map(|r| r.max_generation).unwrap_or(0)
        );
        let (agents_alive, mean_mem) = world.agent_stats();
        println!("  agents eveilles   {awoke_total}");
        println!("  dont vivants      {agents_alive}");
        println!("  souvenirs (moy.)  {mean_mem:.1}");
        println!();
        println!("Ouvre {}", wdir.root.join("index.html").display());
        println!("      {}", wdir.root.join("view.html").display());
        println!("      {}", wdir.root.join("series.html").display());
        println!("      {}", wdir.root.join("lives.html").display());
        genesis_core::profile_dump();
    }
    Ok(())
}

// ---------------------------------------------------------------------------

/// Nombre de vies d'agents deja terminees qu'on garde a la reprise (les plus longues). Les
/// autres sont oubliees des sorties : c'est le prix d'un monde qui ne s'arrete jamais.
const CONTINUE_LIVES_KEPT: usize = 200;
/// Plafond de vies ecrites dans `lives.jsonl` (les plus longues). Le compte vrai des eveils
/// est dans `meta.agents_awoke_total`.
const LIVES_FILE_CAP: usize = 800;
/// Fenetre de frames gardees a la reprise : la scene montre le monde recent, pas toute son
/// histoire. `series.html` et `index.html`, eux, gardent l'arc complet (bon marche).
const CONTINUE_FRAME_WINDOW: usize = 500;
/// Chapitres gardes a la reprise.
const CONTINUE_NOTABLE_KEPT: usize = 2500;
/// Instantanes gardes sur disque. Un monde 24/24 en ecrit un toutes les quelques minutes ;
/// la reprise et `replay` ne lisent que le dernier. ~12 x 5 Mo au plateau.
const SNAPSHOTS_KEPT: usize = 12;
/// Plafond de lignes de serie temporelle gardees en memoire (et dans `series.jsonl`). Au-dela,
/// le quart le plus ancien est decime : recent au pas plein, tres ancien esquisse.
const SERIES_MAX: usize = 4000;

/// `genesis continue <dossier> --ticks N` : reprend un monde depuis son dernier instantane et
/// le fait avancer. Le journal et la serie temporelle continuent de grossir (verite du
/// monde) ; la scene et les biographies sont refaites sur le segment recent.
fn cmd_continue(
    flags: &HashMap<String, String>,
    positional: Option<String>,
) -> std::io::Result<()> {
    let out = resume_path(flags, positional, "genesis continue <dossier-monde> --ticks N")?;
    let ticks: u64 = flags
        .get("ticks")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| err("--ticks <T> est requis"))?;
    let frame_every = resume_frame_every(flags);

    let mut r = Resume::load(&out)?;
    let from_tick = r.world.tick;
    r.sim.run(&mut r.world, &r.cfg, &r.wdir, ticks, frame_every, 0, CONTINUE_FRAME_WINDOW, true)?;

    println!(
        "Monde {} repris : {} -> {} ticks (+{})",
        out.display(),
        from_tick,
        r.world.tick,
        r.world.tick - from_tick
    );
    r.write_pages(true)
}

/// `genesis serve <dossier> [--chunk N] [--for T] [--rate ticks/s]` : fait tourner un monde
/// en continu, en refaisant les pages a chaque tranche. On ouvre `index.html` dans un
/// navigateur et on rafraichit pour voir le monde avancer. Ctrl-C arrete proprement.
fn cmd_serve(
    flags: &HashMap<String, String>,
    positional: Option<String>,
) -> std::io::Result<()> {
    let out = resume_path(flags, positional, "genesis serve <dossier-monde>")?;
    let frame_every = resume_frame_every(flags);
    let limit: Option<u64> = flags.get("for").and_then(|s| s.parse().ok());
    // Cadence voulue, en ticks par seconde reelle. Un monde qui tourne 24/24 se regarde :
    // le defaut est lent. `--rate 0` = a fond.
    let rate: f64 = flags.get("rate").and_then(|s| s.parse().ok()).unwrap_or(30.0);
    // Pas d'avancement : une FRACTION de seconde de temps-monde par tour de boucle. scene.json
    // est reecrit a chaque tour ; si le pas valait une seconde entiere (~45 ticks), l'overlay
    // recevrait les positions par bonds de 45 ticks et le lissage donnerait un "avance puis
    // se fige" a chaque seconde, la saccade du monde. En decoupant en ~1/5 s, la cible du
    // lissage bouge cinq fois plus souvent, par pas cinq fois plus petits : le monde glisse.
    // Le rythme reel (`rate` ticks/s) ne change pas, seule la granularite des ecritures.
    let step: u64 = flags
        .get("step")
        .or_else(|| flags.get("chunk"))
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or_else(|| if rate > 0.0 { (rate / 5.0).round().max(1.0) as u64 } else { 400 });
    // Les pages lourdes (view / series / lives / index) : au plus une fois toutes ces secondes.
    let pages_every: f64 = flags
        .get("pages-every")
        .and_then(|s| s.parse().ok())
        .unwrap_or(15.0);
    // Serveur de fichiers pour l'overlay du direct. 0 (defaut) = coupe.
    let port: u16 = flags.get("port").and_then(|s| s.parse().ok()).unwrap_or(0);
    // Renaissance : quand le monde meurt (ou passe --max-years annees-monde), une nouvelle
    // graine repart au meme endroit. records.json se transmet : chaque monde vise les records
    // du precedent.
    let restart = flags.contains_key("restart");
    let genesis_ticks: u64 = flags
        .get("genesis-ticks")
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(4000);
    let max_years: Option<u64> = flags.get("max-years").and_then(|s| s.parse().ok());

    let mut r = Resume::load(&out)?;
    let mut start = r.world.tick;
    println!(
        "Monde {} en marche depuis le tick {} ({} ticks/s reels). Ctrl-C pour arreter.",
        out.display(),
        start,
        if rate > 0.0 { format!("{rate:.0}") } else { "max".into() },
    );
    println!("Ouvre {}", r.wdir.root.join("index.html").display());
    if restart {
        println!("Renaissance activee : a la mort du monde, une nouvelle graine repart ici.");
    }
    if port > 0 {
        match http_serve::spawn(r.wdir.root.clone(), port) {
            Ok(p) => println!("Overlay du direct : http://localhost:{p}/stream.html"),
            Err(e) => eprintln!("serveur http non demarre ({e})"),
        }
    }

    let year_ticks = (365 * 86400 / r.cfg.time.tick_duration_seconds.max(1)).max(1);
    let pages_gap = std::time::Duration::from_secs_f64(pages_every.max(0.5));
    let mut last_pages = std::time::Instant::now() - pages_gap * 2;
    let mut last_beat = std::time::Instant::now() - std::time::Duration::from_secs(9);
    // Ticks avances sur toute la session, tous mondes confondus (la renaissance remet
    // `world.tick` a zero).
    let mut session_advanced = 0u64;
    loop {
        let t0 = std::time::Instant::now();
        let before = r.world.tick;
        r.sim
            .run(&mut r.world, &r.cfg, &r.wdir, step, frame_every, 0, CONTINUE_FRAME_WINDOW, false)?;
        session_advanced += r.world.tick.saturating_sub(before);

        // L'overlay : petit etat vivant + derniere image, a chaque pas (peu couteux).
        r.write_live_light()?;

        let extinct = r.world.entities.is_empty();
        let age = r.world.tick.saturating_sub(start);
        let old_age = max_years.is_some_and(|y| age >= y * year_ticks);
        let session_done = limit.is_some_and(|l| session_advanced >= l);

        if extinct || old_age || session_done || last_pages.elapsed() >= pages_gap {
            // Bornage memoire d'un monde qui ne s'arrete jamais.
            r.sim.prune_ended_lives(r.world.tick, CONTINUE_LIVES_KEPT);
            if r.sim.notable.len() > CONTINUE_NOTABLE_KEPT {
                let d = r.sim.notable.len() - CONTINUE_NOTABLE_KEPT;
                r.sim.notable.drain(0..d);
            }
            r.write_pages(false)?;

            // Journal en pyramide : `events.jsonl` ne garde que la fenetre recente, les
            // evenements de chapitre plus vieux roulent dans `events.chronicle.jsonl`. Sinon
            // le journal grossit sans fin sur un direct de plusieurs mois. Hysteresis : on ne
            // reecrit que lorsque la plus vieille ligne a une demi-fenetre de retard. On ne
            // compacte jamais au-dela du plus vieil instantane garde : a la reprise, le
            // journal doit couvrir depuis cet instantane.
            let jk = r.cfg.persistence.journal_keep_ticks;
            let snap_span = SNAPSHOTS_KEPT as u64 * r.cfg.persistence.snapshot_interval_ticks + 5000;
            let keep = jk.max(snap_span);
            if jk > 0 && r.world.tick > keep + keep / 2 {
                if let Err(e) = r
                    .wdir
                    .compact_journal(r.world.tick - keep, r.world.tick - keep - keep / 2)
                {
                    eprintln!("compaction du journal : {e}");
                }
            }

            last_pages = std::time::Instant::now();
        }

        if extinct || old_age || session_done || last_beat.elapsed().as_secs_f64() >= 5.0 {
            let (agents_alive, _) = r.world.agent_stats();
            println!(
                "  tick {:>10}  pop {:>5}  agents {:>5}  gen {:>3}",
                r.world.tick,
                r.world.population(),
                agents_alive,
                r.sim.series.last().map(|s| s.max_generation).unwrap_or(0),
            );
            last_beat = std::time::Instant::now();
        }

        if session_done {
            break;
        }
        if extinct || old_age {
            let why = if extinct {
                format!("s'est eteint au tick {}", r.world.tick)
            } else {
                format!("a vecu {} annees-monde", age / year_ticks)
            };
            if !restart {
                println!("Le monde {why}.");
                break;
            }
            let next_seed = r.seed.wrapping_add(1);
            println!("Le monde {why}. Une nouvelle graine ({next_seed}) repart ici.");
            let cfg = r.cfg.clone();
            drop(r);
            // Un petit repos entre deux vies : evite un tour serre si plusieurs graines
            // d'affilee donnent un monde qui meurt aussitot.
            std::thread::sleep(std::time::Duration::from_secs(2));
            birth_world(&out, next_seed, &cfg, genesis_ticks, frame_every, false)?;
            r = Resume::load(&out)?;
            start = r.world.tick;
            last_pages = std::time::Instant::now() - pages_gap * 2;
            continue;
        }

        // Cadence : on attend pour ne pas depasser `rate` ticks par seconde reelle.
        if rate > 0.0 {
            let target = (r.world.tick - before) as f64 / rate;
            let spent = t0.elapsed().as_secs_f64();
            if target > spent {
                std::thread::sleep(std::time::Duration::from_secs_f64(target - spent));
            }
        }
    }
    Ok(())
}

/// Un monde charge pour etre repris : etat, config, accumulateurs, et de quoi ecrire les
/// pages. Partage par `continue` et `serve`.
struct Resume {
    wdir: WorldDir,
    cfg: SimConfig,
    seed: u64,
    world: WorldState,
    sim: Sim,
    /// Compte d'eveils avant ce segment (pour `meta.agents_awoke_total`).
    awoke_before: u64,
}

impl Resume {
    fn load(out: &Path) -> std::io::Result<Self> {
        if !WorldDir::exists(out) {
            return Err(err(&format!(
                "le monde {} n'existe pas ; lance d'abord `genesis run`",
                out.display()
            )));
        }
        let wdir = WorldDir::open_or_create(out)?;
        let cfg = wdir.read_config()?;
        let meta = wdir.read_meta()?;
        let world = wdir
            .latest_snapshot()?
            .ok_or_else(|| err("aucun instantane : impossible de reprendre ce monde"))?;
        let from_tick = world.tick;

        // Robustesse : si une session precedente a ete tuee entre deux instantanes, le journal
        // contient des evenements posterieurs au dernier instantane. On les coupe pour ne pas
        // les ecrire deux fois quand on rejoue depuis cet instantane.
        truncate_events_after(&wdir.root.join("events.jsonl"), from_tick)?;

        let series: Vec<SeriesRow> = read_jsonl(&wdir.root.join("series.jsonl"));
        let mut notable: Vec<Event> = read_jsonl(&wdir.root.join("notable.jsonl"));
        notable.retain(|e| is_chronicle_event(&e.kind));
        let all_lives: Vec<AgentLife> = read_jsonl(&wdir.root.join("lives.jsonl"));
        let awoke_before = meta.agents_awoke_total.max(all_lives.len() as u64);

        if notable.len() > CONTINUE_NOTABLE_KEPT {
            let d = notable.len() - CONTINUE_NOTABLE_KEPT;
            notable.drain(0..d);
        }

        // Toutes les vies encore en cours, plus les plus longues parmi les terminees.
        let mut ended: Vec<AgentLife> = Vec::new();
        let mut lives: std::collections::HashMap<EntityId, AgentLife> =
            std::collections::HashMap::new();
        for l in all_lives {
            if l.ended_tick.is_none() {
                lives.insert(l.id, l);
            } else {
                ended.push(l);
            }
        }
        let span = |l: &AgentLife| l.ended_tick.unwrap_or(from_tick).saturating_sub(l.awoke_tick);
        ended.sort_by_key(|l| std::cmp::Reverse(span(l)));
        ended.truncate(CONTINUE_LIVES_KEPT);
        for l in ended {
            lives.insert(l.id, l);
        }

        let tps = cfg.time.target_ticks_per_real_second;
        let mut sim = Sim::new();
        sim.series = series;
        sim.notable = notable;
        sim.lives = lives;
        sim.frames.push(project(&world, &cfg, tps, &[]));
        if sim.series.last().map_or(true, |r| r.tick != world.tick) {
            sim.series.push(series_row(&world, &cfg));
        }

        Ok(Resume { wdir, cfg, seed: meta.seed, world, sim, awoke_before })
    }

    /// Nombre d'eveils sur ce segment (pour `meta.agents_awoke_total`). Un compteur, pas
    /// `lives.len()` : `serve` oublie les vieilles biographies au fil de l'eau.
    fn new_awoke(&self) -> u64 {
        self.sim.awoke_count
    }

    /// Le strict necessaire a l'overlay, entre deux pas de `serve` : `live.json`,
    /// `scene.json`, `records.json`. Peu couteux, appele en continu ; les pages HTML
    /// lourdes restent l'affaire de `write_pages`.
    fn write_live_light(&self) -> std::io::Result<()> {
        let span =
            |l: &AgentLife| l.ended_tick.unwrap_or(self.world.tick).saturating_sub(l.awoke_tick);
        let mut lives_vec: Vec<AgentLife> = self.sim.lives.values().cloned().collect();
        lives_vec.sort_by(|a, b| span(b).cmp(&span(a)).then(a.id.cmp(&b.id)));
        lives_vec.truncate(LIVES_FILE_CAP);
        let world_name = self
            .wdir
            .root
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("w{}", self.seed));
        live::write_live(
            &self.wdir,
            &self.world,
            &self.cfg,
            self.seed,
            self.awoke_before + self.new_awoke(),
            &world_name,
            &self.sim.frames,
            &self.sim.series,
            &self.sim.notable,
            &lives_vec,
        )?;
        Ok(())
    }

    fn write_pages(&self, verbose: bool) -> std::io::Result<()> {
        let new_awoke = self.new_awoke();
        write_world_pages(
            &self.world,
            &self.cfg,
            &self.wdir,
            self.seed,
            self.awoke_before + new_awoke,
            &self.sim.frames,
            &self.sim.series,
            &self.sim.notable,
            &self.sim.lives,
            self.sim.extinct_at,
            verbose,
        )
    }
}

fn resume_path(
    flags: &HashMap<String, String>,
    positional: Option<String>,
    usage: &str,
) -> std::io::Result<PathBuf> {
    flags
        .get("out")
        .cloned()
        .or(positional)
        .map(PathBuf::from)
        .ok_or_else(|| err(usage))
}

fn resume_frame_every(flags: &HashMap<String, String>) -> u64 {
    flags
        .get("frame-every")
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(400)
}

/// Retire de `events.jsonl` toute ligne dont le tick depasse `keep_upto`. Sert a la reprise
/// apres un arret brutal : le dernier instantane peut etre en retard sur le journal.
fn truncate_events_after(path: &Path, keep_upto: u64) -> std::io::Result<()> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return Ok(()),
    };
    let mut kept = String::with_capacity(text.len());
    let mut cut = false;
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let tick = serde_json::from_str::<Event>(line).map(|e| e.tick).unwrap_or(0);
        if tick <= keep_upto {
            kept.push_str(line);
            kept.push('\n');
        } else {
            cut = true;
        }
    }
    if cut {
        std::fs::write(path, kept)?;
    }
    Ok(())
}

/// Lit un fichier `.jsonl` en un `Vec<T>`. Les lignes illisibles sont ignorees.
fn read_jsonl<T: serde::de::DeserializeOwned>(path: &Path) -> Vec<T> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

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

/// `genesis gallery [dossier]` : reconstruit la page de garde de la bibliotheque.
fn cmd_gallery(dir: String) -> std::io::Result<()> {
    let path = Path::new(&dir);
    build_gallery(path)?;
    println!("Bibliotheque : {}", path.join("index.html").display());
    Ok(())
}

/// `genesis atlas [--dir <dossier>] [--port <P>]` : sert l'arbre d'evolution du projet, la
/// todo et la boite a idees. Local, aucun rapport avec la simulation d'un monde -- reutilise
/// juste le meme binaire pour ne pas faire installer un second outil.
fn cmd_atlas(flags: &HashMap<String, String>) -> std::io::Result<()> {
    let dir = flags.get("dir").cloned().unwrap_or_else(|| "BIBLE/atlas".to_string());
    let port: u16 = flags.get("port").and_then(|s| s.parse().ok()).unwrap_or(8090);
    let root = PathBuf::from(&dir);
    std::fs::create_dir_all(&root)?;
    let real = atlas_serve::spawn(root, port)?;
    println!("Atlas du projet : http://localhost:{real}/");
    println!("Donnees dans {dir} (data.json, todo.json, inbox/). Ctrl-C pour arreter.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}

/// Scanne un dossier de mondes et ecrit `<dossier>/index.html`, la grille de tous les mondes.
fn build_gallery(dir: &Path) -> std::io::Result<()> {
    let mut cards: Vec<gallery_html::Card> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let root = entry.path();
        let meta_path = root.join("meta.json");
        if !meta_path.exists() {
            continue;
        }
        let name = root
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let meta: WorldMeta = match std::fs::read_to_string(&meta_path)
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
        {
            Some(m) => m,
            None => continue,
        };
        let sec_year = SimConfig::load(&root.join("config.toml"))
            .map(|c| c.time.tick_duration_seconds)
            .unwrap_or(3600) as f64;
        let years = (meta.ticks_played as f64 * sec_year / (3600.0 * 24.0 * 365.0)) as u64;

        // Serie temporelle : toutes les lignes pour la mini-courbe, la derniere pour l'etat.
        let series_txt = std::fs::read_to_string(root.join("series.jsonl")).unwrap_or_default();
        let rows: Vec<serde_json::Value> = series_txt
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        let last = rows.last().cloned().unwrap_or(serde_json::json!({}));
        let num = |v: &serde_json::Value, k: &str| v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0);
        let pop = num(&last, "population") as u32;
        let dom_lin = num(&last, "dominant_lineage") as u16;
        let pop_series: Vec<u32> = rows.iter().map(|r| num(r, "population") as u32).collect();

        let agents_awoke = std::fs::read_to_string(root.join("lives.jsonl"))
            .map(|t| t.lines().filter(|l| !l.trim().is_empty()).count())
            .unwrap_or(0);

        cards.push(gallery_html::Card {
            name,
            seed: meta.seed,
            schema: meta.schema_version,
            ticks: meta.ticks_played,
            years,
            pop,
            carrying: num(&last, "carrying_capacity") as u32,
            generations: num(&last, "max_generation") as u32,
            diversity: num(&last, "genetic_diversity") as f32,
            agents_alive: num(&last, "agents_alive") as u32,
            agents_awoke,
            dominant: genesis_core::names::lineage_name(dom_lin),
            pop_series,
            extinct: pop == 0,
        });
    }
    cards.sort_by(|a, b| a.name.cmp(&b.name));
    std::fs::write(dir.join("index.html"), gallery_html::render(&cards))
}

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
            // Un drapeau suivi d'un autre drapeau (ou rien) est booleen : valeur vide.
            match args.get(i + 1) {
                Some(v) if !v.starts_with("--") => {
                    map.insert(name.to_string(), v.clone());
                    i += 2;
                }
                _ => {
                    map.insert(name.to_string(), String::new());
                    i += 1;
                }
            }
        } else {
            i += 1;
        }
    }
    map
}

fn err(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg.to_string())
}


