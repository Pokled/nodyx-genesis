//! Tests d'invariants, ecrits avant de faire confiance au moteur (tranchee 13).

use genesis_core::{tick, SimConfig, WorldState};
#[allow(unused_imports)]
use genesis_core::{ResourceField, Space};

fn small_cfg() -> SimConfig {
    let mut c = SimConfig::default();
    c.world.grid_width = 48;
    c.world.grid_height = 48;
    c
}

/// Config de reference historique : le monde d'avant les grands reglages de septembre 2026
/// (grille 128x128, capacite de charge ~2293, pas de saisons). La grille par defaut est passee
/// a 192x192 et `matter_per_cell` a 0,26 le 2026-09-02/03 ; les tests dont les seuils sont
/// calibres sur la dynamique du plateau ~2293, ou qui n'ont pas besoin d'un grand monde, s'y
/// epinglent pour rester stables et rapides. Le mecanisme teste ne depend pas de ces reglages.
fn ref_cfg() -> SimConfig {
    let mut c = SimConfig::default();
    c.world.grid_width = 128;
    c.world.grid_height = 128;
    c.bricks.matter_per_cell = 0.14;
    c.season.amplitude = 0.0;
    c.season.temp_amplitude_c = 0.0;
    c.planet.heat_tol_span_c = 0.0;
    c.cells.divide = false;
    c.cells.repel = false;
    c.cells.cell_burn_relief = 0.0;
    c
}

/// Comme `ref_cfg` mais un cran plus robuste (160x160, plateau ~5100) : pour les tests qui
/// stressent la temperature et qui, a 128x128 / matiere 0,14, effondreraient le monde au lieu
/// de le tester. Pas de saisons.
fn temp_cfg() -> SimConfig {
    let mut c = ref_cfg();
    c.world.grid_width = 160;
    c.world.grid_height = 160;
    c.bricks.matter_per_cell = 0.20;
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
    // Depuis 0.0.2 (tranche 3b) le `seq` est attribue dans `tick()`, pas a l'ecriture.
    // Il doit rester croissant d'un evenement au suivant (des trous sont admis : le
    // garde-fou anti-cascade tronque certains evenements deja numerotes).
    let cfg = small_cfg();
    let mut w = WorldState::new(99, &cfg);
    let mut last: Option<u64> = None;
    for _ in 0..4000 {
        let ev = tick(&mut w, &cfg);
        for e in ev.iter() {
            if let Some(l) = last {
                assert!(e.seq > l, "seq non croissant : {} apres {}", e.seq, l);
            }
            last = Some(e.seq);
        }
    }
    assert!(last.is_some(), "aucun evenement produit");
}

#[test]
fn causes_are_wired() {
    // Tranche 3b : `PopulationCrash` cite la vague de morts, `LineageExtinct` cite la
    // derniere mort de la lignee. On verifie qu'au moins un lien existe et pointe un
    // `EntityDied` anterieur.
    use genesis_core::EventKind;
    let cfg = SimConfig::default();
    let mut linked = 0usize;
    // Un balayage de graines : selon la trajectoire, la lignee qui s'eteint ou le crash
    // n'arrive pas toujours dans la fenetre. On s'arrete des qu'un lien est verifie.
    for seed in [1u64, 5, 7, 3, 12, 31] {
        let mut w = WorldState::new(seed, &cfg);
        let mut died: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
        for _ in 0..60_000 {
            let ev = tick(&mut w, &cfg);
            for e in ev.iter() {
                match &e.kind {
                    EventKind::EntityDied { .. } => {
                        died.insert(e.seq, e.tick);
                    }
                    EventKind::PopulationCrash { .. } | EventKind::LineageExtinct { .. } => {
                        for &c in e.causes.iter() {
                            assert!(c < e.seq, "cause {} apres l'effet {}", c, e.seq);
                            let dtick = died.get(&c).copied().unwrap_or_else(|| {
                                panic!("cause {} n'est pas un EntityDied connu", c)
                            });
                            assert!(
                                dtick <= e.tick,
                                "cause au tick {} apres l'effet {}",
                                dtick,
                                e.tick
                            );
                            linked += 1;
                        }
                    }
                    _ => {}
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        if linked > 0 {
            break;
        }
    }
    assert!(linked > 0, "aucun lien causal cable sur tout le balayage de graines");
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
    let mut w = WorldState::new(1, &cfg);
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
    assert!(saw_a_cell, "aucune cellule ne s'est formee en 25000 ticks (graine 1)");
}

#[test]
fn series_stats_are_sane() {
    // Les methodes de la serie temporelle (0.0.2, tranche 3a) sont des lectures pures :
    // quantiles ordonnes, generation moyenne qui croit, tout fini.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut final_gen = 0.0f32;
    for _ in 0..40 {
        for _ in 0..500 {
            let _ = tick(&mut w, &cfg);
        }
        if w.entities.is_empty() {
            break;
        }
        let q = w.trait_quantiles();
        for k in 0..genesis_core::genome::N_TRAITS {
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
fn agents_awake_and_remember() {
    // Cognition (0.0.3, tranche 1) : des entites s'eveillent en agents, accumulent des
    // souvenirs bornes et decroissants, chaque souvenir ancre pointe un evenement anterieur.
    use genesis_core::EventKind;
    let cfg = SimConfig::default();
    let max_mem = cfg.cognition.max_memories as usize;
    let mut w = WorldState::new(1, &cfg);
    let mut seen_seq: std::collections::HashSet<u64> = std::collections::HashSet::new();
    let mut awoke = 0usize;
    let mut peak_agents = 0u32;
    for _ in 0..60_000 {
        let ev = tick(&mut w, &cfg);
        for e in ev.iter() {
            seen_seq.insert(e.seq);
            if let EventKind::AgentAwoke { .. } = e.kind {
                awoke += 1;
            }
        }
        let (n, _) = w.agent_stats();
        peak_agents = peak_agents.max(n);
        for a in w.entities.iter() {
            if let Some(m) = &a.mind {
                assert!(m.episodic.len() <= max_mem, "trop de souvenirs au tick {}", w.tick);
                for mem in m.episodic.iter() {
                    assert!(
                        mem.strength > 0.0 && mem.strength <= 1.0 + 1e-4,
                        "force de souvenir hors bornes : {}",
                        mem.strength
                    );
                    if let Some(seq) = mem.event_seq {
                        assert!(seen_seq.contains(&seq), "souvenir ancre sur un seq inconnu");
                    }
                }
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(awoke > 0, "aucune entite ne s'est eveillee en agent");
    assert!(peak_agents > 0, "aucun agent vivant a aucun moment");
}

#[test]
fn social_ties_form() {
    // Cognition (0.0.3, tranche 7) : les agents accumulent des relations vers d'autres
    // agents. Bornees, familiarite dans (0, 1], valence dans [-1, 1], `other` plausible.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut any_tie = false;
    for _ in 0..40_000 {
        let _ = tick(&mut w, &cfg);
        for e in w.entities.iter() {
            let Some(m) = &e.mind else { continue };
            assert!(m.social.len() <= genesis_core::cognition::MAX_TIES);
            for s in m.social.iter() {
                assert!(
                    s.familiarity > 0.0 && s.familiarity <= 1.0 + 1e-4,
                    "familiarite hors bornes : {}",
                    s.familiarity
                );
                assert!((-1.0..=1.0).contains(&s.valence), "valence hors bornes : {}", s.valence);
                assert!(s.other < w.next_entity_id, "relation vers un id impossible");
                any_tie = true;
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(any_tie, "aucun agent n'a noue de relation en 40000 ticks");
}

#[test]
fn alarms_are_emitted_and_bounded() {
    // La Voix (0.0.4, tranche 1) : les agents affames crient. Les signaux restent bornes en
    // nombre et en duree, leurs positions sont dans la grille, et il s'en emet reellement.
    let cfg = SimConfig::default();
    let ttl = cfg.voice.signal_ttl;
    let cap = cfg.voice.max_signals;
    let (w, h) = (cfg.world.grid_width as f32, cfg.world.grid_height as f32);
    let mut ws = WorldState::new(1, &cfg);
    let mut ever = false;
    for _ in 0..40_000 {
        let _ = tick(&mut ws, &cfg);
        assert!(ws.signals.len() <= cap, "trop de signaux : {}", ws.signals.len());
        for s in ws.signals.iter() {
            assert!(
                ws.tick.saturating_sub(s.born) < ttl,
                "signal trop vieux au tick {}",
                ws.tick
            );
            assert!(
                s.pos.x >= 0.0 && s.pos.x < w && s.pos.y >= 0.0 && s.pos.y < h,
                "signal hors grille : {:?}",
                s.pos
            );
        }
        if !ws.signals.is_empty() {
            ever = true;
        }
        if ws.entities.is_empty() {
            break;
        }
    }
    assert!(ever, "aucune alarme emise en 40000 ticks");
}

#[test]
fn health_stays_bounded() {
    // Biologie de fond (0.0.3, tranche 8) : `Entity.health` reste dans [0, 1] a chaque tick.
    // Un corps vieux et eprouve descend sous 0.9 ; un corps jeune et rassasie tient au-dessus
    // de 0.9.
    let cfg = SimConfig::default();
    let expected_mean = cfg.lifecycle.lifespan_ticks_mean as f32;
    let mut w = WorldState::new(1, &cfg);
    let mut saw_worn = false;
    let mut saw_intact = false;
    for _ in 0..40_000 {
        let _ = tick(&mut w, &cfg);
        for e in w.entities.iter() {
            assert!(
                (0.0..=1.0).contains(&e.health),
                "sante hors bornes au tick {} : {}",
                w.tick,
                e.health
            );
            let expected = expected_mean * (0.5 + e.genome.traits.lifespan);
            let age_frac = e.age_ticks as f32 / expected.max(1.0);
            if age_frac > 1.2 && e.health < 0.9 {
                saw_worn = true;
            }
            if age_frac < 0.4 && e.energy > cfg.reproduction.energy_threshold && e.health > 0.9 {
                saw_intact = true;
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(saw_worn, "aucun corps use vu en 40000 ticks");
    assert!(saw_intact, "aucun corps jeune et intact vu en 40000 ticks");
}

#[test]
fn agent_modes_are_chosen() {
    // Cognition (0.0.3, tranche 6) : un agent choisit explicitement un mode de comportement.
    // Sur une course, plusieurs modes doivent apparaitre (pas seulement forage), et le mode
    // survit au rejeu (il est dans l'instantane).
    use genesis_core::BehaviorMode;
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut seen: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
    for _ in 0..60_000 {
        let _ = tick(&mut w, &cfg);
        for e in w.entities.iter() {
            if let Some(m) = &e.mind {
                seen.insert(m.mode.as_str());
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(
        seen.len() >= 3,
        "les agents n'utilisent que {:?} : le modele de comportement ne discrimine pas",
        seen
    );
    assert!(seen.contains(BehaviorMode::Forage.as_str()));
}

#[test]
fn the_herd_does_not_plaster_the_wall() {
    // Le « piege de coin » severe (troupeau ecrase a plat contre x=0) etait un artefact du
    // stream v9 / graine 3. Avec le stade cognition (memoire qui etale les morts, modeles de
    // comportement), la chimiotaxie mene toujours le troupeau vers la region fertile, mais
    // aucune bande de 3 cases au bord ne concentre le gros de la population.
    let cfg = SimConfig::default();
    let (w, h) = (cfg.world.grid_width as f32, cfg.world.grid_height as f32);
    let mut world = WorldState::new(1, &cfg);
    for _ in 0..30_000 {
        let _ = tick(&mut world, &cfg);
        if world.entities.is_empty() {
            break;
        }
    }
    let n = world.entities.len().max(1) as f32;
    let wall = world
        .entities
        .iter()
        .filter(|e| {
            let p = e.position;
            p.x < 3.0 || p.x > w - 3.0 || p.y < 3.0 || p.y > h - 3.0
        })
        .count() as f32
        / n;
    assert!(wall < 0.1, "le troupeau est colle a la paroi : {wall:.2}");
}

#[test]
fn personality_evolves() {
    // Cognition (0.0.3, tranche 5) : `caution` et `curiosity` sont des traits herites. Sur
    // une longue course, leur moyenne doit s'ecarter nettement de sa valeur de depart
    // (~0.5) et rester dans [0, 1] : la selection agit sur le temperament.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let start = w.trait_stats().0; // moyennes de depart
    let (c0, k0) = (start[7], start[8]);
    let mut last = start;
    for _ in 0..60_000 {
        let _ = tick(&mut w, &cfg);
        if w.entities.is_empty() {
            break;
        }
        last = w.trait_stats().0;
        for k in 7..9 {
            assert!(last[k].is_finite() && (0.0..=1.0).contains(&last[k]), "personnalite hors bornes");
        }
    }
    // Les deux traits sont bien serialises, herites, dans les bornes ; ils derivent (la
    // selection sur le temperament peut etre faible, on ne prejuge pas du sens).
    let moved = (last[7] - c0).abs() + (last[8] - k0).abs();
    assert!(moved > 0.0, "la personnalite n'a pas bouge du tout");
    assert!(last[7] > 0.0 && last[8] > 0.0, "une population vivante a une personnalite non nulle");
}

#[test]
fn needs_stay_bounded() {
    // Cognition (0.0.3, tranche 4) : les trois jauges de tout agent restent dans [0, 1] et
    // finies, sur toute la duree.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut saw_hunger = false;
    for _ in 0..40_000 {
        let _ = tick(&mut w, &cfg);
        for a in w.entities.iter() {
            let Some(m) = &a.mind else { continue };
            for v in [m.needs.hunger, m.needs.fear, m.needs.solitude] {
                assert!(v.is_finite() && (0.0..=1.0).contains(&v), "jauge hors bornes : {v}");
            }
            if m.needs.hunger > 0.3 {
                saw_hunger = true;
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(saw_hunger, "aucun agent n'a jamais eu faim");
}

#[test]
fn witnessed_memories_are_anchored() {
    // Cognition (0.0.3, tranche 3) : un agent temoin d'une mort en garde un souvenir de
    // genre `Witnessed` dont `event_seq` pointe le vrai `EntityDied`, anterieur ou du meme
    // tick que la formation du souvenir.
    use genesis_core::{EventKind, MemoryKind};
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut died: std::collections::HashMap<u64, u64> = std::collections::HashMap::new(); // seq -> tick
    let mut anchored = 0usize;
    for _ in 0..60_000 {
        let ev = tick(&mut w, &cfg);
        for e in ev.iter() {
            if let EventKind::EntityDied { .. } = e.kind {
                died.insert(e.seq, e.tick);
            }
        }
        for a in w.entities.iter() {
            let Some(m) = &a.mind else { continue };
            for mem in m.episodic.iter() {
                if matches!(mem.kind, MemoryKind::Witnessed) {
                    let seq = mem.event_seq.expect("un souvenir Witnessed doit etre ancre");
                    let dtick = died
                        .get(&seq)
                        .copied()
                        .unwrap_or_else(|| panic!("event_seq {seq} n'est pas un EntityDied connu"));
                    assert!(dtick <= mem.formed_tick, "la mort vue est posterieure au souvenir");
                    anchored += 1;
                }
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(anchored > 0, "aucun souvenir ancre sur une mort en 60000 ticks");
}

#[test]
fn agent_promotion_is_reversible() {
    // Un agent qui n'a plus aucun souvenir depuis longtemps retombe entite de fond : aucun
    // agent ne doit garder une memoire vide au-dela du delai de grace plus de latence.
    let cfg = SimConfig::default();
    let slack = cfg.cognition.grace_ticks + cfg.cognition.lapse_ticks + cfg.watch.interval_ticks;
    let mut w = WorldState::new(7, &cfg);
    for _ in 0..50_000 {
        let _ = tick(&mut w, &cfg);
        for a in w.entities.iter() {
            if let Some(m) = &a.mind {
                if m.episodic.is_empty() {
                    let since = w.tick.saturating_sub(m.awoke_tick);
                    assert!(
                        since <= slack,
                        "agent {} sans souvenir depuis {} ticks, aurait du retomber",
                        a.id,
                        since
                    );
                }
            }
        }
        if w.entities.is_empty() {
            break;
        }
    }
}

#[test]
fn a_world_knows_which_engine_made_it() {
    let cfg = small_cfg();
    let w = WorldState::new(1, &cfg);
    assert_eq!(w.engine_version, genesis_core::ENGINE_VERSION);
    assert_eq!(w.schema_version, genesis_core::SCHEMA_VERSION);
}

#[test]
fn dominant_genome_shift_is_watched() {
    // Le basculement du genome dominant (schema v16) : sur un monde sous stress climatique,
    // le centre genetique de la population se deplace plusieurs fois, et c'est detecte. La
    // cle etablie et le compteur restent coherents. `genome_shift_persist_checks = 0` coupe.
    use genesis_core::event::EventKind;

    // Le monde de reference complet (grille 192, saisons nourriciere et thermique) : les
    // goulots de disette y deplacent le genome dominant. Graine 1 produit un basculement
    // avant 100000 ticks.
    let cfg = SimConfig::default();
    let mut w = WorldState::new(1, &cfg);
    let mut shifts = 0u32;
    for _ in 0..100_000 {
        let ev = tick(&mut w, &cfg);
        for e in &ev {
            if let EventKind::GenomeShift { from, to, .. } = e.kind {
                shifts += 1;
                assert_ne!(from, to, "basculement du genome vers lui-meme");
            }
        }
        assert!(
            w.watch.dominant_shift_streak < 64,
            "compteur de basculement qui s'emballe au tick {}",
            w.tick
        );
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(shifts >= 1, "aucun basculement de genome en 100000 ticks (monde de reference, graine 1)");

    // Coupe : plus aucun basculement.
    let mut off = cfg.clone();
    off.watch.genome_shift_persist_checks = 0;
    let mut w2 = WorldState::new(1, &off);
    for _ in 0..100_000 {
        let ev = tick(&mut w2, &off);
        assert!(
            !ev.iter().any(|e| matches!(e.kind, EventKind::GenomeShift { .. })),
            "basculement alors que genome_shift_persist_checks = 0"
        );
        if w2.entities.is_empty() {
            break;
        }
    }
}

#[test]
fn climate_shapes_the_world() {
    // Le climat (schema : config seulement, pas d'etat) agit vraiment sur le monde : un
    // monde loin de sa temperature optimale voit plus de morts de faim. Effet inerte au
    // defaut (`temperature_c == temp_optimal_c`), donc pas de regression. Deterministe.
    let optimal = temp_cfg(); // temperature_c == temp_optimal_c == 15 : taxe thermique nulle
    let mut harsh = optimal.clone();
    harsh.planet.temperature_c = -12.0; // 27 degres sous l'optimum : taxe thermique ecrasante

    let run = |cfg: &SimConfig, seed: u64| {
        let mut w = WorldState::new(seed, cfg);
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        w.population()
    };
    // Un ecart thermique severe ecrase le monde : l'effectif d'equilibre s'effondre (ou le
    // monde s'eteint) la ou l'optimum tient un plateau. Vrai sur toute graine viable.
    for &s in &[1u64, 3, 11, 13] {
        let opt_pop = run(&optimal, s);
        let harsh_pop = run(&harsh, s);
        assert!(opt_pop > 1000, "le monde a l'optimum de la graine {s} s'est effondre");
        assert!(
            harsh_pop * 2 < opt_pop,
            "le grand froid ne coute presque rien : graine {s}, {harsh_pop} contre {opt_pop} a l'optimum"
        );
    }

    // Inerte au defaut (`temperature_c == temp_optimal_c`) et deterministe.
    assert_eq!(run(&optimal, 1), run(&optimal, 1), "climat non deterministe");
}

#[test]
fn cells_merge_when_membranes_overlap() {
    // Fusion de cellules (schema v15) : deux membranes stables qui se chevauchent et se
    // ressemblent fusionnent. Il s'en produit reellement (graine 1) ; le compteur cumule
    // colle au nombre d'evenements ; l'invariant d'effectif tient aussi les ticks de fusion ;
    // et `fuse = false` en produit zero.
    use genesis_core::event::EventKind;

    let cfg = ref_cfg();
    let mut w = WorldState::new(1, &cfg);
    let mut merges = 0u64;
    for _ in 0..30_000 {
        let ev = tick(&mut w, &cfg);
        for e in &ev {
            if let EventKind::CellsMerged { cell, absorbed, size, .. } = e.kind {
                merges += 1;
                assert_ne!(cell, absorbed, "une cellule fusionne avec elle-meme");
                assert!(!w.cells.iter().any(|c| c.id == absorbed), "cellule absorbee encore la");
                assert!(size >= cfg.cells.min_members, "fusion sous la taille minimale");
            }
        }
        // Invariant d'effectif : la somme des effectifs de cellule egale les entites taguees,
        // meme les ticks ou une fusion vient d'avoir lieu.
        let tagged = w.entities.iter().filter(|e| e.cell_id.is_some()).count() as u32;
        let members: u32 = w.cells.iter().map(|c| c.member_count).sum();
        assert_eq!(members, tagged, "effectifs incoherents au tick {}", w.tick);
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(merges > 0, "aucune fusion de cellule en 30000 ticks (graine 1)");
    assert_eq!(merges, w.cells_merged_total, "compteur de fusion desynchronise");

    // fuse = false : plus aucune fusion.
    let mut off = cfg.clone();
    off.cells.fuse = false;
    let mut w2 = WorldState::new(1, &off);
    for _ in 0..30_000 {
        let ev = tick(&mut w2, &off);
        assert!(
            !ev.iter().any(|e| matches!(e.kind, EventKind::CellsMerged { .. })),
            "fusion alors que fuse = false, tick {}",
            w2.tick
        );
        if w2.entities.is_empty() {
            break;
        }
    }
    assert_eq!(w2.cells_merged_total, 0);
}

#[test]
fn cells_divide_when_large_and_stretched() {
    // Division cellulaire (schema v19) : une cellule grande, mure et etiree se pince en deux.
    // Il s'en produit reellement (graine 1) ; la fille a `parent_cell` renseigne ; l'invariant
    // d'effectif tient meme aux ticks de division ; le compteur colle ; `divide = false` coupe.
    use genesis_core::event::EventKind;

    let mut cfg = ref_cfg();
    // `ref_cfg` coupe la division ; on la reactive ici et on abaisse un peu les seuils pour
    // qu'elle tombe dans une fenetre courte a l'echelle 128x128 (cellules plus petites).
    cfg.cells.divide = true;
    cfg.cells.divide_members = 22;
    cfg.cells.divide_elongation = 1.5;
    cfg.cells.divide_age_ticks = 1500;

    let mut w = WorldState::new(1, &cfg);
    let mut divs = 0u64;
    for _ in 0..45_000 {
        let ev = tick(&mut w, &cfg);
        for e in &ev {
            if let EventKind::CellDivided { parent, child, size, .. } = e.kind {
                divs += 1;
                assert_ne!(parent, child, "une cellule se divise en elle-meme");
                assert!(size >= cfg.cells.min_members, "cellule fille sous la taille minimale");
                let daughter = w.cells.iter().find(|c| c.id == child);
                assert_eq!(
                    daughter.and_then(|c| c.parent_cell),
                    Some(parent),
                    "la fille ne pointe pas sa mere"
                );
            }
        }
        let tagged = w.entities.iter().filter(|e| e.cell_id.is_some()).count() as u32;
        let members: u32 = w.cells.iter().map(|c| c.member_count).sum();
        assert_eq!(members, tagged, "effectifs incoherents au tick {}", w.tick);
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(divs > 0, "aucune division de cellule en 45000 ticks (graine 1)");
    assert_eq!(divs, w.cells_divided_total, "compteur de division desynchronise");

    // divide = false : plus aucune division.
    let mut off = cfg.clone();
    off.cells.divide = false;
    let mut w2 = WorldState::new(1, &off);
    for _ in 0..45_000 {
        let ev = tick(&mut w2, &off);
        assert!(
            !ev.iter().any(|e| matches!(e.kind, EventKind::CellDivided { .. })),
            "division alors que divide = false, tick {}",
            w2.tick
        );
        if w2.entities.is_empty() {
            break;
        }
    }
    assert_eq!(w2.cells_divided_total, 0);
}

#[test]
fn cells_repel_when_they_overlap_without_fusing() {
    // Repulsion (schema v19, config seulement) : deux cellules qui se frolent sans etre
    // parentes se repoussent. La repulsion ne bouge que des positions (l'invariant d'effectif
    // tient), et son effet mesurable est une distance au voisin plus grande et une diversite
    // genetique plus haute (les cellules non parentes restent distinctes). Deterministe.
    // On isole la repulsion : `cell_burn_relief` (0.0.2 tranche 2b) rend les cellules plus
    // nombreuses et noie l'effet de la repulsion sur la diversite, ce test-ci ne varie que
    // `repel`.
    let mut on = SimConfig::default();
    on.cells.cell_burn_relief = 0.0;
    let mut off = on.clone();
    off.cells.repel = false;

    let run = |cfg: &SimConfig| -> (f64, f32) {
        let mut w = WorldState::new(1, cfg);
        let mut sep_sum = 0.0f64;
        let mut samples = 0usize;
        for step in 0..45_000 {
            let _ = tick(&mut w, cfg);
            let tagged = w.entities.iter().filter(|e| e.cell_id.is_some()).count() as u32;
            let members: u32 = w.cells.iter().map(|c| c.member_count).sum();
            assert_eq!(members, tagged, "effectifs incoherents au tick {}", w.tick);
            if step >= 22_000 && step % 500 == 0 && w.cells.len() >= 3 {
                for i in 0..w.cells.len() {
                    let mut nearest = f64::MAX;
                    for j in 0..w.cells.len() {
                        if i == j {
                            continue;
                        }
                        let reach = (w.cells[i].radius + w.cells[j].radius).max(0.1) as f64;
                        let d = w.cells[i].position.dist2(&w.cells[j].position).sqrt() as f64;
                        nearest = nearest.min(d / reach);
                    }
                    if nearest < 1e5 {
                        sep_sum += nearest;
                        samples += 1;
                    }
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        assert!(samples > 200, "pas assez de cellules echantillonnees ({samples})");
        (sep_sum / samples as f64, w.genetic_diversity())
    };

    let (on_sep, on_div) = run(&on);
    let (off_sep, off_div) = run(&off);
    assert!(
        on_sep >= off_sep && on_div > off_div,
        "la repulsion ne separe / ne diversifie pas : voisin {on_sep:.3} vs {off_sep:.3}, \
         diversite {on_div:.3} vs {off_div:.3}"
    );
    assert_eq!(run(&on).1, on_div, "repulsion non deterministe");
}

#[test]
fn tissues_form_from_adhering_kin_cells() {
    // Tissus (0.0.2, `[cells] tissue`, config seulement). Des cellules de genome proche dont
    // les membranes se touchent adherent en tissu (composante connexe, >= tissue_min). Derive
    // chaque tick, `Cell.tissue` porte l'id, l'id du tissu est le plus petit id de cellule du
    // groupe. Verifie : il s'en forme (graine 1), un tissu a >= tissue_min cellules, toutes ses
    // cellules partagent le meme id, `tissue = false` -> toutes None. Deterministe.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    // seuils un peu plus laches pour que ca tombe dans une fenetre courte a l'echelle 128x128
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    let mut off = on.clone();
    off.cells.tissue = false;

    let run = |cfg: &SimConfig| -> (u32, bool) {
        let mut w = WorldState::new(1, cfg);
        let mut max_tissue = 0u32;
        let mut consistent = true;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            // groupe les cellules par id de tissu, verifie taille et coherence
            let mut by: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
            for c in &w.cells {
                if let Some(tid) = c.tissue {
                    by.entry(tid).or_default().push(c.id);
                    // l'id du tissu doit etre l'id d'une cellule vivante du groupe
                    if !w.cells.iter().any(|d| d.id == tid && d.tissue == Some(tid)) {
                        consistent = false;
                    }
                }
            }
            for (_tid, ids) in &by {
                max_tissue = max_tissue.max(ids.len() as u32);
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (max_tissue, consistent)
    };

    let (on_max, on_consistent) = run(&on);
    let (off_max, _) = run(&off);
    assert!(on_max >= on.cells.tissue_min, "aucun tissu de taille >= {} (graine 1)", on.cells.tissue_min);
    assert!(on_consistent, "id de tissu incoherent (pointe une cellule hors du groupe)");
    assert_eq!(off_max, 0, "tissu forme alors que tissue = false");
    assert_eq!(run(&on).0, on_max, "detection de tissu non deterministe");
}

#[test]
fn tissue_bonds_hold_a_tissue_through_perturbation() {
    // Adhesion persistante (0.0.2, `[cells] tissue_bond`, config seulement). La connexite d'un
    // tissu ne vient plus d'un test de distance refait de zero chaque tick, mais de LIENS de
    // paire gardes dans le temps : un lien ne casse qu'au-dela d'un etirement franc
    // (`bond_break`) ou d'une derive genetique forte, et une cellule tissee resiste a la
    // division. Un tissu tient alors une perturbation au lieu de se defaire au premier ecart.
    // Verifie (graine 1, avec predation comme perturbation) : il se noue des liens ; les tissus
    // tiennent PLUS AU TOTAL avec les liens qu'avec la derivation tick a tick, meme graine ;
    // `tissue_bond = false` ne garde aucun lien ; l'ecosysteme tient ; deterministe.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.cells.tissue_bond = true;
    on.predation.enabled = true; // la perturbation : on mange les cellules de bord
    let mut off = on.clone();
    off.cells.tissue_bond = false;

    let run = |cfg: &SimConfig| -> (u64, u64, bool, u64) {
        let mut w = WorldState::new(1, cfg);
        let mut tissue_cell_ticks = 0u64; // somme sur les ticks du nombre de cellules en tissu
        let mut ever_bonded = false;
        let mut bonds_when_off = 0u64;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            tissue_cell_ticks += w.cells.iter().filter(|c| c.tissue.is_some()).count() as u64;
            if !w.cell_bonds.is_empty() {
                ever_bonded = true;
            }
            if !cfg.cells.tissue_bond {
                bonds_when_off += w.cell_bonds.len() as u64;
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (tissue_cell_ticks, w.entities.len() as u64, ever_bonded, bonds_when_off)
    };

    let (on_ticks, on_pop, on_bonded, _) = run(&on);
    let (off_ticks, _off_pop, _, off_bonds) = run(&off);

    assert!(on_bonded, "aucun lien d'adhesion noue en 45000 ticks (graine 1)");
    assert_eq!(off_bonds, 0, "des liens gardes alors que tissue_bond = false");
    assert!(on_pop > 100, "l'ecosysteme s'eteint avec l'adhesion persistante (pop {on_pop})");
    assert!(
        on_ticks > off_ticks,
        "l'adhesion persistante ne fait pas tenir les tissus plus longtemps : \
         {on_ticks} (liens) vs {off_ticks} (derive tick a tick)"
    );
    assert_eq!(run(&on).0, on_ticks, "adhesion persistante non deterministe");
}

#[test]
fn epithelium_shield_makes_a_sealed_nappe_untouchable() {
    // Nappe scellee (0.0.2, `[cells] epithelium_shield`, config seulement). Un tissu ordonne
    // (psi6 moyen >= shield_order) et assez grand fait REMPART : toutes ses cellules sont hors
    // d'atteinte d'un predateur, pas seulement le coeur. Purement passif (aucune energie ne
    // bouge). Verifie (graine 1, avec tissue_bond et predation) : des nappes se scellent
    // (`Cell.sealed`) ; sous predation, le rempart fait BAISSER le compte de morts par
    // predation ; `epithelium_shield = false` laisse `Cell.sealed` a false partout et la
    // predation intacte ; deterministe ; l'ecosysteme tient.
    let mut base = SimConfig::default();
    base.cells.cell_burn_relief = 0.0;
    base.cells.tissue = true;
    base.cells.tissue_kin = 0.8;
    base.cells.tissue_reach = 1.3;
    base.cells.tissue_bond = true;
    base.predation.enabled = true;

    let mut shield = base.clone();
    shield.cells.epithelium_shield = true;
    shield.cells.shield_order = 0.30; // seuil abaisse pour l'echelle courte du test
    shield.cells.shield_cells = 4;

    let run = |cfg: &SimConfig| -> (u64, u64, bool, bool) {
        let mut w = WorldState::new(1, cfg);
        let mut ever_sealed = false;
        let mut sealed_without_flag = false;
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            for c in &w.cells {
                if c.sealed {
                    ever_sealed = true;
                    if !cfg.cells.epithelium_shield {
                        sealed_without_flag = true;
                    }
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (w.deaths_predation, w.entities.len() as u64, ever_sealed, sealed_without_flag)
    };

    let (shield_deaths, shield_pop, shield_sealed, _) = run(&shield);
    let (base_deaths, _base_pop, _base_sealed, base_sealed_no_flag) = run(&base);

    assert!(shield_sealed, "aucune nappe scellee en 40000 ticks (graine 1) : le test ne prouve rien");
    assert!(!base_sealed_no_flag, "Cell.sealed vrai alors que epithelium_shield = false");
    assert!(shield_pop > 100, "l'ecosysteme s'eteint avec le rempart (pop {shield_pop})");
    assert!(
        shield_deaths < base_deaths,
        "le rempart ne protege pas : {shield_deaths} morts par predation (avec) vs {base_deaths} (sans)"
    );
    assert_eq!(run(&shield).0, shield_deaths, "rempart non deterministe");
}

#[test]
fn muscle_seek_food_moves_tissue_toward_resources() {
    // Locomotion dirigee (0.0.2, `[cells] muscle_seek_food`, config seulement). Une cellule
    // contractile qui sent vraiment mieux a portee (meme chimiotaxie que `forage_target`, deja
    // utilisee par chaque entite) tire tout son nuage d'un cran vers la cible pendant la phase
    // active de contraction, au lieu de seulement reformer sa silhouette sur son propre centre
    // (symetrique, ca ne deplace nulle part). Verifie (graine 1) : des cellules contractiles
    // existent des deux cotes ; l'energie moyenne des membres de cellules contractiles est PLUS
    // HAUTE avec la chimiotaxie qu'avec l'axe arbitraire, meme graine (le tissu qui se dirige
    // vers la nourriture finit mieux nourri) ; `muscle_seek_food = false` laisse le comportement
    // d'origine ; deterministe ; l'ecosysteme tient.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.cells.tissue_bond = true;
    on.cells.muscle_contract = true;
    on.cells.muscle_elong = 1.5; // attrape plus de cellules a l'echelle courte du test
    on.cells.muscle_seek_food = true;
    let mut off = on.clone();
    off.cells.muscle_seek_food = false;

    let run = |cfg: &SimConfig| -> (f64, bool, u64, i64) {
        let mut w = WorldState::new(1, cfg);
        let mut nrg_sum = 0.0f64;
        let mut nrg_n = 0u64;
        let mut had_muscle = false;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            let slot: std::collections::HashMap<u32, usize> =
                w.cells.iter().enumerate().map(|(k, c)| (c.id, k)).collect();
            for e in &w.entities {
                let Some(id) = e.cell_id else { continue };
                let Some(&k) = slot.get(&id) else { continue };
                let c = &w.cells[k];
                if c.tissue.is_some() && c.elongation >= cfg.cells.muscle_elong {
                    had_muscle = true;
                    nrg_sum += e.energy as f64;
                    nrg_n += 1;
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        let mean_nrg = if nrg_n > 0 { nrg_sum / nrg_n as f64 } else { -1.0 };
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.position.y * 89.0) as i64);
        }
        (mean_nrg, had_muscle, w.entities.len() as u64, fp)
    };

    let (on_nrg, on_had, on_pop, on_fp) = run(&on);
    let (off_nrg, off_had, _off_pop, off_fp) = run(&off);

    assert!(on_had && off_had, "aucune cellule contractile en 45000 ticks (graine 1) : le test ne prouve rien");
    assert!(on_pop > 100, "l'ecosysteme s'eteint avec la locomotion dirigee (pop {on_pop})");
    assert!(
        on_nrg > off_nrg,
        "la locomotion dirigee ne nourrit pas mieux le tissu : energie moyenne {on_nrg:.3} (dirige) vs {off_nrg:.3} (axe arbitraire)"
    );
    assert_ne!(on_fp, off_fp, "muscle_seek_food ne change rien a la trajectoire");
    assert_eq!(run(&on).0, on_nrg, "locomotion dirigee non deterministe");
}

#[test]
fn predation_kills_the_weak_and_conserves_the_death_count() {
    // Predation (0.0.2, experiments/012, config seulement, `[predation] enabled`). Une entite
    // affamee mange une entite nettement plus faible a portee : la proie meurt par predation,
    // une part de son energie passe au predateur. Aucune regle ne nomme un predateur.
    // Verifie : il s'en produit (graine 1) ; le compteur cumule colle et l'invariant de
    // comptage tient (total = famine + age + predation) ; l'ecosysteme ne s'effondre pas ;
    // `enabled = false` en produit zero ; deterministe.
    let mut on = SimConfig::default();
    on.cells.cell_burn_relief = 0.0; // isole l'effet predation
    on.predation.enabled = true;
    let mut off = on.clone();
    off.predation.enabled = false;

    let run = |cfg: &SimConfig| -> (u64, u64, f64) {
        let mut w = WorldState::new(1, cfg);
        let mut pop_min = u64::MAX;
        for step in 0..40_000 {
            let _ = tick(&mut w, cfg);
            assert_eq!(
                w.deaths_total,
                w.deaths_starvation + w.deaths_age + w.deaths_predation,
                "comptage des morts incoherent au tick {}",
                w.tick
            );
            if step >= 10_000 {
                pop_min = pop_min.min(w.entities.len() as u64);
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (w.deaths_predation, w.entities.len() as u64, pop_min as f64)
    };

    let (on_pred, on_pop, _on_min) = run(&on);
    let (off_pred, _off_pop, _off_min) = run(&off);
    assert!(on_pred > 0, "aucune predation en 40000 ticks (graine 1)");
    assert_eq!(off_pred, 0, "predation alors que enabled = false");
    assert!(on_pop > 100, "l'ecosysteme s'eteint sous predation (pop finale {on_pop})");
    assert_eq!(run(&on).0, on_pred, "predation non deterministe");
    // La sante de l'ecosysteme sous predation (population, diversite, derive des traits) est
    // une question d'A/B, consignee dans experiments/012_predation.md, pas d'un test unitaire.
}

#[test]
fn cell_burn_relief_buffers_famine_without_distorting_fat_times() {
    // Membrane (0.0.2, tranche 2b, config seulement) : un membre de cellule EN DANGER de
    // famine brule moins d'energie de base (`cell_burn_relief` * gravite). Le repli ne touche
    // que la zone de peril, donc il fait tenir plus de membres de cellule sur le plateau (creux
    // de disette amortis) SANS gonfler la population en cellule aux temps gras. Le mecanisme ne
    // touche que l'energie, pas l'appartenance : l'invariant d'effectif tient. Deterministe.
    let on = SimConfig::default();
    let mut off = on.clone();
    off.cells.cell_burn_relief = 0.0;

    let run = |cfg: &SimConfig| -> (f64, f64) {
        let mut w = WorldState::new(1, cfg);
        let mut member_ticks = 0.0f64;
        let mut trough = f64::MAX;
        let mut n = 0usize;
        for step in 0..45_000 {
            let _ = tick(&mut w, cfg);
            let tagged = w.entities.iter().filter(|e| e.cell_id.is_some()).count() as u32;
            let members: u32 = w.cells.iter().map(|c| c.member_count).sum();
            assert_eq!(members, tagged, "effectifs incoherents au tick {}", w.tick);
            if step >= 22_000 {
                member_ticks += tagged as f64;
                trough = trough.min(w.cells.len() as f64);
                n += 1;
            }
            if w.entities.is_empty() {
                break;
            }
        }
        assert!(n > 20_000, "fenetre trop courte ({n})");
        (member_ticks / n as f64, trough)
    };

    let (on_mt, on_trough) = run(&on);
    let (off_mt, off_trough) = run(&off);
    assert!(
        on_mt >= off_mt && on_trough >= off_trough,
        "le repli anti-famine ne soutient pas les cellules : membres/tick {on_mt:.0} vs \
         {off_mt:.0}, plancher de cellules {on_trough} vs {off_trough}"
    );
    assert_eq!(run(&on).0, on_mt, "cell_burn_relief non deterministe");
}

#[test]
fn tissue_shelter_protects_the_interior_and_flows_energy_outward() {
    // Abri du tissu (0.0.2, `[cells] tissue_shelter`, config seulement). Une cellule entouree
    // (`tissue_bonds >= shelter_bonds`) est a l'interieur de la nappe : un predateur ne peut pas
    // l'atteindre, et une part de l'energie de ses membres coule vers les cellules de bord.
    // Aucune regle ne nomme "germinal" / "somatique" : c'est la geometrie. Verifie (graine 1) :
    // `tissue_bonds` se peuple (des cellules interieures existent) et vaut 0 partout sans tissu ;
    // sous predation, l'abri fait BAISSER le compte de morts par predation ; deterministe ;
    // `tissue_shelter = false` laisse la predation intacte.
    let mut base = SimConfig::default();
    base.cells.cell_burn_relief = 0.0;
    base.cells.tissue = true;
    base.cells.tissue_kin = 0.8;
    base.cells.tissue_reach = 1.3;
    base.predation.enabled = true;

    let mut shelter = base.clone();
    shelter.cells.tissue_shelter = true;

    let mut no_tissue = base.clone();
    no_tissue.cells.tissue = false;

    let run = |cfg: &SimConfig| -> (u64, u8, bool) {
        let mut w = WorldState::new(1, cfg);
        let mut max_bonds = 0u8;
        let mut bonds_zeroed_without_tissue = true;
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            for c in &w.cells {
                max_bonds = max_bonds.max(c.tissue_bonds);
                if c.tissue.is_none() && c.tissue_bonds != 0 {
                    bonds_zeroed_without_tissue = false;
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (w.deaths_predation, max_bonds, bonds_zeroed_without_tissue)
    };

    let (pred_plain, bonds_plain, _) = run(&base);
    let (pred_shelter, _, _) = run(&shelter);
    let (_, bonds_none, none_ok) = run(&no_tissue);

    assert!(bonds_plain >= base.cells.shelter_bonds as u8, "aucune cellule interieure (tissue_bonds max {bonds_plain})");
    assert_eq!(bonds_none, 0, "tissue_bonds non nul alors que tissue = false");
    assert!(none_ok, "tissue_bonds non nul sur une cellule hors tissu");
    // L'abri infléchit la trajectoire (predation immunisee pour l'interieur + flux d'energie
    // vers le bord) : le monde diverge de la variante ou `tissue_bonds` n'est qu'informatif.
    assert_ne!(
        pred_shelter, pred_plain,
        "tissue_shelter ne change rien a la trajectoire (predation {pred_shelter} == {pred_plain})"
    );
    assert_eq!(run(&shelter).0, pred_shelter, "abri du tissu non deterministe");
    // Le SENS (l'abri fait-il durer le tissu, localise-t-il la division au coeur ?) est une
    // question d'A/B, consignee dans experiments/009_organism.md, pas d'un test unitaire.
}

#[test]
fn organism_pool_binds_the_fate_of_the_whole() {
    // Mise en commun de l'energie (0.0.2, `[organism] pool_share`). A chaque controle, chaque
    // membre d'un organisme est ramene d'une fraction vers l'energie moyenne des membres :
    // l'organisme a faim ou est repu EN ENTIER. Conserve (deplacement vers la moyenne), sans
    // RNG. Verifie (graine 1) : il existe des organismes multi-cellules ; l'ecart d'energie
    // ENTRE membres d'un meme organisme est plus serre avec la mise en commun que sans ;
    // deterministe ; `pool_share = 0` ne change rien a la trajectoire d'un temoin sans mise en
    // commun ; l'ecosysteme tient.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.organism.enabled = true;
    on.organism.reach = 1.6;
    on.organism.min_cells = 2;
    on.organism.pool_share = 0.35;
    let mut off = on.clone();
    off.organism.pool_share = 0.0;

    // ecart moyen d'energie entre membres d'un meme organisme, cumule sur la vie du monde
    let run = |cfg: &SimConfig| -> (f64, u64, bool) {
        let mut w = WorldState::new(1, cfg);
        let mut dev_sum = 0.0f64;
        let mut dev_n = 0u64;
        let mut saw_multi = false;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            if w.tick % 1000 != 0 {
                continue;
            }
            let cell_org: std::collections::HashMap<u32, u32> = w
                .cells
                .iter()
                .filter_map(|c| c.organism.map(|o| (c.id, o)))
                .collect();
            let mut by: std::collections::HashMap<u32, Vec<f32>> = std::collections::HashMap::new();
            for e in &w.entities {
                if let Some(cid) = e.cell_id {
                    if let Some(&oid) = cell_org.get(&cid) {
                        by.entry(oid).or_default().push(e.energy);
                    }
                }
            }
            for (_oid, es) in &by {
                if es.len() < 2 {
                    continue;
                }
                saw_multi = true;
                let mean = es.iter().sum::<f32>() / es.len() as f32;
                let mad = es.iter().map(|x| (x - mean).abs()).sum::<f32>() / es.len() as f32;
                dev_sum += mad as f64;
                dev_n += 1;
            }
            if w.entities.is_empty() {
                break;
            }
        }
        let dev = if dev_n > 0 { dev_sum / dev_n as f64 } else { 0.0 };
        (dev, w.entities.len() as u64, saw_multi)
    };

    let (on_dev, on_pop, on_multi) = run(&on);
    let (off_dev, _off_pop, _) = run(&off);

    assert!(on_multi, "aucun organisme multi-cellules en 45000 ticks (graine 1)");
    assert!(on_pop > 100, "l'ecosysteme s'eteint sous mise en commun (pop {on_pop})");
    assert!(
        on_dev < off_dev,
        "la mise en commun ne resserre pas l'energie des membres : {on_dev:.3} (avec) vs {off_dev:.3} (sans)"
    );
    assert_eq!(run(&on).0, on_dev, "mise en commun non deterministe");
}

#[test]
fn adipeux_reserve_rescues_starving_organism_members() {
    // Reserve adipeuse (0.0.2, `[organism] adipeux_share`, config seulement). En plus du
    // lissage uniforme de `pool_share`, les membres d'une cellule RONDE et GORGEE versent une
    // part de leur surplus aux membres de l'organisme vraiment en danger (energie sous 2x le
    // seuil de famine). Purement passif (aucun mouvement, aucune entite hors organisme
    // ponctionnee) : ca ne doit pas reproduire le piege de la digestion (experiments/014).
    // Verifie (graine 1, avec organismes) : l'energie MINIMALE parmi les membres d'organismes
    // multi-cellules, moyennee sur la vie du monde, est PLUS HAUTE avec la reserve qu'avec le
    // seul lissage uniforme, meme graine ; `adipeux_share = 0` ne change rien ; deterministe ;
    // l'ecosysteme tient.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.organism.enabled = true;
    on.organism.reach = 1.6;
    on.organism.min_cells = 2;
    on.organism.pool_share = 0.15;
    on.organism.adipeux_share = 0.6; // marque a l'echelle courte du test
    on.organism.adipeux_rich_frac = 0.6;
    let mut off = on.clone();
    off.organism.adipeux_share = 0.0;

    let run = |cfg: &SimConfig| -> (f64, u64, bool, i64) {
        let mut w = WorldState::new(1, cfg);
        let mut min_sum = 0.0f64;
        let mut min_n = 0u64;
        let mut saw_multi = false;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            if w.tick % 1000 == 0 {
                let cell_org: std::collections::HashMap<u32, u32> = w
                    .cells
                    .iter()
                    .filter_map(|c| c.organism.map(|o| (c.id, o)))
                    .collect();
                let mut by: std::collections::HashMap<u32, Vec<f32>> = std::collections::HashMap::new();
                for e in &w.entities {
                    if let Some(cid) = e.cell_id {
                        if let Some(&oid) = cell_org.get(&cid) {
                            by.entry(oid).or_default().push(e.energy);
                        }
                    }
                }
                for (_oid, es) in &by {
                    if es.len() < 2 {
                        continue;
                    }
                    saw_multi = true;
                    let min = es.iter().cloned().fold(f32::MAX, f32::min);
                    min_sum += min as f64;
                    min_n += 1;
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        let mean_min = if min_n > 0 { min_sum / min_n as f64 } else { -1.0 };
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.energy * 13.0) as i64);
        }
        (mean_min, w.entities.len() as u64, saw_multi, fp)
    };

    let (on_min, on_pop, on_multi, on_fp) = run(&on);
    let (off_min, _off_pop, _off_multi, off_fp) = run(&off);

    assert!(on_multi, "aucun organisme multi-cellules en 45000 ticks (graine 1)");
    assert!(on_pop > 100, "l'ecosysteme s'eteint avec la reserve adipeuse (pop {on_pop})");
    assert!(
        on_min > off_min,
        "la reserve adipeuse ne remonte pas le plancher d'energie : {on_min:.3} (avec) vs {off_min:.3} (sans)"
    );
    assert_ne!(on_fp, off_fp, "adipeux_share ne change rien a la trajectoire");
    assert_eq!(run(&on).0, on_min, "reserve adipeuse non deterministe");
}

#[test]
fn adhesion_gene_changes_tissue_formation() {
    // Gene d'adhesion (0.0.2, piste D etape 1, `[cells] adhesion_gene`) : premier gene d'un
    // genome STRUCTUREL, distinct du genome de traits (`Genome.structural`, hors `trait_l1` --
    // sinon ca fausserait en silence l'echelle de `fuse_kin`/`tissue_kin`/`kin_dist`). Sans le
    // levier, `tissue_kin` est un seuil de parente FIXE pour tout le monde ; avec lui, chaque
    // paire de cellules adhere selon SA tolerance heritee (`Cell.mean_adhesion`, moyenne du
    // gene sur les membres). `adhesion_gene = false` ne tire AUCUN RNG pour ce gene (fige au
    // neutre `0,5`) : la trajectoire du monde reste strictement identique a avant son existence,
    // meme garantie que tout autre levier de cette base.
    //
    // Verifie ici seulement l'effet MECANIQUE, deterministe : le levier change reellement la
    // formation de tissu (nombre de cellules, trajectoire). L'hypothese plus ambitieuse --
    // que la moyenne ponderee-population du gene DERIVE sous selection -- a ete testee
    // empiriquement (plusieurs graines, plusieurs seuils) et n'a montre qu'un signal faible et
    // incoherent (+0,002 a +0,02 selon la graine, parfois nul ou legerement negatif) : la
    // formation de cellule est gouvernee par la parente de TRAITS (`fuse_kin`), pas par ce
    // gene, donc les cellules regroupent des entites sans correlation avec lui -- peu de
    // variance exploitable entre cellules, donc peu de prise pour la selection individuelle,
    // meme si le seuil marche mecaniquement. Documente dans `experiments/018_adhesion_gene.md`.
    // Pas d'assertion de derive ici : ce serait un seuil instable, pas un invariant.
    let mut on = SimConfig::default();
    on.cells.cell_burn_relief = 0.0;
    on.cells.tissue = true;
    on.cells.tissue_bond = true;
    on.cells.tissue_kin = 0.5;
    on.cells.tissue_reach = 1.3;
    on.cells.tissue_shelter = true;
    on.predation.enabled = true;
    on.cells.adhesion_gene = true;
    let mut off = on.clone();
    off.cells.adhesion_gene = false;

    let run = |cfg: &SimConfig| -> (u64, i64) {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.energy * 13.0) as i64);
        }
        (w.entities.len() as u64, fp)
    };

    let (on_pop, on_fp) = run(&on);
    let (off_pop, off_fp) = run(&off);

    assert!(on_pop > 100 && off_pop > 100, "l'ecosysteme s'eteint (on {on_pop}, off {off_pop})");
    assert_ne!(on_fp, off_fp, "adhesion_gene ne change rien a la trajectoire");
    assert_eq!(run(&on).1, on_fp, "gene d'adhesion non deterministe");
}

#[test]
fn role_gene_creates_selectable_variance_without_cell_averaging() {
    // Gene de role (0.0.2, piste D etape 2, `[cells] role_gene` + `role_reproduction_gate`, la
    // consequence DURE -- `role_share` en `020` en essaie une plus douce). Contrairement au gene
    // d'adhesion (etape 1, `018_adhesion_gene.md`) qui moyennait par cellule et diluait toute
    // variance exploitable entre membres, ce gene est lu PAR ENTITE : chaque entite compare
    // SON PROPRE seuil heredite (`germinal_bias`) a l'entassement de sa cellule
    // (`Cell.tissue_bonds`), et seule une entite germinale (assez entouree pour SON seuil) peut
    // se reproduire (phase 7) -- une entite hors cellule reste toujours eligible.
    // Verifie (graine 1, tissu + predation + abri, meme regime que `018`) :
    // - le levier bloque reellement des candidats a la reproduction (la trajectoire diverge) ;
    // - la variance INTRA-cellule du gene est reelle sous selection (`on_sd` > 0), CONTRAIREMENT
    //   a une moyenne par cellule qui l'aurait effacee par construction (`off_sd` = 0, gene fige
    //   au neutre sans le levier, aucun tirage RNG) ;
    // - l'ecosysteme tient des deux cotes ; deterministe.
    // Pas d'assertion de DIRECTION de derive de la moyenne de population : sondage prealable sur
    // plusieurs graines (voir `019_role_gene.md`) montre un sens qui depend du contexte (une
    // graine derive vers plus permissif, une autre vers plus strict) -- une pression de
    // selection reelle mais dependante de l'ecologie locale, pas un seuil stable a figer ici.
    let mut on = SimConfig::default();
    on.cells.cell_burn_relief = 0.0;
    on.cells.tissue = true;
    on.cells.tissue_bond = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.cells.tissue_shelter = true;
    on.predation.enabled = true;
    on.cells.role_gene = true;
    on.cells.role_reproduction_gate = true;
    let mut off = on.clone();
    off.cells.role_gene = false;
    off.cells.role_reproduction_gate = false;

    let run = |cfg: &SimConfig| -> (u64, i64, f64, u64) {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.energy * 13.0) as i64);
        }
        let mut var_sum = 0.0f64;
        let mut var_n = 0usize;
        for c in &w.cells {
            let vals: Vec<f64> = w
                .entities
                .iter()
                .filter(|e| e.cell_id == Some(c.id))
                .map(|e| e.genome.structural.germinal_bias as f64)
                .collect();
            if vals.len() < 2 {
                continue;
            }
            let m = vals.iter().sum::<f64>() / vals.len() as f64;
            let v = vals.iter().map(|x| (x - m).powi(2)).sum::<f64>() / vals.len() as f64;
            var_sum += v;
            var_n += 1;
        }
        let within_cell_sd = if var_n > 0 { (var_sum / var_n as f64).sqrt() } else { 0.0 };
        (w.entities.len() as u64, fp, within_cell_sd, w.role_blocked_total)
    };

    let (on_pop, on_fp, on_sd, on_blocked) = run(&on);
    let (off_pop, off_fp, off_sd, off_blocked) = run(&off);

    assert!(on_pop > 100 && off_pop > 100, "l'ecosysteme s'eteint (on {on_pop}, off {off_pop})");
    assert_ne!(on_fp, off_fp, "role_gene ne change rien a la trajectoire");
    assert_eq!(off_sd, 0.0, "germinal_bias varie sans le levier (devrait rester fige a 0,5)");
    assert_eq!(off_blocked, 0, "role_blocked_total avance sans le levier");
    assert!(on_blocked > 0, "le gene de role n'a jamais ecarte personne (mecanisme jamais exerce)");
    assert!(
        on_sd > 0.02,
        "le gene de role n'a pas de variance intra-cellule exploitable : sd {on_sd:.4}"
    );
    assert_eq!(run(&on).1, on_fp, "gene de role non deterministe");
}

#[test]
fn role_share_moves_energy_only_when_the_gene_actually_varies() {
    // Partage de role (0.0.2, piste D etape 2 bis, `[cells] role_share`, `experiments/020`).
    // Version douce de `role_reproduction_gate` (`019`, blocage dur, cout ecologique severe) :
    // une entite somatique (sous SON seuil `germinal_bias`) reverse une part de son surplus aux
    // entites germinales de sa cellule, sans jamais bloquer personne. Verifie deux choses :
    // (1) avec `role_gene = true` (le seuil varie reellement d'une entite a l'autre), le flux
    //     change reellement la trajectoire (fingerprint different) ;
    // (2) avec `role_gene = false` (seuil fige a 0,5 pour tout le monde), `role_share` est un
    //     no-op MECANIQUE garanti : toutes les entites d'une meme cellule partagent alors le
    //     meme seuil ET le meme `tissue_bonds` (propriete de la cellule, pas de l'entite), donc
    //     jamais de scission donneurs/receveurs au sein d'une cellule -- pas juste inefficace,
    //     structurellement impossible. Confirme par le sondage prealable (deux graines,
    //     trajectoires bit a bit identiques).
    // Cout ecologique et derive de selection non asserts ici (voir `020_role_share.md` : reel
    // mais modere, sans direction fiable -- un troisieme resultat honnete apres `018`/`019`).
    let mut base = SimConfig::default();
    base.cells.cell_burn_relief = 0.0;
    base.cells.tissue = true;
    base.cells.tissue_bond = true;
    base.cells.tissue_kin = 0.8;
    base.cells.tissue_reach = 1.3;
    base.cells.tissue_shelter = true;
    base.predation.enabled = true;
    base.cells.role_reproduction_gate = false;

    let run = |cfg: &SimConfig| -> (u64, i64) {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.energy * 13.0) as i64);
        }
        (w.entities.len() as u64, fp)
    };

    // (1) gene actif : le partage doit changer quelque chose.
    let mut gene_off = base.clone();
    gene_off.cells.role_gene = true;
    gene_off.cells.role_share = false;
    let mut gene_on = base.clone();
    gene_on.cells.role_gene = true;
    gene_on.cells.role_share = true;
    let (pop_no_share, fp_no_share) = run(&gene_off);
    let (pop_share, fp_share) = run(&gene_on);
    assert!(pop_no_share > 100 && pop_share > 100, "l'ecosysteme s'eteint avec role_share");
    assert_ne!(fp_no_share, fp_share, "role_share ne change rien alors que le gene varie");
    assert_eq!(run(&gene_on).1, fp_share, "role_share non deterministe");

    // (2) gene fige (role_gene = false) : role_share doit rester un no-op mecanique garanti.
    let mut frozen_off = base.clone();
    frozen_off.cells.role_gene = false;
    frozen_off.cells.role_share = false;
    let mut frozen_on = base.clone();
    frozen_on.cells.role_gene = false;
    frozen_on.cells.role_share = true;
    let (_, fp_frozen_off) = run(&frozen_off);
    let (_, fp_frozen_on) = run(&frozen_on);
    assert_eq!(
        fp_frozen_off, fp_frozen_on,
        "role_share agit alors que le seuil est uniforme (aucune scission possible dans une cellule)"
    );
}

#[test]
fn muscle_contract_perturbs_only_when_an_elongated_tissue_cell_exists() {
    // Contraction musculaire (0.0.2, `[cells] muscle_contract`, config seulement). Une cellule
    // d'un tissu dont le nuage de membres est assez fusiforme exerce une force axiale oscillante
    // sur ses membres (+ un peu de courant sur les libres proches). Aucune regle ne nomme un
    // muscle : c'est une condition sur l'allongement et l'appartenance a un tissu. Verifie
    // (graine 1) : il existe bien des cellules contractiles ; la trajectoire diverge du temoin
    // des lors ; l'ecosysteme tient ; `muscle_contract = false` ne change rien ; deterministe.
    let mut on = SimConfig::default();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.cells.muscle_contract = true;
    on.cells.muscle_elong = 1.5; // attrape plus de cellules a l'echelle courte du test
    let mut off = on.clone();
    off.cells.muscle_contract = false;

    let run = |cfg: &SimConfig| -> (u64, bool, i64) {
        let mut w = WorldState::new(1, cfg);
        let mut had_contractile = false;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            if cfg.cells.muscle_contract {
                for c in &w.cells {
                    if c.tissue.is_some() && c.elongation >= cfg.cells.muscle_elong {
                        had_contractile = true;
                    }
                }
            }
            if w.entities.is_empty() {
                break;
            }
        }
        // empreinte de position, entiere pour une comparaison exacte
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.position.y * 89.0) as i64);
        }
        (w.entities.len() as u64, had_contractile, fp)
    };

    let (on_pop, on_had, on_fp) = run(&on);
    let (off_pop, _off_had, off_fp) = run(&off);

    assert!(on_had, "aucune cellule contractile en 45000 ticks (graine 1) : le test ne prouve rien");
    assert!(on_pop > 100, "l'ecosysteme s'eteint sous contraction musculaire (pop {on_pop})");
    assert_ne!(on_fp, off_fp, "la contraction musculaire ne change rien a la trajectoire");
    assert_eq!(run(&on).2, on_fp, "contraction musculaire non deterministe");
    let _ = off_pop;
}

#[test]
fn organisms_form_and_keep_a_stable_id() {
    // Organisme (0.0.2, `[organism] enabled`, config seulement). Une composante connexe de
    // cellules qui adherent (aucune parente exigee) reconnue apres quelques controles tenus,
    // gardee avec un id stable tant qu'un noyau persiste. Verifie (graine 1) : il s'en forme ;
    // une cellule qui pointe un organisme pointe un organisme vivant ; un id reconnu tient
    // plusieurs controles (il ne clignote pas) ; `enabled = false` n'en forme aucun et laisse
    // tous les `Cell.organism` a None ; deterministe.
    let mut on = SimConfig::default();
    on.organism.enabled = true;
    on.organism.reach = 1.6; // membranes qui se frolent, fenetre courte a l'echelle du test
    on.organism.min_cells = 2;
    let mut off = on.clone();
    off.organism.enabled = false;

    let run = |cfg: &SimConfig| -> (u64, u32, bool, u32) {
        let mut w = WorldState::new(1, cfg);
        let mut consistent = true;
        let mut max_lifespan = 0u32; // plus longue suite de controles ou un meme id est reste
        let mut seen: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        let mut off_clean = true;
        for _ in 0..45_000 {
            let _ = tick(&mut w, cfg);
            let live: std::collections::HashSet<u32> = w.organisms.iter().map(|o| o.id).collect();
            for c in &w.cells {
                if let Some(oid) = c.organism {
                    if !live.contains(&oid) {
                        consistent = false;
                    }
                    if !cfg.organism.enabled {
                        off_clean = false;
                    }
                }
            }
            if !cfg.organism.enabled && !w.organisms.is_empty() {
                off_clean = false;
            }
            for o in &w.organisms {
                let e = seen.entry(o.id).or_insert(0);
                *e += 1;
                max_lifespan = max_lifespan.max(*e);
            }
            if w.entities.is_empty() {
                break;
            }
        }
        (w.organisms_formed_total, w.organisms.len() as u32, consistent && off_clean, max_lifespan)
    };

    let (on_formed, _on_alive, on_ok, on_life) = run(&on);
    let (off_formed, off_alive, off_ok, _off_life) = run(&off);

    assert!(on_formed > 0, "aucun organisme reconnu en 45000 ticks (graine 1)");
    assert!(on_ok, "une cellule pointe un organisme mort");
    assert!(on_life >= 5, "les id d'organisme clignotent (suite max {on_life} controles)");
    assert_eq!(off_formed, 0, "organisme reconnu alors que enabled = false");
    assert_eq!(off_alive, 0, "organismes vivants alors que enabled = false");
    assert!(off_ok, "Cell.organism non nul alors que enabled = false");
    assert_eq!(run(&on).0, on_formed, "reconnaissance d'organisme non deterministe");
}

#[test]
fn bounty_calls_are_heard() {
    // La Voix tranche 2 (schema v17) : un agent qui mange bien sur une case franchement riche
    // lance un appel `Bounty`. Il s'en emet reellement (graine 1, monde mur) ; `bounty_call =
    // false` en produit zero ; et un appel entendu (`bounty_pull > 0`) inflechit la trajectoire
    // de la population, donc le monde diverge de la variante ou l'appel est visible mais inerte.
    use genesis_core::SignalKind;

    let cfg = ref_cfg();
    let mut w = WorldState::new(1, &cfg);
    let mut bounty_seen = 0u64;
    for _ in 0..40_000 {
        let _ = tick(&mut w, &cfg);
        bounty_seen += w.signals.iter().filter(|s| s.kind == SignalKind::Bounty).count() as u64;
        for s in &w.signals {
            assert!(
                w.tick.saturating_sub(s.born) < cfg.voice.signal_ttl,
                "signal fossile au tick {}",
                w.tick
            );
        }
        if w.entities.is_empty() {
            break;
        }
    }
    assert!(bounty_seen > 0, "aucun appel Bounty en 40000 ticks (graine 1)");

    // bounty_call = false : plus aucun appel.
    let mut off = cfg.clone();
    off.voice.bounty_call = false;
    let mut w2 = WorldState::new(1, &off);
    for _ in 0..40_000 {
        let _ = tick(&mut w2, &off);
        assert!(
            !w2.signals.iter().any(|s| s.kind == SignalKind::Bounty),
            "appel Bounty alors que bounty_call = false, tick {}",
            w2.tick
        );
        if w2.entities.is_empty() {
            break;
        }
    }

    // Appel entendu contre appel inerte : les appels sont emis dans les deux cas, mais seul le
    // premier inflechit les cibles. Les mondes doivent diverger.
    let run_pop = |pull: f32| {
        let mut c = ref_cfg();
        c.voice.bounty_pull = pull;
        let mut w = WorldState::new(1, &c);
        for _ in 0..40_000 {
            let _ = tick(&mut w, &c);
            if w.entities.is_empty() {
                break;
            }
        }
        (w.population(), w.deaths_starvation)
    };
    let heard = run_pop(0.35);
    let inert = run_pop(0.0);
    assert_ne!(heard, inert, "l'appel entendu ne change rien au monde");
}

#[test]
fn nerve_relay_extends_alarm_perception_through_a_tissue() {
    // Relais nerveux (0.0.2, `[voice] nerve_relay`, config seulement). Un tissu qui compte assez
    // de membres agents etend leur portee de perception d'alarme au-dela de `signal_radius`,
    // comme si le reseau du tissu relayait le cri plutot que chacun le percevant seul.
    // Verifie directement (pas d'effet ecologique indirect a mesurer) : le moteur cumule
    // `nerve_signals_relayed`, incremente SEULEMENT quand une alarme est percue hors de la
    // portee simple mais dans la portee relayee. `nerve_relay = false` le laisse a zero
    // (mecaniquement : aucun multiplicateur de portee n'est calcule).
    let mut on = ref_cfg();
    on.cells.tissue = true;
    on.cells.tissue_kin = 0.8;
    on.cells.tissue_reach = 1.3;
    on.voice.nerve_relay = true;
    on.voice.nerve_min_agents = 2; // marge basse a l'echelle courte du test
    on.voice.nerve_radius_mult = 3.0;
    let mut off = on.clone();
    off.voice.nerve_relay = false;

    let run = |cfg: &SimConfig| -> (u64, u64, i64) {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..40_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        let mut fp: i64 = 0;
        for e in &w.entities {
            fp = fp.wrapping_add((e.position.x * 97.0) as i64);
            fp = fp.wrapping_mul(1_000_003).wrapping_add((e.energy * 13.0) as i64);
        }
        (w.nerve_signals_relayed, w.entities.len() as u64, fp)
    };

    let (on_relayed, on_pop, on_fp) = run(&on);
    let (off_relayed, _off_pop, off_fp) = run(&off);

    assert!(on_relayed > 0, "le relais nerveux ne s'est jamais declenche en 40000 ticks (graine 1)");
    assert_eq!(off_relayed, 0, "nerve_relay = false relaie quand meme des alarmes");
    assert!(on_pop > 100, "l'ecosysteme s'eteint avec le relais nerveux (pop {on_pop})");
    assert_ne!(on_fp, off_fp, "le relais nerveux ne change rien a la trajectoire");
    assert_eq!(run(&on).0, on_relayed, "relais nerveux non deterministe");
}

#[test]
fn seasons_swing_the_world() {
    // Les saisons (config seulement, pas d'etat) : la regeneration des ressources oscille,
    // sinusoide pure du tick. `amplitude = 0` -> inerte, byte-identique. Sinon le monde
    // diverge et respire (plus de morts de faim, la disette mord). Deterministe.
    use genesis_core::sim::{season_phase, season_factor};

    // Bornes et forme de la sinusoide.
    let mut seasoned = ref_cfg();
    seasoned.season.amplitude = 0.5;
    seasoned.season.period_years = 1.5;
    assert_eq!(season_phase(&seasoned, 0), 0.0, "la saison ne demarre pas a l'intersaison");
    for t in [0u64, 137, 5000, 99999, 250_000] {
        let p = season_phase(&seasoned, t);
        assert!((-1.0..=1.0).contains(&p), "phase hors bornes a {t}");
        let f = season_factor(&seasoned, t);
        assert!(f >= seasoned.season.regen_floor && f <= 1.0 + seasoned.season.amplitude + 1e-6,
            "facteur de regen hors bornes a {t} : {f}");
    }

    // amplitude = 0 : byte-identique a un monde sans bloc [season].
    let plain = ref_cfg();
    let mut off = ref_cfg();
    off.season.amplitude = 0.0;
    let mut wa = WorldState::new(1, &plain);
    let mut wb = WorldState::new(1, &off);
    for _ in 0..8_000 {
        let _ = tick(&mut wa, &plain);
        let _ = tick(&mut wb, &off);
    }
    assert_eq!(state_json(&wa), state_json(&wb), "les saisons coupees ne sont pas inertes");

    // amplitude > 0 : le monde diverge du monde fige, et la disette tue davantage.
    let run = |cfg: &SimConfig| {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..60_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        (w.deaths_starvation, w.population(), state_json(&w))
    };
    let (flat_starv, _, flat_json) = run(&plain);
    let (seas_starv, seas_pop, seas_json) = run(&seasoned);
    assert!(seas_pop > 100, "le monde a saisons s'est effondre");
    assert_ne!(flat_json, seas_json, "les saisons ne changent rien au monde");
    assert!(
        seas_starv > flat_starv,
        "les saisons ne coutent rien : {seas_starv} morts de faim contre {flat_starv} sans"
    );

    // Deterministe : meme config, meme monde.
    let (a, _, _) = run(&seasoned);
    assert_eq!(a, seas_starv, "saisons non deterministes");
}

#[test]
fn heat_tolerance_is_selected() {
    // Le trait `heat_tol` (schema v18) : la temperature du monde selectionne l'adaptation
    // thermique. Un monde chaud pousse `heat_tol` vers le haut, un monde froid vers le bas.
    // `heat_tol_span_c = 0` coupe l'effet : le trait derive sans pression. Deterministe.
    const HEAT_TOL: usize = 9; // index dans le tableau de traits

    // Un monde froid pousse `heat_tol` vers le bas (adaptation au froid), un monde chaud vers
    // le haut. On force un gradient thermique net (`temp_metab_slope` releve) pour que l'effet
    // se lise sans ambiguite dans une fenetre courte ; le mecanisme est le meme au defaut.
    let run = |temp: f32, span: f32| -> (f32, u32) {
        let mut c = temp_cfg();
        c.planet.temperature_c = temp;
        c.planet.heat_tol_span_c = span;
        c.planet.temp_metab_slope = 0.03;
        c.resources.regen_rate = 0.03; // de quoi encaisser un gradient thermique fort
        let mut w = WorldState::new(1, &c);
        for _ in 0..60_000 {
            let _ = tick(&mut w, &c);
            if w.entities.is_empty() {
                break;
            }
        }
        (w.trait_stats().0[HEAT_TOL], w.population())
    };

    let (cold, cold_pop) = run(6.0, 24.0);
    let (warm, warm_pop) = run(24.0, 24.0);
    let (cold_inert, _) = run(6.0, 0.0);
    let (warm_inert, _) = run(24.0, 0.0);
    assert!(cold_pop > 100 && warm_pop > 100, "un monde thermique adapte s'est effondre");

    // L'adaptation thermique est reelle mais douce (la selection sur `heat_tol` passe par une
    // survie un peu meilleure, pas par plus de descendance) : un monde chaud tient `heat_tol`
    // plus haut qu'un monde froid, et l'ecart s'efface quand `span = 0`.
    let sep_active = warm - cold;
    let sep_inert = warm_inert - cold_inert;
    assert!(
        sep_active > 0.02,
        "la temperature ne separe pas l'adaptation thermique : chaud {warm:.3} vs froid {cold:.3} \
         (inerte : {warm_inert:.3} / {cold_inert:.3})"
    );
    assert!(
        sep_active > sep_inert.abs() + 0.015,
        "span 0 devrait annuler la separation thermique : active {sep_active:.3} vs inerte {sep_inert:.3}"
    );

    // Deterministe.
    assert_eq!(run(6.0, 24.0).0, cold, "selection thermique non deterministe");
}
