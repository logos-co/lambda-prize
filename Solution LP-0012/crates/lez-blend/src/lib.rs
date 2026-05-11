//! # lez-blend — Blend proposer-anonymity layer
//!
//! Five concrete improvements to the Blend mechanism, all ready to integrate
//! with the rest of the LEZ workspace:
//!
//! | Module | Improvement |
//! |---|---|
//! | [`sphinx`] | **A** — Fixed-size Sphinx-style onion packets (X25519 + ChaCha20-Poly1305) |
//! | [`delay`] | **B** — `DelayStrategy` trait: Exponential / Poisson / Hybrid |
//! | [`cover`] | **C** — Poisson-rate cover traffic engine |
//! | [`mix_select`] | **D** — VRF-based Sybil-resistant mix-node path selection |
//! | [`decoy`] | **E** — Decoy broadcast: `fan_out` unlinkable dummy packets per real send |
//!
//! ## Quick-start
//!
//! ```rust
//! use lez_blend::{
//!     sphinx::{sphinx_wrap, SphinxHop},
//!     delay::{HybridDelay, DelayStrategy},
//!     cover::{CoverEngine, CoverConfig},
//!     mix_select::{MixNode, VrfMixSelector},
//!     decoy::{DecoyBroadcast, DecoyConfig},
//! };
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod cover;
pub mod decoy;
pub mod delay;
pub mod error;
pub mod mix_select;
pub mod sphinx;

pub use cover::{CoverConfig, CoverEngine};
pub use decoy::{DecoyBatch, DecoyBroadcast, DecoyConfig};
pub use delay::{
    AdaptiveDelay, DelayStrategy, ExponentialDelay, GeometricDelay, HybridDelay, ParetoDelay,
    PoissonDelay,
};
pub use error::{BlendError, BlendResult};
pub use mix_select::{MixNode, SelectedPath, VrfMixSelector};
pub use sphinx::{
    sphinx_unwrap, sphinx_wrap, SphinxHop, SphinxPacket,
    SPHINX_MAX_HOPS, SPHINX_PACKET_SIZE, SPHINX_PAYLOAD_SIZE,
};
