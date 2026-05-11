use core::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackoffConfig {
    pub initial_delay_ms: u64,
    pub max_delay_ms:     u64,
    pub multiplier:       u32,
    pub max_elapsed_ms:   u64,
    pub jitter:           bool,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            initial_delay_ms: 100,
            max_delay_ms:     5_000,
            multiplier:       2,
            max_elapsed_ms:   30_000,
            jitter:           true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Backoff {
    cfg:        BackoffConfig,
    attempt:    usize,
    elapsed_ms: u64,
    seed:       u64,
}

impl Backoff {
    pub fn new(cfg: BackoffConfig) -> Self {
        Self { cfg, attempt: 0, elapsed_ms: 0, seed: 0x9E37_79B9_7F4A_7C15 }
    }

    fn next_jitter(&mut self, base: u64) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        let range  = (base / 4).max(1);
        let offset = self.seed % (range * 2 + 1);
        base.saturating_sub(range) + offset
    }

    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.elapsed_ms >= self.cfg.max_elapsed_ms {
            return None;
        }
        let mut delay = self.cfg.initial_delay_ms;
        for _ in 0..self.attempt {
            delay = delay.saturating_mul(self.cfg.multiplier as u64);
            if delay >= self.cfg.max_delay_ms {
                delay = self.cfg.max_delay_ms;
                break;
            }
        }
        if self.cfg.jitter {
            delay = self.next_jitter(delay).min(self.cfg.max_delay_ms);
        }
        self.attempt    += 1;
        self.elapsed_ms  = self.elapsed_ms.saturating_add(delay);
        Some(Duration::from_millis(delay))
    }

    pub fn reset(&mut self) {
        self.attempt    = 0;
        self.elapsed_ms = 0;
    }
}
