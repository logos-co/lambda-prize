/// Improvement B — Machine-learning-resistant adaptive timing delays.
///
/// Timing analysis is one of the strongest de-anonymisation tools against
/// mixnets.  Fixed delays are easy to subtract; uniform-random delays have
/// fingerprints learnable by neural classifiers.  This module provides a
/// `DelayStrategy` trait and five concrete implementations that are resistant
/// to statistical and ML-based timing attacks:
///
/// 1. [`ExponentialDelay`] — delays drawn from Exp(λ); provably maximum-entropy
///    under a mean constraint (Chaum 1981).  Supports optional ±`jitter_fraction`
///    additive noise (Dandelion BIP-156 style) for extra timing decorrelation.
///
/// 2. [`PoissonDelay`] — inter-arrival times of a Poisson process; ergonomically
///    equivalent to `ExponentialDelay` but parameterised by a rate in Hz.
///
/// 3. [`HybridDelay`] — Poisson baseline plus independent uniform jitter; adds a
///    second source of randomness to break lag-correlation.
///
/// 4. [`GeometricDelay`] — a discrete delay: number of Bernoulli trials × `slot_ms`.
///    The discretisation makes the distribution harder for a continuous-valued ML
///    model to fit, while the geometric tail maintains good anonymity properties.
///
/// 5. [`ParetoDelay`] — heavy-tailed Pareto distribution.  Rare large values
///    distort any learned decision boundary and force an adversary to handle
///    extreme cases, degrading classifier confidence significantly.
///
/// 6. [`AdaptiveDelay`] — ML-resistant strategy that cycles through the four
///    distributions above on a jittered schedule, so no single pattern can be
///    learnt for long.
///
/// All value-producing implementations are `no_std`-compatible: they return a
/// delay in milliseconds without sleeping; callers apply the delay themselves.
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
extern crate alloc;

/// A source of per-packet forwarding delays (in milliseconds).
///
/// Implementors must be `Send + Sync` so they can be held in async tasks.
pub trait DelayStrategy: Send + Sync {
    /// Draw the next delay value in milliseconds.
    fn next_delay_ms(&mut self) -> u64;

    /// Human-readable name for logging / telemetry.
    fn strategy_name(&self) -> &'static str;
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. Exponential delay
// ─────────────────────────────────────────────────────────────────────────────

/// Delays drawn from an Exponential distribution with the given mean.
///
/// Samples are generated via the inverse-CDF method: `x = −μ · ln(U)` where
/// `U ~ Uniform(0, 1)`.  This maximises entropy among all non-negative
/// distributions with the same mean, making it hardest to distinguish from a
/// "natural" inter-arrival time.
///
/// An optional uniform `jitter_fraction` (Dandelion BIP-156 style) adds
/// symmetric noise in `[−fraction·μ, +fraction·μ]` to each sample, decorrelating
/// consecutive delays from each other.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialDelay {
    /// Mean delay in milliseconds (μ = 1/λ).
    pub mean_ms: f64,
    /// Minimum delay clamp (prevents pathological zero delays).
    pub min_ms: u64,
    /// Maximum delay clamp (prevents queue starvation).
    pub max_ms: u64,
    /// Fraction of mean_ms used as ±jitter amplitude.  0.0 = no jitter.
    pub jitter_fraction: f64,
    #[serde(skip)]
    rng: Option<SmallRng>,
}

impl ExponentialDelay {
    /// Pure exponential delay with no additive jitter.
    pub fn new(mean_ms: f64, min_ms: u64, max_ms: u64) -> Self {
        assert!(mean_ms > 0.0, "mean_ms must be positive");
        assert!(min_ms <= max_ms, "min_ms must be <= max_ms");
        Self {
            mean_ms,
            min_ms,
            max_ms,
            jitter_fraction: 0.0,
            rng: Some(SmallRng::from_entropy()),
        }
    }

    /// Exponential delay with Dandelion-style ±`jitter_fraction` · `mean_ms`
    /// additive noise per sample.
    ///
    /// A value of `0.10` adds ±10 % noise, which is sufficient to break
    /// timing-correlation attacks while keeping the mean within < 1 % of the
    /// configured value for large sample sizes.
    pub fn with_jitter(mean_ms: f64, min_ms: u64, max_ms: u64, jitter_fraction: f64) -> Self {
        assert!(mean_ms > 0.0, "mean_ms must be positive");
        assert!(min_ms <= max_ms, "min_ms must be <= max_ms");
        assert!(
            (0.0..=1.0).contains(&jitter_fraction),
            "jitter_fraction must be in [0, 1]"
        );
        Self {
            mean_ms,
            min_ms,
            max_ms,
            jitter_fraction,
            rng: Some(SmallRng::from_entropy()),
        }
    }

    fn rng_mut(&mut self) -> &mut SmallRng {
        self.rng.get_or_insert_with(SmallRng::from_entropy)
    }
}

impl DelayStrategy for ExponentialDelay {
    fn next_delay_ms(&mut self) -> u64 {
        let mean = self.mean_ms;
        let u: f64 = self.rng_mut().gen_range(1e-15_f64..1.0_f64);
        let mut sample = (-u.ln()) * mean;
        if self.jitter_fraction > 0.0 {
            let half = mean * self.jitter_fraction;
            let j: f64 = self.rng_mut().gen_range(-half..=half);
            sample += j;
        }
        (sample.max(0.0) as u64).clamp(self.min_ms, self.max_ms)
    }

    fn strategy_name(&self) -> &'static str { "exponential" }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Poisson delay
// ─────────────────────────────────────────────────────────────────────────────

/// Delays drawn from a Poisson process with arrival rate `rate_hz`.
///
/// A Poisson process has exponentially-distributed inter-arrival times with
/// mean `1000 / rate_hz` ms.  This is equivalent to `ExponentialDelay` with
/// `mean_ms = 1000 / rate_hz` and is provided for API ergonomics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoissonDelay {
    /// Packet emission rate in packets per second (λ).
    pub rate_hz: f64,
    pub min_ms: u64,
    pub max_ms: u64,
    #[serde(skip)]
    inner: Option<ExponentialDelay>,
}

impl PoissonDelay {
    pub fn new(rate_hz: f64, min_ms: u64, max_ms: u64) -> Self {
        assert!(rate_hz > 0.0, "rate_hz must be positive");
        let mean_ms = 1000.0 / rate_hz;
        Self {
            rate_hz,
            min_ms,
            max_ms,
            inner: Some(ExponentialDelay::new(mean_ms, min_ms, max_ms)),
        }
    }

    fn inner_mut(&mut self) -> &mut ExponentialDelay {
        self.inner.get_or_insert_with(|| {
            ExponentialDelay::new(1000.0 / self.rate_hz, self.min_ms, self.max_ms)
        })
    }
}

impl DelayStrategy for PoissonDelay {
    fn next_delay_ms(&mut self) -> u64 { self.inner_mut().next_delay_ms() }
    fn strategy_name(&self) -> &'static str { "poisson" }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Hybrid delay
// ─────────────────────────────────────────────────────────────────────────────

/// Hybrid = Poisson baseline + independent uniform jitter.
///
/// Adding an independent jitter component means an adversary must jointly
/// model two separate distributions; the cross-correlation of observed delays
/// to true delays is strictly lower than for a single-source strategy.
///
/// `delay = Poisson(rate_hz) + Uniform(0, jitter_ms)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridDelay {
    pub rate_hz: f64,
    /// Maximum additive uniform jitter in milliseconds.
    pub jitter_ms: u64,
    pub min_ms: u64,
    pub max_ms: u64,
    #[serde(skip)]
    poisson: Option<PoissonDelay>,
    #[serde(skip)]
    rng: Option<SmallRng>,
}

impl HybridDelay {
    pub fn new(rate_hz: f64, jitter_ms: u64, min_ms: u64, max_ms: u64) -> Self {
        Self {
            rate_hz,
            jitter_ms,
            min_ms,
            max_ms,
            poisson: Some(PoissonDelay::new(rate_hz, 0, max_ms)),
            rng: Some(SmallRng::from_entropy()),
        }
    }

    fn poisson_mut(&mut self) -> &mut PoissonDelay {
        self.poisson
            .get_or_insert_with(|| PoissonDelay::new(self.rate_hz, 0, self.max_ms))
    }

    fn rng_mut(&mut self) -> &mut SmallRng {
        self.rng.get_or_insert_with(SmallRng::from_entropy)
    }
}

impl DelayStrategy for HybridDelay {
    fn next_delay_ms(&mut self) -> u64 {
        let base = self.poisson_mut().next_delay_ms();
        let jitter_cap = self.jitter_ms;
        let min_ms = self.min_ms;
        let max_ms = self.max_ms;
        let jitter = if jitter_cap > 0 {
            self.rng_mut().gen_range(0..=jitter_cap)
        } else {
            0
        };
        base.saturating_add(jitter).clamp(min_ms, max_ms)
    }

    fn strategy_name(&self) -> &'static str { "hybrid" }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Geometric delay
// ─────────────────────────────────────────────────────────────────────────────

/// Delays drawn from a Geometric distribution.
///
/// The Geometric distribution models the number of independent Bernoulli
/// trials needed to achieve one success with success probability `p`.  Each
/// trial corresponds to `slot_ms` milliseconds, giving a discrete delay of
/// `k × slot_ms` with mean `slot_ms / p`.
///
/// The discretisation makes the distribution harder for a continuous-valued
/// ML classifier to learn than a smooth exponential, while the geometric tail
/// still provides good max-entropy properties under a mean constraint.
///
/// Samples are generated via the inverse-CDF method:
/// `k = ⌈ ln(U) / ln(1 − p) ⌉` where `U ~ Uniform(0, 1)`, `k ≥ 1`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricDelay {
    /// Success probability per trial (0 < p < 1).
    pub p: f64,
    /// Milliseconds per trial; scales the discrete delay into wall-clock time.
    pub slot_ms: u64,
    pub min_ms: u64,
    pub max_ms: u64,
    #[serde(skip)]
    rng: Option<SmallRng>,
}

impl GeometricDelay {
    /// Create a new `GeometricDelay`.
    ///
    /// Mean delay = `slot_ms / p`.  Use a small `p` (e.g. 0.04) and small
    /// `slot_ms` (e.g. 10 ms) for a mean around 250 ms with fine granularity.
    pub fn new(p: f64, slot_ms: u64, min_ms: u64, max_ms: u64) -> Self {
        assert!(p > 0.0 && p < 1.0, "p must be in (0, 1)");
        assert!(slot_ms > 0, "slot_ms must be positive");
        assert!(min_ms <= max_ms, "min_ms must be <= max_ms");
        Self { p, slot_ms, min_ms, max_ms, rng: Some(SmallRng::from_entropy()) }
    }

    fn rng_mut(&mut self) -> &mut SmallRng {
        self.rng.get_or_insert_with(SmallRng::from_entropy)
    }
}

impl DelayStrategy for GeometricDelay {
    fn next_delay_ms(&mut self) -> u64 {
        // Inverse CDF: k = ⌈ ln(U) / ln(1−p) ⌉, k ≥ 1
        let u: f64 = self.rng_mut().gen_range(1e-15_f64..1.0_f64);
        let k = (u.ln() / (1.0 - self.p).ln()).ceil().max(1.0) as u64;
        k.saturating_mul(self.slot_ms).clamp(self.min_ms, self.max_ms)
    }

    fn strategy_name(&self) -> &'static str { "geometric" }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. Pareto delay
// ─────────────────────────────────────────────────────────────────────────────

/// Delays drawn from a Pareto (power-law) distribution.
///
/// PDF: `f(x) = shape × scale^shape / x^(shape+1)` for `x ≥ scale`.
/// Samples are generated via the inverse-CDF: `x = scale / U^(1/shape)`.
///
/// The heavy tail means that rare large delays are much more likely than under
/// exponential or uniform distributions.  This distorts any learned decision
/// boundary and forces an adversary to account for extreme cases, degrading
/// classifier confidence significantly.
///
/// Use `shape ≥ 2.0` to ensure finite variance; `shape ≥ 3.0` for finite
/// skewness.  Typical anonymity-appropriate configuration: `shape = 2.5`,
/// `scale_ms = 60.0`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParetoDelay {
    /// Shape parameter (α).  Higher values produce lighter tails.
    pub shape: f64,
    /// Scale parameter (minimum value) in milliseconds.
    pub scale_ms: f64,
    pub min_ms: u64,
    pub max_ms: u64,
    #[serde(skip)]
    rng: Option<SmallRng>,
}

impl ParetoDelay {
    /// Create a new `ParetoDelay`.
    ///
    /// The theoretical mean is `scale_ms × shape / (shape − 1)` when `shape > 1`.
    pub fn new(shape: f64, scale_ms: f64, min_ms: u64, max_ms: u64) -> Self {
        assert!(shape > 0.0, "shape must be positive");
        assert!(scale_ms > 0.0, "scale_ms must be positive");
        assert!(min_ms <= max_ms, "min_ms must be <= max_ms");
        Self { shape, scale_ms, min_ms, max_ms, rng: Some(SmallRng::from_entropy()) }
    }

    fn rng_mut(&mut self) -> &mut SmallRng {
        self.rng.get_or_insert_with(SmallRng::from_entropy)
    }
}

impl DelayStrategy for ParetoDelay {
    fn next_delay_ms(&mut self) -> u64 {
        // Inverse CDF: x = scale / U^(1/shape)
        let u: f64 = self.rng_mut().gen_range(1e-15_f64..1.0_f64);
        let sample = self.scale_ms / u.powf(1.0 / self.shape);
        (sample as u64).clamp(self.min_ms, self.max_ms)
    }

    fn strategy_name(&self) -> &'static str { "pareto" }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. Adaptive (ML-resistant) delay
// ─────────────────────────────────────────────────────────────────────────────

/// ML-resistant adaptive delay that rotates through multiple distributions.
///
/// An adversary training a neural classifier must adapt to a moving target.
/// `AdaptiveDelay` cycles through
///
/// ```text
/// Exponential  →  Geometric  →  Pareto  →  Hybrid  →  (repeat)
/// ```
///
/// switching strategy every ≈ `switch_every` samples with ±25 % random noise
/// on the switch point, so no single distribution can be reliably learnt.
///
/// # Parameters
///
/// `new(min_ms, max_ms)` uses sensible anonymity defaults:
///
/// | Strategy   | Parameters |
/// |------------|------------|
/// | Exponential | μ = 200 ms |
/// | Geometric   | p = 0.04, slot = 10 ms → mean ≈ 250 ms |
/// | Pareto      | α = 2.5, scale = 60 ms → mean ≈ 100 ms (heavy tail) |
/// | Hybrid      | rate = 4 Hz, jitter = 100 ms |
pub struct AdaptiveDelay {
    roster: Vec<Box<dyn DelayStrategy>>,
    current: usize,
    count: usize,
    switch_every: usize,
    rng: SmallRng,
    next_switch: usize,
}

impl AdaptiveDelay {
    /// Build an `AdaptiveDelay` with the default four-strategy roster.
    pub fn new(min_ms: u64, max_ms: u64) -> Self {
        let switch_every = 50;
        let mut rng = SmallRng::from_entropy();
        let next_switch = Self::jittered_next(&mut rng, switch_every);
        let roster: Vec<Box<dyn DelayStrategy>> = vec![
            Box::new(ExponentialDelay::with_jitter(200.0, min_ms, max_ms, 0.10)),
            Box::new(GeometricDelay::new(0.04, 10, min_ms, max_ms)),
            Box::new(ParetoDelay::new(2.5, 60.0, min_ms, max_ms)),
            Box::new(HybridDelay::new(4.0, 100, min_ms, max_ms)),
        ];
        Self { roster, current: 0, count: 0, switch_every, rng, next_switch }
    }

    /// Index (0..4) of the currently active strategy.
    pub fn current_strategy_index(&self) -> usize { self.current }

    /// Name of the currently active strategy.
    pub fn current_strategy_name(&self) -> &'static str {
        self.roster[self.current].strategy_name()
    }

    fn jittered_next(rng: &mut SmallRng, base: usize) -> usize {
        let lo = (base as f64 * 0.75) as usize;
        let hi = (base as f64 * 1.25) as usize;
        rng.gen_range(lo..=hi)
    }
}

impl DelayStrategy for AdaptiveDelay {
    fn next_delay_ms(&mut self) -> u64 {
        let delay = self.roster[self.current].next_delay_ms();
        self.count += 1;
        if self.count >= self.next_switch {
            self.current = (self.current + 1) % self.roster.len();
            self.count = 0;
            self.next_switch = Self::jittered_next(&mut self.rng, self.switch_every);
        }
        delay
    }

    fn strategy_name(&self) -> &'static str { "adaptive" }
}

// ─────────────────────────────────────────────────────────────────────────────
// Telemetry helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Collect a sample of delay values for benchmarking / regression testing.
pub fn sample_delays(strategy: &mut dyn DelayStrategy, n: usize) -> Vec<u64> {
    (0..n).map(|_| strategy.next_delay_ms()).collect()
}

/// Compute the sample mean of a delay distribution (milliseconds).
pub fn sample_mean(samples: &[u64]) -> f64 {
    if samples.is_empty() { return 0.0; }
    samples.iter().sum::<u64>() as f64 / samples.len() as f64
}

/// Compute the sample variance of a delay distribution.
pub fn sample_variance(samples: &[u64]) -> f64 {
    let mean = sample_mean(samples);
    samples.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>()
        / samples.len().max(1) as f64
}
