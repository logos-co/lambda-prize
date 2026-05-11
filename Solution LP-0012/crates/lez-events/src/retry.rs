use alloc::string::{String, ToString};

use crate::{Backoff, BackoffConfig, EventError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryConfig {
    pub attempts: usize,
    pub backoff:  BackoffConfig,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self { attempts: 5, backoff: BackoffConfig::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryError {
    pub attempts:   usize,
    pub last_error: String,
}

impl core::fmt::Display for RetryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "retry exhausted after {} attempts: {}", self.attempts, self.last_error)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RetryError {}

pub fn retry<T, F>(cfg: RetryConfig, mut op: F) -> Result<T, EventError>
where
    F: FnMut() -> Result<T, EventError>,
{
    let mut backoff    = Backoff::new(cfg.backoff);
    let mut last_error = None;

    for attempt in 0..cfg.attempts {
        match op() {
            Ok(v)    => return Ok(v),
            Err(err) => {
                last_error = Some(err.to_string());
                if attempt + 1 >= cfg.attempts { break; }
                let _delay = backoff.next_delay();
            }
        }
    }

    Err(EventError::RetryExhausted {
        attempts:   cfg.attempts,
        last_error: last_error.unwrap_or_else(|| "unknown error".into()),
    })
}
