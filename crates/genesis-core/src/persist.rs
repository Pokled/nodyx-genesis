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

    pub fn latest_snapshot(&self) -> std::io::Result<Option<WorldState>> {
        let dir = self.root.join("snapshots");
        if !dir.exists() {
            return Ok(None);
        }
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map_or(false, |x| x == "json"))
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
}
