use warp_crypto::merkleize;

pub use crate::hashes::MerkleRoot;
use crate::TxID;
use serde_derive::{Deserializable, Serializable};

// #[derive(Serializable, Deserializable, Debug)]
// pub struct MerkleRoot {
//     root: [u8; 32],
// }

// impl PartialEq for MerkleRoot {
//     fn eq(&self, other: &Self) -> bool {
//         self.root == other.root
//     }
// }
// struct MerkleNode (
//     Vec<u8>
// );
// impl MerkleNode {
//     pub fn new()
// }
impl MerkleRoot {
    pub fn from_vec(txids: Vec<&TxID>) -> MerkleRoot {
        MerkleRoot::from(merkle_root(txids.iter().map(|h| *h)))
    }

    pub fn from_iter<'a, I: ExactSizeIterator<Item = &'a TxID>>(mut iter: I) -> MerkleRoot {
        MerkleRoot::from(merkle_root(iter))
    }
    pub fn root(&self) -> &[u8; 32] {
        self.inner()
    }
}
/// Calculates the merkle root of a list of hashes inline
/// into the allocated slice.
///
/// In most cases, you'll want to use [merkle_root] instead.
/// Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/hash.rs
pub fn merkle_root_inline(data: &mut [[u8; 32]]) -> [u8; 32] {
    // Base case
    if data.is_empty() {
        return [0u8; 32];
    }
    if data.len() < 2 {
        return data[0];
    }
    // Recursion
    for idx in 0..((data.len() + 1) / 2) {
        let idx1 = 2 * idx;
        let idx2 = std::cmp::min(idx1 + 1, data.len() - 1);
        data[idx] = merkleize(&data[idx1], &data[idx2]);
    }
    let half_len = data.len() / 2 + data.len() % 2;
    merkle_root_inline(&mut data[0..half_len])
}

/// Creates a merkle tree from an iterator over u256.
///
///Adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/hash.rs
pub fn merkle_root<'a, I: ExactSizeIterator<Item = &'a TxID>>(mut iter: I) -> [u8; 32] {
    // Base case
    if iter.len() == 0 {
        return [0u8; 32];
    }
    // If the vec contains only the coinbase, the merkle root is the coinbase
    if iter.len() == 1 {
        return iter.next().unwrap().to_le_bytes().clone();
    }
    // Recursion
    let half_len = iter.len() / 2 + iter.len() % 2;
    let mut alloc = Vec::with_capacity(half_len);
    while let Some(hash1) = iter.next() {
        let hash2 = iter.next().unwrap_or(hash1);
        alloc.push(merkleize(hash1.to_le_bytes(), hash2.to_le_bytes()))
    }
    merkle_root_inline(&mut alloc)
}

// impl MerkleRoot {
//     pub fn new() -> MerkleRoot {
//         MerkleRoot { root: u256::new() }
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
