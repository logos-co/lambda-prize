use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MerkleNode {
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MerklePath {
    pub leaf:     String,
    pub index:    usize,
    pub siblings: Vec<String>,
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn merkle_root(leaves: &[Vec<u8>]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }

    let mut level: Vec<[u8; 32]> = leaves.iter().map(|x| sha256(x)).collect();

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last = *level.last().unwrap();
            level.push(last);
        }

        let mut next = Vec::with_capacity(level.len() / 2);
        let mut i = 0;
        while i < level.len() {
            let mut data = Vec::with_capacity(64);
            data.extend_from_slice(&level[i]);
            data.extend_from_slice(&level[i + 1]);
            next.push(sha256(&data));
            i += 2;
        }
        level = next;
    }

    level[0]
}

pub fn merkle_path(leaves: &[Vec<u8>], mut index: usize) -> MerklePath {
    let leaf = hex::encode(leaves[index].clone());
    let mut level: Vec<[u8; 32]> = leaves.iter().map(|x| sha256(x)).collect();
    let mut siblings = Vec::new();

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last = *level.last().unwrap();
            level.push(last);
        }

        let sibling = if index % 2 == 0 { index + 1 } else { index - 1 };
        siblings.push(hex::encode(level[sibling]));

        let mut next = Vec::with_capacity(level.len() / 2);
        let mut i = 0;
        while i < level.len() {
            let mut data = Vec::with_capacity(64);
            data.extend_from_slice(&level[i]);
            data.extend_from_slice(&level[i + 1]);
            next.push(sha256(&data));
            i += 2;
        }

        index /= 2;
        level = next;
    }

    MerklePath { leaf, index, siblings }
}

pub fn verify_merkle_path(root: [u8; 32], path: &MerklePath) -> bool {
    let mut current = match hex::decode(&path.leaf) {
        Ok(v) => sha256(&v),
        Err(_) => return false,
    };

    let mut idx = path.index;
    for sib in &path.siblings {
        let sibling = match hex::decode(sib) {
            Ok(v) => v,
            Err(_) => return false,
        };
        if sibling.len() != 32 {
            return false;
        }
        let mut sibling_hash = [0u8; 32];
        sibling_hash.copy_from_slice(&sibling);

        let mut data = Vec::with_capacity(64);
        if idx % 2 == 0 {
            data.extend_from_slice(&current);
            data.extend_from_slice(&sibling_hash);
        } else {
            data.extend_from_slice(&sibling_hash);
            data.extend_from_slice(&current);
        }

        current = sha256(&data);
        idx /= 2;
    }

    current == root
}
