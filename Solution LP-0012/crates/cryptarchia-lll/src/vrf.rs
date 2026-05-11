/// ECVRF-RISTRETTO255-SHA512 implementation.
///
/// Follows the structure of the IETF ECVRF draft (draft-irtf-cfrg-vrf-15):
///   1.  H  = hash_to_ristretto(pk_bytes || alpha)   — deterministic curve point
///   2.  Γ  = sk · H                                 — VRF gamma
///   3.  k  = random scalar (ephemeral)
///   4.  U  = k · G,  V = k · H
///   5.  c  = challenge_hash(G, H, pk, Γ, U, V)
///   6.  s  = k − c · sk  (mod ℓ)
///   7.  β  = SHA-512(Γ_compressed)[..32]            — VRF output
///
/// Verification reconstructs U′ = s·G + c·pk and V′ = s·H + c·Γ,
/// then checks that c matches the hash.
use alloc::vec::Vec;

use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT,
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

use crate::{
    crypto::NodeSecret,
    types::{ChainId, EpochId, Slot},
    LllError, LllResult,
};

const SUITE: &[u8] = b"cryptarchia-ecvrf-ristretto255-sha512-v1";

/// The verified random output of the VRF — 32 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VrfOutput(pub [u8; 32]);

/// Compact VRF proof: compressed Γ (32 bytes), challenge c (32 bytes), scalar s (32 bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VrfProof {
    pub gamma: [u8; 32],
    pub c: [u8; 32],
    pub s: [u8; 32],
}

/// Context bound to a specific slot evaluation.
#[derive(Debug, Clone)]
pub struct VrfInput {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub beacon_seed: [u8; 32],
}

impl VrfInput {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(SUITE);
        v.extend_from_slice(b"input");
        v.extend_from_slice(&self.chain_id.to_le_bytes());
        v.extend_from_slice(&self.epoch_id.to_le_bytes());
        v.extend_from_slice(&self.slot.to_le_bytes());
        v.extend_from_slice(&self.beacon_seed);
        v
    }
}

/// Derive the secret Ristretto scalar from a `NodeSecret`.
///
/// We hash with SHA-512 and use `from_bytes_mod_order_wide` so the
/// mapping is uniform over the scalar field — no clamping artefacts.
fn secret_scalar(secret: &NodeSecret) -> Scalar {
    let mut h = Sha512::new();
    h.update(SUITE);
    h.update(b"secret-scalar");
    h.update(&secret.secret);
    let wide: [u8; 64] = h.finalize().into();
    Scalar::from_bytes_mod_order_wide(&wide)
}

/// Hash arbitrary bytes to a Ristretto255 curve point (Elligator2 map).
fn hash_to_ristretto(data: &[u8]) -> RistrettoPoint {
    let mut h = Sha512::new();
    h.update(SUITE);
    h.update(b"hash-to-curve");
    h.update(data);
    let wide: [u8; 64] = h.finalize().into();
    RistrettoPoint::from_uniform_bytes(&wide)
}

/// Compute the Fiat-Shamir challenge scalar from the six proof inputs.
fn challenge_scalar(
    pk: &RistrettoPoint,
    h: &RistrettoPoint,
    gamma: &RistrettoPoint,
    u: &RistrettoPoint,
    v: &RistrettoPoint,
) -> Scalar {
    let mut hash = Sha512::new();
    hash.update(SUITE);
    hash.update(b"challenge");
    hash.update(RISTRETTO_BASEPOINT_POINT.compress().as_bytes());
    hash.update(h.compress().as_bytes());
    hash.update(pk.compress().as_bytes());
    hash.update(gamma.compress().as_bytes());
    hash.update(u.compress().as_bytes());
    hash.update(v.compress().as_bytes());
    let wide: [u8; 64] = hash.finalize().into();
    Scalar::from_bytes_mod_order_wide(&wide)
}

/// Derive the VRF output beta from Γ.
fn gamma_to_output(gamma: &RistrettoPoint) -> VrfOutput {
    let mut h = Sha512::new();
    h.update(SUITE);
    h.update(b"proof-to-hash");
    h.update(gamma.compress().as_bytes());
    let digest = h.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..32]);
    VrfOutput(out)
}

/// Prove: given a `NodeSecret` and VRF alpha input, return `(beta, proof)`.
///
/// `beta` is the pseudorandom output; `proof` enables network peers to
/// verify the output without learning the secret key.
pub fn vrf_prove(secret: &NodeSecret, alpha: &[u8]) -> (VrfOutput, VrfProof) {
    let sk = secret_scalar(secret);
    let pk = RISTRETTO_BASEPOINT_POINT * sk;

    let h = hash_to_ristretto(&[pk.compress().as_bytes(), alpha].concat());
    let gamma = h * sk;

    let k = Scalar::random(&mut OsRng);
    let u = RISTRETTO_BASEPOINT_POINT * k;
    let v = h * k;

    let c = challenge_scalar(&pk, &h, &gamma, &u, &v);
    let s = k - c * sk;

    let output = gamma_to_output(&gamma);
    let proof = VrfProof {
        gamma: gamma.compress().to_bytes(),
        c: c.to_bytes(),
        s: s.to_bytes(),
    };

    (output, proof)
}

/// Verify: check that `beta` was produced by the holder of `pk_bytes` on input `alpha`.
///
/// Returns `Ok(beta)` if the proof is valid; `Err` otherwise.
pub fn vrf_verify(
    pk_bytes: &[u8; 32],
    alpha: &[u8],
    claimed_output: &VrfOutput,
    proof: &VrfProof,
) -> LllResult<VrfOutput> {
    let pk = CompressedRistretto::from_slice(pk_bytes)
        .map_err(|_| LllError::InvalidKeyMaterial)?
        .decompress()
        .ok_or(LllError::InvalidKeyMaterial)?;

    let gamma = CompressedRistretto::from_slice(&proof.gamma)
        .map_err(|_| LllError::ProofVerificationFailed("bad gamma encoding".into()))?
        .decompress()
        .ok_or(LllError::ProofVerificationFailed("gamma not on curve".into()))?;

    let c = Scalar::from_canonical_bytes(proof.c)
        .into_option()
        .ok_or(LllError::ProofVerificationFailed("non-canonical c scalar".into()))?;
    let s = Scalar::from_canonical_bytes(proof.s)
        .into_option()
        .ok_or(LllError::ProofVerificationFailed("non-canonical s scalar".into()))?;

    let h = hash_to_ristretto(&[pk.compress().as_bytes(), alpha].concat());

    // Reconstruct U' = s·G + c·pk  and  V' = s·H + c·Γ
    let u_prime = RISTRETTO_BASEPOINT_POINT * s + pk * c;
    let v_prime = h * s + gamma * c;

    let c_prime = challenge_scalar(&pk, &h, &gamma, &u_prime, &v_prime);
    if c_prime != c {
        return Err(LllError::ProofVerificationFailed(
            "VRF challenge mismatch".into(),
        ));
    }

    let computed = gamma_to_output(&gamma);
    if computed != *claimed_output {
        return Err(LllError::ProofVerificationFailed(
            "VRF output mismatch".into(),
        ));
    }

    Ok(computed)
}

/// Return the 32-byte VRF public key for a `NodeSecret`.
pub fn vrf_public_key(secret: &NodeSecret) -> [u8; 32] {
    let sk = secret_scalar(secret);
    let pk = RISTRETTO_BASEPOINT_POINT * sk;
    pk.compress().to_bytes()
}

/// Convert a `VrfOutput` to a u128 value used for threshold comparison.
///
/// Takes the first 16 bytes of the output, interpreted as little-endian.
pub fn vrf_output_to_u128(output: &VrfOutput) -> u128 {
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&output.0[..16]);
    u128::from_le_bytes(buf)
}

/// Check that a VRF output falls below the leadership threshold.
pub fn vrf_wins(output: &VrfOutput, threshold: u128) -> bool {
    vrf_output_to_u128(output) <= threshold
}
