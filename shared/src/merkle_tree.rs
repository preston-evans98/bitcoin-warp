use crate::u256;

pub struct MerkleTree {
    root: u256,
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree { root: u256::new() }
    }
    pub fn update(&mut self, hash: &u256) {
        todo!()
    }
    pub fn matches(&self, other: &u256) -> bool {
        &self.root == other
    }
}
