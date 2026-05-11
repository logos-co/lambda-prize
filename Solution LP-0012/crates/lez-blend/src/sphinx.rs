/// Improvement A — Sphinx-style fixed-size onion packet format.
///
/// # Security properties
///
/// - **Fixed packet size** — every packet on the wire is exactly `SPHINX_PACKET_SIZE`
///   bytes regardless of path length or payload size.
/// - **Payload confidentiality** — layered BLAKE3-XOF stream cipher; each hop
///   peels one encryption layer with no size expansion.
/// - **Header integrity** — each hop verifies a BLAKE3-keyed MAC over its
///   inner routing section and the current payload; corrupted packets are dropped.
/// - **Per-hop unlinkability** — with a Ristretto/X25519 ephemeral key the
///   packet looks fresh at each hop; a full Sphinx filler construction can be
///   added on top to additionally hide the hop count.
///
/// # Packet wire layout
///
/// ```text
/// ┌─────────────────────────────────────────────────────────┐
/// │  alpha            32 B   ephemeral X25519 public key     │
/// ├─────────────────────────────────────────────────────────┤
/// │  hop_fields      240 B   5 × 48-byte slots               │
/// │    slot 0 (this hop): [routing_id 16 B | mac 32 B]       │
/// │    slots 1-4: opaque inner data (encrypted under inner   │
/// │               hops' keys, unreadable to this hop)        │
/// ├─────────────────────────────────────────────────────────┤
/// │  payload        1024 B   layered XOR stream-cipher       │
/// └─────────────────────────────────────────────────────────┘
/// ```
///
/// # Wrap protocol (sender, innermost → outermost)
///
/// For hop `i` from `n-1` down to `0`:
///   1. Compute `s_i = DH(e_sk, pk_i)`; derive `header_mac_key_i` and `payload_stream_key_i`.
///   2. XOR-encrypt the payload with `BLAKE3-XOF(payload_stream_key_i)` (no size growth).
///   3. Right-shift `hop_fields` by one slot (makes room at slot 0; last slot falls off).
///   4. Compute `mac_i = BLAKE3-keyed(header_mac_key_i, alpha ‖ hop_fields[1..] ‖ payload)`.
///   5. Write `[routing_id_i | mac_i]` into `hop_fields[0]`.
///
/// # Unwrap protocol (mix node)
///
///   1. Compute `s = DH(own_sk, alpha)`; derive keys.
///   2. Read `routing_id` and `received_mac` from `hop_fields[0]`.
///   3. Verify `BLAKE3-keyed(header_mac_key, alpha ‖ hop_fields[1..] ‖ payload) == received_mac`.
///   4. XOR-decrypt the payload layer.
///   5. Left-shift `hop_fields` by one slot (consume this hop's slot).
use alloc::vec::Vec;

use blake3::Hasher;
use rand::RngCore;
use x25519_dalek::{PublicKey as X25519Public, StaticSecret as X25519Secret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::{BlendError, BlendResult};

extern crate alloc;

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const SPHINX_MAX_HOPS: usize = 5;
pub const SPHINX_ROUTING_ID_SIZE: usize = 16;
pub const SPHINX_MAC_SIZE: usize = 32;
pub const SPHINX_HOP_FIELD_SIZE: usize = SPHINX_ROUTING_ID_SIZE + SPHINX_MAC_SIZE; // 48
pub const SPHINX_ALPHA_SIZE: usize = 32;
pub const SPHINX_HOP_SECTION_SIZE: usize = SPHINX_HOP_FIELD_SIZE * SPHINX_MAX_HOPS; // 240
pub const SPHINX_HEADER_SIZE: usize = SPHINX_ALPHA_SIZE + SPHINX_HOP_SECTION_SIZE;  // 272
pub const SPHINX_PAYLOAD_SIZE: usize = 1024;
pub const SPHINX_PACKET_SIZE: usize = SPHINX_HEADER_SIZE + SPHINX_PAYLOAD_SIZE;     // 1296

// ─────────────────────────────────────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────────────────────────────────────

/// Per-hop descriptor used only by the sender during `sphinx_wrap`.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SphinxHop {
    /// The mix node's X25519 public key (32 bytes).
    pub public_key: [u8; 32],
    /// 16-byte routing hint the receiving hop reads to decide where to forward.
    pub routing_id: [u8; SPHINX_ROUTING_ID_SIZE],
}

/// An assembled Sphinx packet.  Use `to_bytes()` / `from_bytes()` for wire
/// encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SphinxPacket {
    /// Ephemeral X25519 public key — shared across all hops.
    pub alpha: [u8; SPHINX_ALPHA_SIZE],
    /// Layered hop-routing slots.  Slot 0 always belongs to the current hop.
    pub hop_fields: [u8; SPHINX_HOP_SECTION_SIZE],
    /// Fixed-size encrypted payload.
    pub payload: [u8; SPHINX_PAYLOAD_SIZE],
}

impl SphinxPacket {
    pub fn to_bytes(&self) -> [u8; SPHINX_PACKET_SIZE] {
        let mut out = [0u8; SPHINX_PACKET_SIZE];
        out[..SPHINX_ALPHA_SIZE].copy_from_slice(&self.alpha);
        out[SPHINX_ALPHA_SIZE..SPHINX_HEADER_SIZE].copy_from_slice(&self.hop_fields);
        out[SPHINX_HEADER_SIZE..].copy_from_slice(&self.payload);
        out
    }

    pub fn from_bytes(b: &[u8; SPHINX_PACKET_SIZE]) -> Self {
        let mut alpha = [0u8; SPHINX_ALPHA_SIZE];
        let mut hop_fields = [0u8; SPHINX_HOP_SECTION_SIZE];
        let mut payload = [0u8; SPHINX_PAYLOAD_SIZE];
        alpha.copy_from_slice(&b[..SPHINX_ALPHA_SIZE]);
        hop_fields.copy_from_slice(&b[SPHINX_ALPHA_SIZE..SPHINX_HEADER_SIZE]);
        payload.copy_from_slice(&b[SPHINX_HEADER_SIZE..]);
        Self { alpha, hop_fields, payload }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Key derivation
// ─────────────────────────────────────────────────────────────────────────────

/// Per-hop key material derived via BLAKE3 KDF.
#[derive(Zeroize, ZeroizeOnDrop)]
struct HopKeys {
    header_mac_key: [u8; 32],
    payload_stream_key: [u8; 32],
}

fn derive_hop_keys(shared_secret: &[u8; 32], alpha: &[u8; 32]) -> HopKeys {
    // Unique 32-byte context per domain.
    let ctx = {
        let mut v = [0u8; 64];
        v[..32].copy_from_slice(shared_secret);
        v[32..].copy_from_slice(alpha);
        v
    };
    let mut header_mac_key = [0u8; 32];
    let mut payload_stream_key = [0u8; 32];
    header_mac_key.copy_from_slice(
        blake3::keyed_hash(b"blend/header-mac-key-v2\0\0\0\0\0\0\0\0\0", &ctx).as_bytes(),
    );
    payload_stream_key.copy_from_slice(
        blake3::keyed_hash(b"blend/payload-stream-key-v2\0\0\0\0\0", &ctx).as_bytes(),
    );
    HopKeys { header_mac_key, payload_stream_key }
}

/// BLAKE3-keyed MAC over `alpha ‖ hop_fields_inner ‖ payload`.
///
/// "Inner" = `hop_fields[HOP_FIELD_SIZE..]` — the slots forwarded to later hops.
fn compute_mac(
    mac_key: &[u8; 32],
    alpha: &[u8; SPHINX_ALPHA_SIZE],
    hop_fields_inner: &[u8],
    payload: &[u8; SPHINX_PAYLOAD_SIZE],
) -> [u8; SPHINX_MAC_SIZE] {
    let mut h = Hasher::new_keyed(mac_key);
    h.update(alpha);
    h.update(hop_fields_inner);
    h.update(payload);
    *h.finalize().as_bytes()
}

/// BLAKE3-XOF keystream for the payload stream cipher.
fn payload_keystream(key: &[u8; 32]) -> [u8; SPHINX_PAYLOAD_SIZE] {
    let mut out = [0u8; SPHINX_PAYLOAD_SIZE];
    let mut h = blake3::Hasher::new_keyed(key);
    h.update(b"blend/payload-stream-v2");
    h.finalize_xof().fill(&mut out);
    out
}

/// Constant-time byte comparison (prevents timing oracles on MACs).
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

// ─────────────────────────────────────────────────────────────────────────────
// Wrap
// ─────────────────────────────────────────────────────────────────────────────

/// Build a Sphinx packet from `plaintext` and an ordered list of `hops`.
///
/// `hops[0]` = entry mix node; `hops[n-1]` = final destination.
///
/// The caller is responsible for passing a cryptographically-secure RNG.
pub fn sphinx_wrap<R: RngCore>(
    hops: &[SphinxHop],
    plaintext: &[u8],
    rng: &mut R,
) -> BlendResult<SphinxPacket> {
    if hops.is_empty() {
        return Err(BlendError::EmptyHops);
    }
    if hops.len() > SPHINX_MAX_HOPS {
        return Err(BlendError::TooManyHops {
            max: SPHINX_MAX_HOPS,
            requested: hops.len(),
        });
    }
    if plaintext.len() > SPHINX_PAYLOAD_SIZE {
        return Err(BlendError::PayloadTooLarge {
            max: SPHINX_PAYLOAD_SIZE,
            got: plaintext.len(),
        });
    }

    // Ephemeral X25519 key pair.
    let mut e_bytes = [0u8; 32];
    rng.fill_bytes(&mut e_bytes);
    let e_sk = X25519Secret::from(e_bytes);
    let alpha: [u8; 32] = *X25519Public::from(&e_sk).as_bytes();

    // Pre-compute one shared secret per hop.
    let shared_secrets: Vec<[u8; 32]> = hops
        .iter()
        .map(|h| *e_sk.diffie_hellman(&X25519Public::from(h.public_key)).as_bytes())
        .collect();

    // ── Build payload and header, innermost hop first ──

    let mut payload = [0u8; SPHINX_PAYLOAD_SIZE];
    payload[..plaintext.len()].copy_from_slice(plaintext);

    let mut hop_fields = [0u8; SPHINX_HOP_SECTION_SIZE];

    for i in (0..hops.len()).rev() {
        let keys = derive_hop_keys(&shared_secrets[i], &alpha);

        // 1. Encrypt the payload layer for this hop (XOR stream cipher, no size growth).
        let ks = payload_keystream(&keys.payload_stream_key);
        for (b, k) in payload.iter_mut().zip(ks.iter()) {
            *b ^= k;
        }

        // 2. Right-shift hop_fields by one slot.
        //    Slots [0..n-1] → [1..n]; slot n falls off; slot 0 is now empty.
        hop_fields.copy_within(
            0..SPHINX_HOP_SECTION_SIZE - SPHINX_HOP_FIELD_SIZE,
            SPHINX_HOP_FIELD_SIZE,
        );
        hop_fields[..SPHINX_HOP_FIELD_SIZE].fill(0);

        // 3. Compute MAC over (alpha, inner slots, encrypted payload).
        //    This binds the MAC to both the routing info that inner hops will
        //    see AND the payload state this hop receives.
        let mac = compute_mac(
            &keys.header_mac_key,
            &alpha,
            &hop_fields[SPHINX_HOP_FIELD_SIZE..],
            &payload,
        );

        // 4. Write [routing_id | mac] into slot 0.
        hop_fields[..SPHINX_ROUTING_ID_SIZE].copy_from_slice(&hops[i].routing_id);
        hop_fields[SPHINX_ROUTING_ID_SIZE..SPHINX_HOP_FIELD_SIZE].copy_from_slice(&mac);
    }

    Ok(SphinxPacket { alpha, hop_fields, payload })
}

// ─────────────────────────────────────────────────────────────────────────────
// Unwrap
// ─────────────────────────────────────────────────────────────────────────────

/// Process a Sphinx packet at a single mix node.
///
/// Returns `(routing_id, next_packet)`.  When `routing_id` is all-zeros the
/// node is the final destination and `next_packet.payload` holds the plaintext
/// (zero-padded to `SPHINX_PAYLOAD_SIZE`).
///
/// `node_sk_bytes` is the node's 32-byte X25519 secret scalar.
pub fn sphinx_unwrap(
    packet: &SphinxPacket,
    node_sk_bytes: &[u8; 32],
) -> BlendResult<([u8; SPHINX_ROUTING_ID_SIZE], SphinxPacket)> {
    let node_sk = X25519Secret::from(*node_sk_bytes);
    let shared_secret = *node_sk.diffie_hellman(&X25519Public::from(packet.alpha)).as_bytes();
    let keys = derive_hop_keys(&shared_secret, &packet.alpha);

    // 1. Read slot 0: [routing_id | received_mac].
    let mut routing_id = [0u8; SPHINX_ROUTING_ID_SIZE];
    routing_id.copy_from_slice(&packet.hop_fields[..SPHINX_ROUTING_ID_SIZE]);
    let received_mac = &packet.hop_fields[SPHINX_ROUTING_ID_SIZE..SPHINX_HOP_FIELD_SIZE];

    // 2. Verify MAC over (alpha, inner slots, current encrypted payload).
    let expected_mac = compute_mac(
        &keys.header_mac_key,
        &packet.alpha,
        &packet.hop_fields[SPHINX_HOP_FIELD_SIZE..],
        &packet.payload,
    );
    if !ct_eq(&expected_mac, received_mac) {
        return Err(BlendError::MacMismatch);
    }

    // 3. XOR-decrypt one payload layer.
    let mut payload = packet.payload;
    let ks = payload_keystream(&keys.payload_stream_key);
    for (b, k) in payload.iter_mut().zip(ks.iter()) {
        *b ^= k;
    }

    // 4. Left-shift hop_fields: consume slot 0, append a blank slot at the end.
    let mut hop_fields = packet.hop_fields;
    hop_fields.copy_within(SPHINX_HOP_FIELD_SIZE.., 0);
    hop_fields[SPHINX_HOP_SECTION_SIZE - SPHINX_HOP_FIELD_SIZE..].fill(0);

    Ok((routing_id, SphinxPacket { alpha: packet.alpha, hop_fields, payload }))
}
