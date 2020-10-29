use crate::transaction::Transaction;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};

#[derive(Serializable, Deserializable)]
pub struct Tx{
    transaction: Transaction
}
impl Tx{
    pub fn new(magic: i32, ins: Vec<TxInputs>, outs: Vec<TxOutputs>) -> Tx{
        Transaction{
            version: magic,
            inputs: ins,
            outputs: outs,
        }
    }
}
impl crate::payload::Payload for Tx{
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        size += self.transaction.len();
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}