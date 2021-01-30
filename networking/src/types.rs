use bytes::{Buf, BytesMut};
use serde_derive::Serializable;
use shared::{CompactInt, Deserializable, DeserializationError, Transaction};

#[derive(Serializable, Debug)]
pub struct PrefilledTransaction {
    index: CompactInt,
    tx: Transaction,
}

impl Deserializable for PrefilledTransaction {
    fn deserialize<B: Buf>(mut reader: B) -> Result<Self, DeserializationError>
    where
        Self: Sized,
    {
        let index = CompactInt::deserialize(&mut reader)?;
        let tx = Transaction::deserialize(&mut reader)?;
        Ok(PrefilledTransaction { index, tx })
    }
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
