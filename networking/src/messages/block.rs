use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};
use crate::transaction::Transaction;
#[derive(Deserializable, Serializable)]
pub struct Block {
    block_header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, txs: Vec<Transaction>) -> Block {
        let message = Block {
            block_header: header,
            transactions: txs,
        };
        message
    }
}
impl crate::payload::Payload for Block{ 
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        size += BlockHeader::len();
        for transaction in self.transactions.iter() {
            size += transaction.len();
        }
        let mut target = Vec::with_capacity(size + CompactInt::size(self.transactions.len()));
        self.serialize(&mut target)?;
        Ok(target)
    }
}