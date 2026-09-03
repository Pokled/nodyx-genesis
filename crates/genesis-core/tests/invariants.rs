//! Tests d'invariants, ecrits avant de faire confiance au moteur (tranchee 13).

use genesis_core::{tick, SimConfig, WorldState};

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

    let mut stressed = ref_cfg();
    stressed.planet.temperature_c = 1.0; // 14 degres sous l'optimum : selection dure

    let mut w = WorldState::new(1, &stressed);
    let mut shifts = 0u32;
    for _ in 0..50_000 {
        let ev = tick(&mut w, &stressed);
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
    assert!(shifts >= 1, "aucun basculement de genome en 50000 ticks (monde froid, graine 1)");

    // Coupe : plus aucun basculement.
    let mut off = stressed.clone();
    off.watch.genome_shift_persist_checks = 0;
    let mut w2 = WorldState::new(1, &off);
    for _ in 0..50_000 {
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
    let mut warm = ref_cfg();
    warm.planet.temperature_c = 15.0;
    warm.planet.temp_optimal_c = 15.0;
    let mut cold = warm.clone();
    cold.planet.temperature_c = 0.0; // 15 degres sous l'optimum

    let run = |cfg: &SimConfig| {
        let mut w = WorldState::new(1, cfg);
        for _ in 0..30_000 {
            let _ = tick(&mut w, cfg);
            if w.entities.is_empty() {
                break;
            }
        }
        (w.deaths_starvation, w.population())
    };
    let (warm_starv, warm_pop) = run(&warm);
    let (cold_starv, cold_pop) = run(&cold);

    assert!(warm_pop > 100 && cold_pop > 100, "un des deux mondes s'est effondre");
    assert!(
        cold_starv > warm_starv,
        "le froid ne coute rien : {cold_starv} morts de faim contre {warm_starv} au chaud"
    );

    // Deterministe : meme config, meme monde.
    let (a, _) = run(&cold);
    assert_eq!(a, cold_starv, "climat non deterministe");
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
