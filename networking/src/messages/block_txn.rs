use crate::transaction::Transaction;
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable)]
pub struct BlockTxn {
    block_hash: [u8; 32],
    txs: Vec<Transaction>,
}
impl crate::payload::Payload for BlockTxn {
    fn serialized_size(&self) -> usize {
        let mut size = 32;
        size += shared::CompactInt::size(self.txs.len());
        for transaction in self.txs.iter() {
            size += transaction.len();
        }
        size
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}
