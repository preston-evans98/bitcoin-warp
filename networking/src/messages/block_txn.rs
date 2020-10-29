use crate::block::Transaction;
use serde_derive::{Deserializable, Serializable};
use shared::{Serializable};
#[derive(Serializable, Deserializable)]
pub struct BlockTransactions{
    block_hash: [u8;32],
    txs: Vec<Transaction>

}
impl crate::payload::Payload{
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>{
        let mut size = 0;
        size += 256;
        for transaction in self.txs.iter() {
            size += transaction.len();
        }
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}
