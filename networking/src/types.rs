use bytes::{Buf, BytesMut};
use serde_derive::Serializable;
use shared::{CompactInt, Deserializable, DeserializationError, Transaction};

#[derive(Serializable, Debug)]
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
    pub fn deserialize(reader: &mut BytesMut) -> Result<PrefilledTransaction, DeserializationError>
    where
        Self: Sized,
    {
        let index = CompactInt::deserialize(&mut reader.reader())?;
        let tx = Transaction::deserialize(reader)?;
        Ok(PrefilledTransaction { index, tx })
    }
}

pub type Services = u64;
pub type Nonce = u64;
pub type ProtocolVersion = u32;
