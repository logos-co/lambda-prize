//! # Merkle Event-Inclusion Proofs
//!
//! Builds a binary SHA-256 Merkle tree over all events in a transaction.
//! Produces a 32-byte `event_root` that is stored in the receipt, and
//! generates / verifies compact inclusion proofs for individual events.
//!
//! ## Security properties
//! - **Binding**: the root commits to the exact ordered set of events.
//! - **Soundness**: a forged proof requires a SHA-256 second-preimage.
//! - **Completeness**: every event in the tree has a valid proof.
//!
//! ## Wire format
//! Each leaf is `SHA256(0x00 || event_wire_bytes)`.
//! Each internal node is `SHA256(0x01 || left_child || right_child)`.
//! An empty tree has root `[0u8; 32]`.
//!
//! ## Example
//! ```rust,ignore
//! use lez_events::merkle::{EventMerkleTree, verify_proof};
//!
//! let events: Vec<&[u8]> = vec![b"event_a", b"event_b", b"event_c"];
//! let tree  = EventMerkleTree::build(&events);
//! let proof = tree.prove(1).unwrap();          // proof for "event_b"
//! assert!(verify_proof(&proof, tree.root()));
//! ```
use sha2::{Digest, Sha256};

// ── Domain-separation prefixes (prevents second-preimage attacks) ─────────────
const LEAF_PREFIX:   u8 = 0x00;
const BRANCH_PREFIX: u8 = 0x01;

// ── Types ─────────────────────────────────────────────────────────────────────
pub type Hash = [u8; 32];

/// Direction of a sibling node in a Merkle proof step.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProofDirection { Left, Right }

/// A single step in a Merkle inclusion proof.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofStep {
    pub sibling:   Hash,
    pub direction: ProofDirection,
}

/// A complete Merkle inclusion proof for one event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InclusionProof {
    /// Index of the event in the transaction's event list.
    pub event_index: usize,
    /// SHA-256 hash of the event's wire bytes (the leaf).
    pub leaf_hash:   Hash,
    /// Ordered list of sibling hashes from leaf to root.
    pub path:        Vec<ProofStep>,
    /// The Merkle root this proof is valid against.
    pub root:        Hash,
}

// ── EventMerkleTree ───────────────────────────────────────────────────────────
/// A complete binary SHA-256 Merkle tree over a transaction's events.
#[derive(Debug, Clone)]
pub struct EventMerkleTree {
    /// All nodes in the tree, level by level (leaves first).
    /// `nodes[0..n_leaves]` are the leaf hashes.
    nodes:    Vec<Hash>,
    n_leaves: usize,
}

impl EventMerkleTree {
    /// Build a Merkle tree from a slice of raw event wire-byte slices.
    /// An empty slice produces a tree whose root is `[0u8; 32]`.
    pub fn build(events: &[&[u8]]) -> Self {
        if events.is_empty() {
            return Self { nodes: vec![], n_leaves: 0 };
        }
        // Hash each leaf.
        let mut level: Vec<Hash> = events.iter().map(|e| leaf_hash(e)).collect();
        let n_leaves = level.len();
        let mut nodes = level.clone();

        // Build up the tree level by level.
        while level.len() > 1 {
            let mut next = Vec::with_capacity(level.len().div_ceil(2));
            let mut i = 0;
            while i < level.len() {
                let left  = level[i];
                // If odd number of nodes, duplicate the last one.
                let right = if i + 1 < level.len() { level[i + 1] } else { level[i] };
                next.push(branch_hash(&left, &right));
                i += 2;
            }
            nodes.extend_from_slice(&next);
            level = next;
        }
        Self { nodes, n_leaves }
    }

    /// The Merkle root of this tree.
    pub fn root(&self) -> Hash {
        if self.nodes.is_empty() { return [0u8; 32]; }
        *self.nodes.last().unwrap()
    }

    /// Number of leaves (events) in this tree.
    pub fn len(&self) -> usize { self.n_leaves }

    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool { self.n_leaves == 0 }

    /// Generate an inclusion proof for the event at `index`.
    /// Returns `None` if `index >= self.len()`.
    pub fn prove(&self, index: usize) -> Option<InclusionProof> {
        if index >= self.n_leaves { return None; }
        let leaf_hash = self.nodes[index];
        let mut path  = Vec::new();
        let mut idx   = index;
        let mut level_start = 0usize;
        let mut level_len   = self.n_leaves;

        while level_len > 1 {
            let sibling_idx = if idx.is_multiple_of(2) {
                // We are a left child; sibling is to the right (duplicate if last).
                let s = if idx + 1 < level_len { idx + 1 } else { idx };
                path.push(ProofStep {
                    sibling:   self.nodes[level_start + s],
                    direction: ProofDirection::Right,
                });
                s
            } else {
                // We are a right child; sibling is to the left.
                path.push(ProofStep {
                    sibling:   self.nodes[level_start + idx - 1],
                    direction: ProofDirection::Left,
                });
                idx - 1
            };
            let _ = sibling_idx;
            level_start += level_len;
            level_len    = level_len.div_ceil(2);
            idx         /= 2;
        }
        Some(InclusionProof { event_index: index, leaf_hash, path, root: self.root() })
    }
}

// ── Verification ──────────────────────────────────────────────────────────────
/// Verify an [`InclusionProof`] against a known `expected_root`.
///
/// Returns `true` iff the proof is valid and the computed root matches.
pub fn verify_proof(proof: &InclusionProof, expected_root: &Hash) -> bool {
    let mut current = proof.leaf_hash;
    for step in &proof.path {
        current = match step.direction {
            ProofDirection::Right => branch_hash(&current, &step.sibling),
            ProofDirection::Left  => branch_hash(&step.sibling, &current),
        };
    }
    &current == expected_root
}

/// Verify that `event_bytes` is the preimage of the leaf in `proof`.
pub fn verify_event_preimage(proof: &InclusionProof, event_bytes: &[u8]) -> bool {
    leaf_hash(event_bytes) == proof.leaf_hash
}

// ── Hash helpers ──────────────────────────────────────────────────────────────
fn leaf_hash(data: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update([LEAF_PREFIX]);
    h.update(data);
    h.finalize().into()
}

fn branch_hash(left: &Hash, right: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update([BRANCH_PREFIX]);
    h.update(left);
    h.update(right);
    h.finalize().into()
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const E0: &[u8] = b"event_alpha";
    const E1: &[u8] = b"event_beta";
    const E2: &[u8] = b"event_gamma";
    const E3: &[u8] = b"event_delta";

    // ── Tree construction ─────────────────────────────────────────────────────
    #[test]
    fn empty_tree_root_is_zero() {
        assert_eq!(EventMerkleTree::build(&[]).root(), [0u8; 32]);
    }

    #[test]
    fn single_leaf_root_equals_leaf_hash() {
        let tree = EventMerkleTree::build(&[E0]);
        let expected = leaf_hash(E0);
        assert_eq!(tree.root(), expected);
    }

    #[test]
    fn two_leaf_root_is_branch_of_leaves() {
        let tree     = EventMerkleTree::build(&[E0, E1]);
        let expected = branch_hash(&leaf_hash(E0), &leaf_hash(E1));
        assert_eq!(tree.root(), expected);
    }

    #[test]
    fn root_changes_when_event_changes() {
        let t1 = EventMerkleTree::build(&[E0, E1]);
        let t2 = EventMerkleTree::build(&[E0, E2]);
        assert_ne!(t1.root(), t2.root());
    }

    #[test]
    fn root_changes_when_order_changes() {
        let t1 = EventMerkleTree::build(&[E0, E1]);
        let t2 = EventMerkleTree::build(&[E1, E0]);
        assert_ne!(t1.root(), t2.root());
    }

    #[test]
    fn len_matches_event_count() {
        assert_eq!(EventMerkleTree::build(&[E0, E1, E2]).len(), 3);
    }

    // ── Proof generation ──────────────────────────────────────────────────────
    #[test]
    fn prove_returns_none_for_out_of_bounds() {
        let tree = EventMerkleTree::build(&[E0]);
        assert!(tree.prove(1).is_none());
    }

    #[test]
    fn prove_single_event_has_empty_path() {
        let tree  = EventMerkleTree::build(&[E0]);
        let proof = tree.prove(0).unwrap();
        assert!(proof.path.is_empty());
        assert!(verify_proof(&proof, &tree.root()));
    }

    // ── Proof verification ────────────────────────────────────────────────────
    #[test]
    fn all_proofs_verify_for_two_events() {
        let tree = EventMerkleTree::build(&[E0, E1]);
        for i in 0..2 {
            let proof = tree.prove(i).unwrap();
            assert!(verify_proof(&proof, &tree.root()), "proof {i} failed");
        }
    }

    #[test]
    fn all_proofs_verify_for_four_events() {
        let tree = EventMerkleTree::build(&[E0, E1, E2, E3]);
        for i in 0..4 {
            let proof = tree.prove(i).unwrap();
            assert!(verify_proof(&proof, &tree.root()), "proof {i} failed");
        }
    }

    #[test]
    fn all_proofs_verify_for_odd_count() {
        let tree = EventMerkleTree::build(&[E0, E1, E2]);
        for i in 0..3 {
            let proof = tree.prove(i).unwrap();
            assert!(verify_proof(&proof, &tree.root()), "proof {i} failed");
        }
    }

    #[test]
    fn tampered_proof_fails_verification() {
        let tree  = EventMerkleTree::build(&[E0, E1, E2, E3]);
        let mut proof = tree.prove(0).unwrap();
        proof.path[0].sibling = [0xFF; 32]; // corrupt a sibling
        assert!(!verify_proof(&proof, &tree.root()));
    }

    #[test]
    fn wrong_root_fails_verification() {
        let tree  = EventMerkleTree::build(&[E0, E1]);
        let proof = tree.prove(0).unwrap();
        let wrong_root = [0xDE; 32];
        assert!(!verify_proof(&proof, &wrong_root));
    }

    #[test]
    fn event_preimage_verification() {
        let tree  = EventMerkleTree::build(&[E0, E1]);
        let proof = tree.prove(0).unwrap();
        assert!( verify_event_preimage(&proof, E0));
        assert!(!verify_event_preimage(&proof, E1));
    }

    #[test]
    fn proof_serialises_to_json() {
        let tree  = EventMerkleTree::build(&[E0, E1, E2]);
        let proof = tree.prove(1).unwrap();
        let json  = serde_json::to_string(&proof).unwrap();
        let back: InclusionProof = serde_json::from_str(&json).unwrap();
        assert!(verify_proof(&back, &tree.root()));
    }
}
