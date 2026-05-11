/// Integration tests for all five Blend improvements.
use lez_blend::{
    cover::{CoverConfig, CoverEngine},
    decoy::{DecoyBroadcast, DecoyConfig},
    delay::{
        sample_delays, sample_mean, sample_variance, AdaptiveDelay, DelayStrategy,
        ExponentialDelay, GeometricDelay, HybridDelay, ParetoDelay, PoissonDelay,
    },
    mix_select::{MixNode, VrfMixSelector},
    sphinx::{
        sphinx_unwrap, sphinx_wrap, SphinxHop, SphinxPacket,
        SPHINX_HOP_FIELD_SIZE, SPHINX_MAX_HOPS, SPHINX_PACKET_SIZE, SPHINX_PAYLOAD_SIZE,
    },
    BlendError,
};

use cryptarchia_lll::random_node_secret;
use rand::{rngs::OsRng, RngCore};

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn make_hop(seed: u8) -> (SphinxHop, [u8; 32]) {
    // Derive a deterministic X25519 key pair from a seed byte.
    let mut sk_bytes = [seed; 32];
    sk_bytes[0] ^= 0x5a; // add a bit of asymmetry
    let sk = x25519_dalek::StaticSecret::from(sk_bytes);
    let pk = x25519_dalek::PublicKey::from(&sk);
    let mut routing_id = [0u8; 16];
    routing_id[0] = seed;
    (
        SphinxHop { public_key: *pk.as_bytes(), routing_id },
        sk_bytes,
    )
}

fn make_n_hops(n: usize) -> (Vec<SphinxHop>, Vec<[u8; 32]>) {
    (0..n as u8).map(|i| make_hop(i + 1)).unzip()
}

fn make_mix_nodes(n: usize) -> Vec<MixNode> {
    (0..n)
        .map(|i| {
            let (hop, _) = make_hop(i as u8 + 10);
            let mut id = [0u8; 32];
            id[0] = i as u8;
            MixNode {
                id,
                x25519_public_key: hop.public_key,
                stake_weight: (i as u64 + 1) * 1000,
                label: alloc::format!("node-{}", i),
            }
        })
        .collect()
}

extern crate alloc;

// ─────────────────────────────────────────────────────────────────────────────
// A. Sphinx packet tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn sphinx_packet_size_is_constant() {
    let (hops, _) = make_n_hops(3);
    let pkt = sphinx_wrap(&hops, b"hello blend", &mut OsRng).unwrap();
    assert_eq!(pkt.to_bytes().len(), SPHINX_PACKET_SIZE);
}

#[test]
fn sphinx_packet_size_invariant_to_hop_count() {
    let (hops1, _) = make_n_hops(1);
    let (hops3, _) = make_n_hops(3);
    let (hops5, _) = make_n_hops(5);
    let msg = b"same size regardless of path length";
    let p1 = sphinx_wrap(&hops1, msg, &mut OsRng).unwrap();
    let p3 = sphinx_wrap(&hops3, msg, &mut OsRng).unwrap();
    let p5 = sphinx_wrap(&hops5, msg, &mut OsRng).unwrap();
    assert_eq!(p1.to_bytes().len(), SPHINX_PACKET_SIZE);
    assert_eq!(p3.to_bytes().len(), SPHINX_PACKET_SIZE);
    assert_eq!(p5.to_bytes().len(), SPHINX_PACKET_SIZE);
}

#[test]
fn sphinx_wrap_unwrap_single_hop() {
    let (hops, sks) = make_n_hops(1);
    let plaintext = b"single hop message";
    let pkt = sphinx_wrap(&hops, plaintext, &mut OsRng).unwrap();

    let (routing_id, inner) = sphinx_unwrap(&pkt, &sks[0]).unwrap();
    // Single hop: routing_id encodes hops[0].routing_id
    assert_eq!(routing_id, hops[0].routing_id);
    // Inner payload starts with the plaintext (rest is zero padding)
    assert_eq!(&inner.payload[..plaintext.len()], plaintext);
}

#[test]
fn sphinx_wrap_unwrap_three_hops() {
    let (hops, sks) = make_n_hops(3);
    let plaintext = b"three hop message through the blend network";
    let pkt = sphinx_wrap(&hops, plaintext, &mut OsRng).unwrap();

    // Hop 0 (entry mix)
    let (routing_id_0, pkt1) = sphinx_unwrap(&pkt, &sks[0]).unwrap();
    assert_eq!(routing_id_0, hops[0].routing_id);

    // Hop 1 (middle mix)
    let (routing_id_1, pkt2) = sphinx_unwrap(&pkt1, &sks[1]).unwrap();
    assert_eq!(routing_id_1, hops[1].routing_id);

    // Hop 2 (exit / destination)
    let (routing_id_2, pkt3) = sphinx_unwrap(&pkt2, &sks[2]).unwrap();
    assert_eq!(routing_id_2, hops[2].routing_id);

    // Final payload must start with the plaintext
    assert_eq!(&pkt3.payload[..plaintext.len()], plaintext);
}

#[test]
fn sphinx_wrap_unwrap_five_hops() {
    let (hops, sks) = make_n_hops(5);
    let plaintext = b"five hop maximum-path message";
    let pkt = sphinx_wrap(&hops, plaintext, &mut OsRng).unwrap();
    let mut current = pkt;
    for (i, sk) in sks.iter().enumerate() {
        let (rid, next) = sphinx_unwrap(&current, sk).unwrap();
        assert_eq!(rid, hops[i].routing_id);
        current = next;
    }
    assert_eq!(&current.payload[..plaintext.len()], plaintext);
}

#[test]
fn sphinx_wrong_key_fails_mac() {
    let (hops, _) = make_n_hops(2);
    let pkt = sphinx_wrap(&hops, b"secret", &mut OsRng).unwrap();
    let wrong_sk = [0xdeu8; 32];
    let result = sphinx_unwrap(&pkt, &wrong_sk);
    assert!(
        matches!(result, Err(BlendError::MacMismatch) | Err(BlendError::AeadDecryptFailed)),
        "Wrong key must fail: {:?}",
        result
    );
}

#[test]
fn sphinx_rejects_too_many_hops() {
    let (hops, _) = make_n_hops(SPHINX_MAX_HOPS + 1);
    let result = sphinx_wrap(&hops, b"overflow", &mut OsRng);
    assert!(matches!(result, Err(BlendError::TooManyHops { .. })));
}

#[test]
fn sphinx_rejects_empty_hops() {
    let result = sphinx_wrap(&[], b"no hops", &mut OsRng);
    assert!(matches!(result, Err(BlendError::EmptyHops)));
}

#[test]
fn sphinx_bytes_roundtrip() {
    let (hops, _) = make_n_hops(2);
    let pkt = sphinx_wrap(&hops, b"roundtrip", &mut OsRng).unwrap();
    let bytes = pkt.to_bytes();
    let pkt2 = SphinxPacket::from_bytes(&bytes);
    assert_eq!(pkt, pkt2);
}

#[test]
fn sphinx_two_packets_have_different_alpha() {
    let (hops, _) = make_n_hops(2);
    let p1 = sphinx_wrap(&hops, b"msg", &mut OsRng).unwrap();
    let p2 = sphinx_wrap(&hops, b"msg", &mut OsRng).unwrap();
    assert_ne!(p1.alpha, p2.alpha, "Each packet must have a fresh ephemeral key");
}

#[test]
fn sphinx_packets_are_indistinguishable_by_size() {
    let (hops2, _) = make_n_hops(2);
    let (hops4, _) = make_n_hops(4);
    let p2 = sphinx_wrap(&hops2, b"short", &mut OsRng).unwrap().to_bytes();
    let p4 = sphinx_wrap(&hops4, b"short", &mut OsRng).unwrap().to_bytes();
    assert_eq!(p2.len(), p4.len(), "Different path lengths must produce the same wire size");
}

// ─────────────────────────────────────────────────────────────────────────────
// B. Delay strategy tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn exponential_delay_mean_within_tolerance() {
    let mut d = ExponentialDelay::new(100.0, 0, 10_000);
    let samples = sample_delays(&mut d, 2_000);
    let mean = sample_mean(&samples);
    // Sample mean should be within 20% of the configured mean for 2000 samples.
    assert!(
        mean > 75.0 && mean < 140.0,
        "ExponentialDelay mean {mean:.1} ms is outside expected range [75, 140] for μ=100 ms"
    );
}

#[test]
fn exponential_delay_respects_min_max() {
    let mut d = ExponentialDelay::new(50.0, 10, 200);
    let samples = sample_delays(&mut d, 500);
    assert!(samples.iter().all(|&v| v >= 10 && v <= 200));
}

#[test]
fn poisson_delay_mean_within_tolerance() {
    // rate = 10 Hz → mean inter-arrival = 100 ms
    let mut d = PoissonDelay::new(10.0, 0, 10_000);
    let samples = sample_delays(&mut d, 2_000);
    let mean = sample_mean(&samples);
    assert!(
        mean > 75.0 && mean < 140.0,
        "PoissonDelay mean {mean:.1} ms outside expected range for rate=10 Hz"
    );
}

#[test]
fn hybrid_delay_larger_than_poisson_alone() {
    let mut poisson = PoissonDelay::new(10.0, 0, 10_000);
    let mut hybrid = HybridDelay::new(10.0, 200, 0, 10_000);
    let p_mean = sample_mean(&sample_delays(&mut poisson, 1_000));
    let h_mean = sample_mean(&sample_delays(&mut hybrid, 1_000));
    // Hybrid adds up to 200 ms uniform jitter so its mean must be higher.
    assert!(
        h_mean > p_mean,
        "Hybrid mean {h_mean:.1} should exceed Poisson mean {p_mean:.1} due to additive jitter"
    );
}

#[test]
fn hybrid_delay_respects_bounds() {
    let mut d = HybridDelay::new(5.0, 50, 20, 500);
    let samples = sample_delays(&mut d, 500);
    assert!(samples.iter().all(|&v| v >= 20 && v <= 500));
}

#[test]
fn delay_strategy_names_are_unique() {
    let names = [
        ExponentialDelay::new(100.0, 0, 5000).strategy_name(),
        PoissonDelay::new(10.0, 0, 5000).strategy_name(),
        HybridDelay::new(10.0, 50, 0, 5000).strategy_name(),
        GeometricDelay::new(0.1, 50, 0, 5000).strategy_name(),
        ParetoDelay::new(2.5, 60.0, 0, 5000).strategy_name(),
        AdaptiveDelay::new(0, 5000).strategy_name(),
    ];
    for i in 0..names.len() {
        for j in i + 1..names.len() {
            assert_ne!(
                names[i], names[j],
                "strategies {} and {} share the name {:?}",
                i, j, names[i]
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// B2. Exponential jitter (Dandelion BIP-156)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn exponential_jitter_mean_stays_close_to_configured() {
    // Jitter is symmetric around zero, so it must not bias the mean.
    let mut d = ExponentialDelay::with_jitter(200.0, 0, 10_000, 0.10);
    let samples = sample_delays(&mut d, 3_000);
    let mean = sample_mean(&samples);
    assert!(
        mean > 140.0 && mean < 280.0,
        "Jittered ExponentialDelay mean {mean:.1} ms too far from μ=200"
    );
}

#[test]
fn exponential_jitter_respects_min_max() {
    let mut d = ExponentialDelay::with_jitter(100.0, 20, 300, 0.20);
    let samples = sample_delays(&mut d, 500);
    assert!(
        samples.iter().all(|&v| v >= 20 && v <= 300),
        "Jittered ExponentialDelay violated bounds"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// B3. Geometric delay
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn geometric_delay_mean_within_tolerance() {
    // p = 0.1, slot = 100 ms → theoretical mean = 100 / 0.1 = 1000 ms
    let mut d = GeometricDelay::new(0.1, 100, 0, 50_000);
    let samples = sample_delays(&mut d, 3_000);
    let mean = sample_mean(&samples);
    // Allow ±35 % for 3 000 samples of a heavy-tailed geometric.
    assert!(
        mean > 650.0 && mean < 1_350.0,
        "GeometricDelay mean {mean:.1} ms outside expected range [650, 1350] for μ=1000 ms"
    );
}

#[test]
fn geometric_delay_respects_min_max() {
    let mut d = GeometricDelay::new(0.2, 50, 10, 400);
    let samples = sample_delays(&mut d, 500);
    assert!(
        samples.iter().all(|&v| v >= 10 && v <= 400),
        "GeometricDelay violated bounds"
    );
}

#[test]
fn geometric_delay_is_discrete_multiples_of_slot() {
    // Without clamping, all delays must be a whole multiple of slot_ms.
    let slot = 17u64;
    let mut d = GeometricDelay::new(0.3, slot, 0, 1_000_000);
    let samples = sample_delays(&mut d, 200);
    for &v in &samples {
        assert_eq!(v % slot, 0, "GeometricDelay produced {v} which is not a multiple of {slot}");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// B4. Pareto delay
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn pareto_delay_mean_within_tolerance() {
    // shape=2.5, scale=60 ms → theoretical mean = 60 * 2.5 / (2.5-1) = 100 ms
    let mut d = ParetoDelay::new(2.5, 60.0, 0, 50_000);
    let samples = sample_delays(&mut d, 5_000);
    let mean = sample_mean(&samples);
    // Allow ±40 % because the heavy tail inflates variance.
    assert!(
        mean > 60.0 && mean < 160.0,
        "ParetoDelay mean {mean:.1} ms outside expected range [60, 160]"
    );
}

#[test]
fn pareto_delay_is_heavy_tailed() {
    // The hallmark of a heavy-tailed distribution is that the sample maximum
    // vastly exceeds the sample mean.  For Pareto(shape=1.5, scale=30) with
    // 5000 draws the expected maximum is scale × n^(1/shape) ≈ 30 × 292 ≈ 8700 ms
    // while the theoretical mean is 90 ms.  We test conservatively for > 20×.
    let n = 5_000;
    let mut d = ParetoDelay::new(1.5, 30.0, 0, 50_000_000);
    let samples = sample_delays(&mut d, n);
    let mean = sample_mean(&samples);
    let max = *samples.iter().max().unwrap() as f64;
    assert!(
        max > mean * 20.0,
        "Pareto max ({max:.0} ms) should be > 20× mean ({mean:.0} ms) — heavy-tail signature"
    );
}

#[test]
fn pareto_delay_respects_min_max() {
    let mut d = ParetoDelay::new(3.0, 50.0, 10, 800);
    let samples = sample_delays(&mut d, 500);
    assert!(
        samples.iter().all(|&v| v >= 10 && v <= 800),
        "ParetoDelay violated bounds"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// B5. Adaptive (ML-resistant) delay
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn adaptive_delay_rotates_strategy() {
    let mut d = AdaptiveDelay::new(0, 10_000);
    let initial_idx = d.current_strategy_index();
    // The switch_every is 50 (±25 %), so max first switch is at sample 63.
    // Drawing 100 samples guarantees at least one rotation.
    for _ in 0..100 {
        d.next_delay_ms();
    }
    assert_ne!(
        initial_idx,
        d.current_strategy_index(),
        "AdaptiveDelay should have rotated after 100 samples"
    );
}

#[test]
fn adaptive_delay_cycles_through_all_four_strategies() {
    let mut d = AdaptiveDelay::new(0, 10_000);
    let mut seen = alloc::collections::BTreeSet::new();
    // 400 samples = ~8 switch intervals; enough to see all 4 roster slots.
    for _ in 0..400 {
        seen.insert(d.current_strategy_index());
        d.next_delay_ms();
    }
    assert_eq!(seen.len(), 4, "Expected all 4 strategies to be visited, got {:?}", seen);
}

#[test]
fn adaptive_delay_respects_bounds() {
    let mut d = AdaptiveDelay::new(10, 500);
    let samples = sample_delays(&mut d, 500);
    assert!(
        samples.iter().all(|&v| v >= 10 && v <= 500),
        "AdaptiveDelay violated bounds"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// C. Cover traffic tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn cover_engine_generates_sphinx_packet() {
    let (hops, _) = make_n_hops(3);
    let mut engine = CoverEngine::new(CoverConfig::default());
    let pkt = engine.next_packet(&hops).unwrap();
    assert_eq!(pkt.to_bytes().len(), SPHINX_PACKET_SIZE);
}

#[test]
fn cover_engine_generates_batch() {
    let (hops, _) = make_n_hops(4);
    let mut engine = CoverEngine::new(CoverConfig { rate_hz: 2.0, min_hops: 2, max_hops: 3, enabled: true });
    let batch = engine.generate_batch(&hops, 10).unwrap();
    assert_eq!(batch.len(), 10);
    for pkt in &batch {
        assert_eq!(pkt.to_bytes().len(), SPHINX_PACKET_SIZE);
    }
}

#[test]
fn cover_engine_packets_are_distinct() {
    let (hops, _) = make_n_hops(3);
    let mut engine = CoverEngine::new(CoverConfig::default());
    let p1 = engine.next_packet(&hops).unwrap();
    let p2 = engine.next_packet(&hops).unwrap();
    // Two cover packets must use different ephemeral keys (alpha).
    assert_ne!(p1.alpha, p2.alpha);
}

#[test]
fn cover_engine_disabled_does_not_tick() {
    let mut engine = CoverEngine::new(CoverConfig {
        rate_hz: 1.0,
        min_hops: 1,
        max_hops: 2,
        enabled: false,
    });
    // Even after a very large elapsed time, disabled engine should not tick.
    assert!(!engine.tick(1_000_000));
}

#[test]
fn cover_engine_ticks_after_interval() {
    let mut engine = CoverEngine::new(CoverConfig {
        rate_hz: 1000.0, // 1 packet/ms → very short expected interval
        min_hops: 1,
        max_hops: 2,
        enabled: true,
    });
    // After 10 000 ms elapsed, the engine must have ticked.
    assert!(engine.tick(10_000));
}

#[test]
fn cover_engine_rejects_empty_nodes() {
    let mut engine = CoverEngine::new(CoverConfig::default());
    let result = engine.next_packet(&[]);
    assert!(matches!(result, Err(BlendError::EmptyHops)));
}

// ─────────────────────────────────────────────────────────────────────────────
// D. VRF mix selection tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn vrf_select_returns_correct_count() {
    let nodes = make_mix_nodes(10);
    let secret = random_node_secret();
    let path = VrfMixSelector::select_path(&nodes, &secret, b"nonce-test", 3).unwrap();
    assert_eq!(path.hops.len(), 3);
}

#[test]
fn vrf_select_path_is_deterministic() {
    let nodes = make_mix_nodes(10);
    let secret = random_node_secret();
    let nonce = b"deterministic-nonce";
    let p1 = VrfMixSelector::select_path(&nodes, &secret, nonce, 3).unwrap();
    let p2 = VrfMixSelector::select_path(&nodes, &secret, nonce, 3).unwrap();
    let ids1: Vec<_> = p1.hops.iter().map(|n| n.id).collect();
    let ids2: Vec<_> = p2.hops.iter().map(|n| n.id).collect();
    assert_eq!(ids1, ids2, "Same nonce and secret must produce the same path");
}

#[test]
fn vrf_select_path_differs_with_different_nonce() {
    let nodes = make_mix_nodes(10);
    let secret = random_node_secret();
    let p1 = VrfMixSelector::select_path(&nodes, &secret, b"nonce-A", 4).unwrap();
    let p2 = VrfMixSelector::select_path(&nodes, &secret, b"nonce-B", 4).unwrap();
    let ids1: Vec<_> = p1.hops.iter().map(|n| n.id).collect();
    let ids2: Vec<_> = p2.hops.iter().map(|n| n.id).collect();
    assert_ne!(ids1, ids2, "Different nonces should produce different paths");
}

#[test]
fn vrf_select_path_differs_with_different_secret() {
    let nodes = make_mix_nodes(10);
    let s1 = random_node_secret();
    let s2 = random_node_secret();
    let nonce = b"same-nonce";
    let p1 = VrfMixSelector::select_path(&nodes, &s1, nonce, 4).unwrap();
    let p2 = VrfMixSelector::select_path(&nodes, &s2, nonce, 4).unwrap();
    let ids1: Vec<_> = p1.hops.iter().map(|n| n.id).collect();
    let ids2: Vec<_> = p2.hops.iter().map(|n| n.id).collect();
    assert_ne!(ids1, ids2, "Different secrets must produce different paths");
}

#[test]
fn vrf_select_path_no_duplicates() {
    let nodes = make_mix_nodes(10);
    let secret = random_node_secret();
    for trial in 0u64..10 {
        let nonce = trial.to_le_bytes();
        let path = VrfMixSelector::select_path(&nodes, &secret, &nonce, 5).unwrap();
        let mut ids: Vec<_> = path.hops.iter().map(|n| n.id).collect();
        let before = ids.len();
        ids.dedup();
        assert_eq!(before, ids.len(), "Path must not contain duplicate nodes");
    }
}

#[test]
fn vrf_select_rejects_insufficient_nodes() {
    let nodes = make_mix_nodes(2);
    let secret = random_node_secret();
    let result = VrfMixSelector::select_path(&nodes, &secret, b"n", 5);
    assert!(matches!(result, Err(BlendError::InsufficientMixNodes { .. })));
}

#[test]
fn vrf_select_proof_is_populated() {
    let nodes = make_mix_nodes(5);
    let secret = random_node_secret();
    let path = VrfMixSelector::select_path(&nodes, &secret, b"nonce", 3).unwrap();
    assert!(!path.vrf_proof_bytes.is_empty());
    assert_eq!(path.vrf_output.len(), 32);
}

#[test]
fn vrf_to_sphinx_hops_matches_path_len() {
    let nodes = make_mix_nodes(6);
    let secret = random_node_secret();
    let path = VrfMixSelector::select_path(&nodes, &secret, b"n", 4).unwrap();
    let hops = VrfMixSelector::to_sphinx_hops(&path);
    assert_eq!(hops.len(), 4);
}

// ─────────────────────────────────────────────────────────────────────────────
// E. Decoy broadcast tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn decoy_generates_correct_fan_out() {
    let nodes = make_mix_nodes(8);
    let secret = random_node_secret();
    let (hops, _) = make_n_hops(2);
    let real_pkt = sphinx_wrap(&hops, b"real proposal", &mut OsRng).unwrap();

    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 5, hop_count: 2 });
    let batch = decoy.wrap_with_decoys(real_pkt.clone(), &nodes, &secret, b"nonce-0").unwrap();
    assert_eq!(batch.packets.len(), 5, "fan_out=5 must produce 5 decoys");
}

#[test]
fn decoy_packets_match_real_packet_size() {
    let nodes = make_mix_nodes(6);
    let secret = random_node_secret();
    let (hops, _) = make_n_hops(2);
    let real_pkt = sphinx_wrap(&hops, b"proposal body", &mut OsRng).unwrap();
    let real_size = real_pkt.to_bytes().len();

    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 4, hop_count: 2 });
    let batch = decoy.wrap_with_decoys(real_pkt, &nodes, &secret, b"nonce-1").unwrap();
    for pkt in &batch.packets {
        assert_eq!(pkt.to_bytes().len(), real_size, "All packets (real + decoy) must be the same size");
    }
}

#[test]
fn decoy_disabled_produces_no_decoys() {
    let nodes = make_mix_nodes(4);
    let secret = random_node_secret();
    let (hops, _) = make_n_hops(1);
    let real_pkt = sphinx_wrap(&hops, b"real", &mut OsRng).unwrap();

    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: false, fan_out: 10, hop_count: 2 });
    let batch = decoy.wrap_with_decoys(real_pkt, &nodes, &secret, b"n").unwrap();
    assert!(batch.packets.is_empty(), "Disabled decoy module must produce no decoys");
}

#[test]
fn decoy_real_packet_preserved_in_batch() {
    let nodes = make_mix_nodes(5);
    let secret = random_node_secret();
    let (hops, _) = make_n_hops(2);
    let real_pkt = sphinx_wrap(&hops, b"the real one", &mut OsRng).unwrap();

    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 3, hop_count: 2 });
    let batch = decoy.wrap_with_decoys(real_pkt.clone(), &nodes, &secret, b"n2").unwrap();
    assert_eq!(batch.real_packet, real_pkt, "Real packet must be preserved verbatim in the batch");
}

#[test]
fn decoy_shuffle_contains_real_packet() {
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    let nodes = make_mix_nodes(5);
    let secret = random_node_secret();
    let (hops, _) = make_n_hops(2);
    let real_pkt = sphinx_wrap(&hops, b"real packet", &mut OsRng).unwrap();

    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 4, hop_count: 2 });
    let batch = decoy.wrap_with_decoys(real_pkt.clone(), &nodes, &secret, b"nonce-shuffle").unwrap();

    let mut rng = SmallRng::from_entropy();
    let shuffled = batch.shuffle_all(&mut rng);
    assert_eq!(shuffled.len(), 5, "Shuffle must produce fan_out+1 = 5 packets");
    assert!(
        shuffled.contains(&real_pkt),
        "Shuffled batch must contain the real packet"
    );
}

#[test]
fn decoy_raw_decoys_correct_count() {
    let (hops, _) = make_n_hops(4);
    let mut db = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 3, hop_count: 2 });
    let decoys = db.generate_raw_decoys(&hops, 7).unwrap();
    assert_eq!(decoys.len(), 7);
}

// ─────────────────────────────────────────────────────────────────────────────
// F. End-to-end: VRF path → Sphinx → Decoy broadcast
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn full_pipeline_vrf_path_sphinx_wrap_decoy() {
    let nodes = make_mix_nodes(8);
    let secret = random_node_secret();
    let plaintext = b"block proposal payload for epoch 42 slot 7";

    // 1. Select a 3-hop path via VRF
    let path = VrfMixSelector::select_path(&nodes, &secret, b"epoch42-slot7", 3).unwrap();
    let hops = VrfMixSelector::to_sphinx_hops(&path);
    assert_eq!(hops.len(), 3);

    // 2. Wrap the proposal in a Sphinx packet
    let real_pkt = sphinx_wrap(&hops, plaintext, &mut OsRng).unwrap();
    assert_eq!(real_pkt.to_bytes().len(), SPHINX_PACKET_SIZE);

    // 3. Generate decoy packets
    let mut decoy = DecoyBroadcast::new(DecoyConfig { enabled: true, fan_out: 5, hop_count: 2 });
    let batch = decoy
        .wrap_with_decoys(real_pkt, &nodes, &secret, b"epoch42-slot7")
        .unwrap();
    assert_eq!(batch.packets.len(), 5);

    // 4. Verify all packets are the same size (network indistinguishability)
    let expected_size = batch.real_packet.to_bytes().len();
    for pkt in &batch.packets {
        assert_eq!(pkt.to_bytes().len(), expected_size);
    }
}

#[test]
fn full_pipeline_unwrap_after_vrf_path() {
    let nodes = make_mix_nodes(5);
    let secret = random_node_secret();

    // Build hop list from nodes 0,1,2
    let sub_nodes: Vec<_> = nodes[..3].to_vec();
    let path = VrfMixSelector::select_path(&sub_nodes, &secret, b"test-nonce", 3).unwrap();
    let hops = VrfMixSelector::to_sphinx_hops(&path);

    // Re-derive the X25519 secret keys for those nodes
    let sk_for = |node: &lez_blend::mix_select::MixNode| -> [u8; 32] {
        // In the test setup make_mix_nodes uses seed = index + 10
        let seed = node.id[0] + 10;
        let mut sk_bytes = [seed; 32];
        sk_bytes[0] ^= 0x5a;
        sk_bytes
    };

    let plaintext = b"vrf-routed message";
    let pkt = sphinx_wrap(&hops, plaintext, &mut OsRng).unwrap();

    // Unwrap through each hop in order
    let mut current = pkt;
    for hop_node in &path.hops {
        let sk = sk_for(hop_node);
        let (_, next) = sphinx_unwrap(&current, &sk).unwrap();
        current = next;
    }
    assert_eq!(&current.payload[..plaintext.len()], plaintext);
}
