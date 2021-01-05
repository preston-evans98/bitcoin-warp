use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Transaction};

#[derive(Serializable, Deserializable, Debug)]
pub struct PrefilledTransaction {
    index: CompactInt,
    pub tx: Transaction,
}

pub type Services = u64;
pub type Nonce = u64;
pub type ProtocolVersion = u32;
