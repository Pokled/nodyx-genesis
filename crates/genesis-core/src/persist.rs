//! Persistance, 0.0.1 : un dossier par monde.
//!
//!   worlds/<nom>/
//!     config.toml            la config utilisee
//!     meta.json              graine, version du moteur, nombre de ticks joues
//!     snapshots/NNNNNNNNN.json   instantanes du World State, tous les N ticks
//!     events.jsonl           journal append-only, un evenement JSON par ligne
//!
//! World State mutable, Event Log append-only, instantanes : c'est le modele verrouille.
//! PostgreSQL est la cible (tranchee 2), le systeme de fichiers suffit pour 0.0.1.

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::SimConfig;
use crate::event::Event;
use crate::world::WorldState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMeta {
    pub seed: u64,
    pub engine_version: String,
    pub schema_version: u32,
    pub ticks_played: u64,
    pub last_event_seq: u64,
    /// Nombre total d'individus qui se sont eveilles sur toute la vie du monde. Cumule a
    /// travers les reprises (`genesis continue`), meme quand les vieilles biographies sont
    /// oubliees des sorties.
    #[serde(default)]
    pub agents_awoke_total: u64,
}

pub struct WorldDir {
    pub root: PathBuf,
}

impl WorldDir {
    pub fn open_or_create(root: impl AsRef<Path>) -> std::io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        std::fs::create_dir_all(root.join("snapshots"))?;
        Ok(WorldDir { root })
    }

    pub fn exists(root: impl AsRef<Path>) -> bool {
        root.as_ref().join("meta.json").exists()
    }

    // -- config

    pub fn write_config(&self, cfg: &SimConfig) -> std::io::Result<()> {
        std::fs::write(self.root.join("config.toml"), cfg.to_toml())
    }

    pub fn read_config(&self) -> std::io::Result<SimConfig> {
        let p = self.root.join("config.toml");
        if p.exists() {
            SimConfig::load(&p)
        } else {
            Ok(SimConfig::default())
        }
    }

    // -- meta

    pub fn write_meta(&self, m: &WorldMeta) -> std::io::Result<()> {
        let json = serde_json::to_vec_pretty(m).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        std::fs::write(self.root.join("meta.json"), json)
    }

    pub fn read_meta(&self) -> std::io::Result<WorldMeta> {
        let bytes = std::fs::read(self.root.join("meta.json"))?;
        serde_json::from_slice(&bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }

    // -- snapshots

    pub fn write_snapshot(&self, w: &WorldState) -> std::io::Result<()> {
        let p = self
            .root
            .join("snapshots")
            .join(format!("{:09}.json", w.tick));
        let json = serde_json::to_vec_pretty(w).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        std::fs::write(p, json)
    }

    /// Ne garde que les `keep` instantanes les plus recents. Un monde qui tourne sans fin
    /// n'a pas besoin de tout son passe sur disque : la reprise et `replay` ne lisent que le
    /// dernier. `keep` doit valoir au moins 1.
    pub fn prune_snapshots(&self, keep: usize) -> std::io::Result<()> {
        let dir = self.root.join("snapshots");
        if !dir.exists() {
            return Ok(());
        }
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        if files.len() <= keep.max(1) {
            return Ok(());
        }
        files.sort();
        let cut = files.len() - keep.max(1);
        for p in &files[..cut] {
            let _ = std::fs::remove_file(p);
        }
        Ok(())
    }

    pub fn latest_snapshot(&self) -> std::io::Result<Option<WorldState>> {
        let dir = self.root.join("snapshots");
        if !dir.exists() {
            return Ok(None);
        }
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        files.sort();
        match files.last() {
            Some(p) => {
                let bytes = std::fs::read(p)?;
                let w = serde_json::from_slice(&bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
                Ok(Some(w))
            }
            None => Ok(None),
        }
    }

    // -- journal

    /// Ecrit les evenements en fin de journal, un JSON par ligne. Depuis 0.0.2 (tranche 3b)
    /// le `seq` est attribue a la creation dans `tick()`, plus ici.
    pub fn append_events(&self, events: &[Event]) -> std::io::Result<()> {
        if events.is_empty() {
            return Ok(());
        }
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.root.join("events.jsonl"))?;
        for e in events.iter() {
            let line = serde_json::to_string(e).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }

    /// Journal en pyramide (`serve`) : `events.jsonl` ne garde en detail que les evenements
    /// au tick `>= keep_from`. Les evenements de chapitre (`EventKind::is_chapter`) plus vieux
    /// sont deverses dans `events.chronicle.jsonl` (append-only, grossit tres lentement), le
    /// reste est laisse tomber. Le moteur ne relit jamais le journal : le monde reste
    /// identique.
    ///
    /// La compaction ne se declenche que si la plus vieille ligne est anterieure a
    /// `trigger_below` (hysteresis : on ne reecrit pas 100 Mo a chaque tour de boucle pour
    /// grignoter quelques centaines de ticks). Bon marche sinon (on ne lit que la premiere
    /// ligne). Renvoie le nombre de lignes retirees de `events.jsonl`.
    pub fn compact_journal(&self, keep_from: u64, trigger_below: u64) -> std::io::Result<u64> {
        use std::io::{BufRead, BufReader};

        // Filtre bon marche : on ne parse l'evenement complet que si la saillance vaut la
        // peine (les chapitres sont tous a 180+). Le gros du journal est a 2-3.
        #[derive(Deserialize)]
        struct Line {
            tick: u64,
            #[serde(default)]
            salience: u8,
        }
        const CHAPTER_MIN_SALIENCE: u8 = 180;

        let path = self.root.join("events.jsonl");
        let Ok(file) = std::fs::File::open(&path) else {
            return Ok(0);
        };
        let mut reader = BufReader::new(file);

        // Coup d'oeil sur la premiere ligne : si elle est assez recente, rien a faire.
        let mut first = String::new();
        if reader.read_line(&mut first)? == 0 {
            return Ok(0);
        }
        if let Ok(l) = serde_json::from_str::<Line>(first.trim()) {
            if l.tick >= trigger_below {
                return Ok(0);
            }
        }

        let tmp = self.root.join("events.jsonl.tmp");
        let mut keep_w = std::io::BufWriter::new(std::fs::File::create(&tmp)?);
        let mut chron_w = std::io::BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.root.join("events.chronicle.jsonl"))?,
        );
        let mut dropped = 0u64;

        // La premiere ligne, deja lue.
        let process = |raw: &str, keep_w: &mut std::io::BufWriter<std::fs::File>,
                       chron_w: &mut std::io::BufWriter<std::fs::File>,
                       dropped: &mut u64|
         -> std::io::Result<()> {
            let s = raw.trim_end();
            if s.is_empty() {
                return Ok(());
            }
            let Ok(l) = serde_json::from_str::<Line>(s) else {
                *dropped += 1;
                return Ok(());
            };
            if l.tick >= keep_from {
                writeln!(keep_w, "{s}")?;
                return Ok(());
            }
            *dropped += 1;
            if l.salience >= CHAPTER_MIN_SALIENCE {
                if let Ok(ev) = serde_json::from_str::<Event>(s) {
                    if ev.kind.is_chapter() {
                        writeln!(chron_w, "{s}")?;
                    }
                }
            }
            Ok(())
        };

        process(&first, &mut keep_w, &mut chron_w, &mut dropped)?;
        let mut buf = String::new();
        loop {
            buf.clear();
            if reader.read_line(&mut buf)? == 0 {
                break;
            }
            process(&buf, &mut keep_w, &mut chron_w, &mut dropped)?;
        }

        keep_w.flush()?;
        chron_w.flush()?;
        drop(keep_w);
        drop(chron_w);
        std::fs::rename(&tmp, &path)?;
        Ok(dropped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventKind;

    fn tmp_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "genesis-persist-{}-{}-{}",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(d.join("snapshots")).unwrap();
        d
    }

    fn ev(seq: u64, tick: u64, kind: EventKind) -> Event {
        let mut e = Event::now(tick, kind);
        e.seq = seq;
        e
    }

    #[test]
    fn compact_journal_keeps_window_and_chronicles_the_rest() {
        let root = tmp_dir("compact");
        let wd = WorldDir { root: root.clone() };

        // 0..2000 : un evenement de bruit par tick, plus un evenement de chapitre
        // (`PopulationMilestone`) et un evenement saillant mais pas de chapitre (`AgentAwoke`)
        // tous les 500 ticks.
        let mut batch = Vec::new();
        for t in 0..2000u64 {
            batch.push(ev(t * 3, t, EventKind::EntitySpawned { entity: t }));
            if t % 500 == 0 {
                batch.push(ev(t * 3 + 1, t, EventKind::PopulationMilestone { level: 100 }));
                batch.push(ev(t * 3 + 2, t, EventKind::AgentAwoke { entity: t }));
            }
        }
        wd.append_events(&batch).unwrap();
        let before_lines = std::fs::read_to_string(root.join("events.jsonl")).unwrap().lines().count();

        // Fenetre : garder a partir du tick 1500, ne declencher que sous 1200.
        let dropped = wd.compact_journal(1500, 1200).unwrap();
        assert!(dropped > 0, "rien n'a ete compacte");

        let after = std::fs::read_to_string(root.join("events.jsonl")).unwrap();
        let chron = std::fs::read_to_string(root.join("events.chronicle.jsonl")).unwrap();

        // events.jsonl ne contient plus que des ticks >= 1500.
        for line in after.lines() {
            let t: serde_json::Value = serde_json::from_str(line).unwrap();
            assert!(t["tick"].as_u64().unwrap() >= 1500, "ligne trop vieille gardee : {line}");
        }
        // La chronique ne contient que des chapitres anterieurs a la fenetre : les
        // `PopulationMilestone` des ticks 0/500/1000, pas les `AgentAwoke`, pas le bruit.
        assert_eq!(chron.lines().count(), 3, "chronique : {}", chron);
        for line in chron.lines() {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            assert!(v["tick"].as_u64().unwrap() < 1500);
            assert!(v["kind"].get("population_milestone").is_some(), "pas un chapitre : {line}");
        }
        assert_eq!(
            after.lines().count() as u64 + dropped,
            before_lines as u64,
            "des lignes ont disparu sans etre comptees"
        );

        // Deuxieme appel : la premiere ligne est deja >= 1200, no-op.
        assert_eq!(wd.compact_journal(1500, 1200).unwrap(), 0);

        // `journal_keep_ticks = 0` cote appelant : la compaction n'est jamais appelee, rien
        // ici a tester de plus. On nettoie.
        let _ = std::fs::remove_dir_all(&root);
    }
}
