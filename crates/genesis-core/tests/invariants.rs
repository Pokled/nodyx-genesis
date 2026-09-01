//! Tests d'invariants, ecrits avant de faire confiance au moteur (tranchee 13).

use genesis_core::{tick, SimConfig, WorldState};

fn small_cfg() -> SimConfig {
    let mut c = SimConfig::default();
    c.world.grid_width = 48;
    c.world.grid_height = 48;
    c
}

fn state_json(w: &WorldState) -> String {
    serde_json::to_string(w).unwrap()
}

#[test]
fn same_seed_same_config_same_world_frame_by_frame() {
    let cfg = small_cfg();
    let mut a = WorldState::new(12345, &cfg);
    let mut b = WorldState::new(12345, &cfg);
    for _ in 0..3000 {
        let _ = tick(&mut a, &cfg);
        let _ = tick(&mut b, &cfg);
        assert_eq!(state_json(&a), state_json(&b), "divergence au tick {}", a.tick);
    }
}

#[test]
fn different_seed_different_world() {
    let cfg = small_cfg();
    let mut a = WorldState::new(1, &cfg);
    let mut b = WorldState::new(2, &cfg);
    for _ in 0..500 {
        let _ = tick(&mut a, &cfg);
        let _ = tick(&mut b, &cfg);
    }
    assert_ne!(state_json(&a), state_json(&b));
}

#[test]
fn snapshot_plus_replay_equals_live_state() {
    let cfg = small_cfg();
    let mut live = WorldState::new(777, &cfg);
    for _ in 0..1200 {
        let _ = tick(&mut live, &cfg);
    }
    // instantane
    let snap_json = serde_json::to_string(&live).unwrap();
    // on continue le monde vivant
    for _ in 0..800 {
        let _ = tick(&mut live, &cfg);
    }
    // on repart de l'instantane et on rejoue les memes 800 ticks
    let mut restored: WorldState = serde_json::from_str(&snap_json).unwrap();
    for _ in 0..800 {
        let _ = tick(&mut restored, &cfg);
    }
    assert_eq!(state_json(&live), state_json(&restored));
}

#[test]
fn event_seq_is_monotonic_within_a_run() {
    let cfg = small_cfg();
    let mut w = WorldState::new(99, &cfg);
    let mut next_seq = 0u64;
    let mut last = 0u64;
    for _ in 0..4000 {
        let mut ev = tick(&mut w, &cfg);
        for e in ev.iter_mut() {
            e.seq = next_seq;
            next_seq += 1;
            assert!(e.seq >= last);
            last = e.seq;
        }
    }
}

#[test]
fn population_is_conserved_and_state_stays_bounded() {
    let cfg = small_cfg();
    let mut w = WorldState::new(2024, &cfg);
    for _ in 0..40_000 {
        let ev = tick(&mut w, &cfg);
        assert!(ev.len() <= cfg.events.max_events_per_tick as usize);
        assert!(w.entities.len() < 200_000, "explosion de population");

        // Conservation exacte : 2 fondateurs + naissances = vivants + morts.
        assert_eq!(
            w.population() as u64 + w.deaths_total,
            2 + w.births_total,
            "au tick {}",
            w.tick
        );
    }
}

#[test]
fn structural_matter_is_conserved() {
    // La matiere structurelle (briques, 0.0.2) est conservee exactement : le stock libre
    // plus ce qui est immobilise dans les corps vivants egale la matiere totale du monde,
    // a chaque tick, sur toute la duree.
    let cfg = small_cfg();
    let body = cfg.bricks.body_matter as f64;
    let mut w = WorldState::new(4242, &cfg);
    let total = w.free_matter as f64 + w.population() as f64 * body;
    assert!(w.free_matter >= 0.0);
    for _ in 0..40_000 {
        let _ = tick(&mut w, &cfg);
        assert!(w.free_matter >= -1e-3, "matiere libre negative au tick {}", w.tick);
        let now = w.free_matter as f64 + w.population() as f64 * body;
        assert!(
            (now - total).abs() < 1e-2,
            "matiere non conservee au tick {} : {} vs {}",
            w.tick,
            now,
            total
        );
    }
}

#[test]
fn cells_stay_consistent() {
    // Cellules (0.0.2, tranche 2, etape 1) : tout `cell_id` pointe une cellule vivante, et
    // la somme des effectifs des cellules egale le nombre d'entites en cellule. Les membres
    // restent dans `entities` (etape 1) : la population reste conservee sans changement.
    // Grille par defaut (128x128) : c'est la que des amas assez denses se forment.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(3, &cfg);
    let mut saw_a_cell = false;
    for _ in 0..25_000 {
        let _ = tick(&mut w, &cfg);
        let live: std::collections::HashSet<u32> = w.cells.iter().map(|c| c.id).collect();
        let mut tagged = 0u32;
        for e in w.entities.iter() {
            if let Some(id) = e.cell_id {
                assert!(live.contains(&id), "cell_id orphelin au tick {}", w.tick);
                tagged += 1;
            }
        }
        let members: u32 = w.cells.iter().map(|c| c.member_count).sum();
        assert_eq!(members, tagged, "effectifs de cellule incoherents au tick {}", w.tick);
        if !w.cells.is_empty() {
            saw_a_cell = true;
        }
        // l'invariant de population n'a pas bouge (but de l'etape 1).
        assert_eq!(w.population() as u64 + w.deaths_total, 2 + w.births_total);
    }
    assert!(saw_a_cell, "aucune cellule ne s'est formee en 25000 ticks (graine 3)");
}

#[test]
fn series_stats_are_sane() {
    // Les methodes de la serie temporelle (0.0.2, tranche 3a) sont des lectures pures :
    // quantiles ordonnes, generation moyenne qui croit, tout fini.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(3, &cfg);
    let mut final_gen = 0.0f32;
    for _ in 0..40 {
        for _ in 0..500 {
            let _ = tick(&mut w, &cfg);
        }
        if w.entities.is_empty() {
            break;
        }
        let q = w.trait_quantiles();
        for k in 0..7 {
            assert!(
                q[0][k] <= q[1][k] + 1e-4 && q[1][k] <= q[2][k] + 1e-4,
                "quantiles desordonnes trait {k}"
            );
            for row in q.iter() {
                assert!(row[k].is_finite() && (0.0..=1.0).contains(&row[k]));
            }
        }
        let (gm, gs) = w.generation_stats();
        assert!(gm.is_finite() && gs.is_finite() && gm >= 0.0);
        final_gen = gm;
    }
    // Sur 20000 ticks la population a traverse plusieurs generations.
    assert!(final_gen > 2.0, "la generation moyenne n'a pas progresse : {final_gen}");
}

#[test]
fn a_world_knows_which_engine_made_it() {
    let cfg = small_cfg();
    let w = WorldState::new(1, &cfg);
    assert_eq!(w.engine_version, genesis_core::ENGINE_VERSION);
    assert_eq!(w.schema_version, genesis_core::SCHEMA_VERSION);
}
