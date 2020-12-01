use crate::transaction::Transaction;
use crate::transaction::{TxInput, TxOutput};
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct Tx {
    transaction: Transaction,
}
impl Tx {
    pub fn new(magic: i32, ins: Vec<TxInput>, outs: Vec<TxOutput>) -> Tx {
        Tx {
            transaction: Transaction::new(magic, ins, outs),
        }
    }
}
impl crate::payload::Payload for Tx {
    fn serialized_size(&self) -> usize {
        self.transaction.len()
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}
