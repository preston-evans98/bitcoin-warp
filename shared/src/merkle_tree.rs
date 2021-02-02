use warp_crypto::{merkleize, sha256d};

use crate::{u256, Serializable};

pub struct MerkleTree {
    root: u256,
    // hashes: Vec<u256>,
}

// struct MerkleNode (
//     Vec<u8>
// );
// impl MerkleNode {
//     pub fn new()
// }
impl MerkleTree {
    pub fn from(hashes: Vec<u256>) -> MerkleTree {
        MerkleTree {
            root: MerkleTree::merkle_root(hashes.iter()),
        }
    }

    pub fn from_iter<'a, I: ExactSizeIterator<Item = &'a u256>>(mut iter: I) -> MerkleTree {
        MerkleTree {
            root: MerkleTree::merkle_root(iter),
        }
    }

    pub fn matches(&self, other: &u256) -> bool {
        &self.root == other
    }
    /// Calculates the merkle root of a list of hashes inline
    /// into the allocated slice.
    ///
    /// In most cases, you'll want to use [merkle_root] instead.
    /// Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/hash.rs
    pub fn merkle_root_inline(data: &mut [[u8; 32]]) -> u256 {
        // Base case
        if data.is_empty() {
            return u256::new();
        }
        if data.len() < 2 {
            return u256::from_bytes(data[0]);
        }
        // Recursion
        for idx in 0..((data.len() + 1) / 2) {
            let idx1 = 2 * idx;
            let idx2 = std::cmp::min(idx1 + 1, data.len() - 1);
            data[idx] = merkleize(&data[idx1], &data[idx2]);
        }
        let half_len = data.len() / 2 + data.len() % 2;
        MerkleTree::merkle_root_inline(&mut data[0..half_len])
    }

    /// Creates a merkle tree from an iterator over u256.
    ///
    ///Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/hash.rs
    pub fn merkle_root<'a, I: ExactSizeIterator<Item = &'a u256>>(mut iter: I) -> u256 {
        // Base case
        if iter.len() == 0 {
            return u256::new();
        }
        // If the vec contains only the coinbase, the merkle root is the coinbase
        if iter.len() == 1 {
            return iter.next().unwrap().clone();
        }
        // Recursion
        let half_len = iter.len() / 2 + iter.len() % 2;
        let mut alloc = Vec::with_capacity(half_len);
        while let Some(hash1) = iter.next() {
            let hash2 = iter.next().unwrap_or(hash1);
            alloc.push(merkleize(hash1.to_le_bytes(), hash2.to_le_bytes()))
        }
        MerkleTree::merkle_root_inline(&mut alloc)
    }
}

// impl MerkleTree {
//     pub fn new() -> MerkleTree {
//         MerkleTree { root: u256::new() }
//     }
//     pub fn update(&mut self, hash: &u256) {
//         self.hashes.push(hash)
//     }
//     pub fn matches(&self, other: &u256) -> bool {
//         &self.root == other
//     }
//     pub fn finish(&mut self) {
//         if self.hashes.len() == 1 {
//             self.root = self.hashes.pop().expect("Merkle tree must have one hash");
//             return;
//         }

//     }
// }
