use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Transaction};

#[derive(Serializable, Deserializable, Debug)]
pub struct PrefilledTransaction {
    index: CompactInt,
    tx: Transaction,
}

impl PrefilledTransaction {
    pub fn new(index: CompactInt, tx: Transaction) -> PrefilledTransaction {
        PrefilledTransaction { index, tx }
    }
    pub fn tx(&self) -> &Transaction {
        &self.tx
    }
}

pub type Services = u64;
pub type Nonce = u64;
pub type ProtocolVersion = u32;
