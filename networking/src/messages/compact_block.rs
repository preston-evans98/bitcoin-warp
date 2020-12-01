use crate::transaction::Transaction;
use crate::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::CompactInt;
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct CompactBlock {
    header: BlockHeader,
    nonce: u64,
    short_ids: Vec<u64>,
    prefilled_txns: Vec<PrefilledTransaction>,
}

#[derive(Serializable, Deserializable)]
pub struct PrefilledTransaction {
    index: CompactInt,
    tx: Transaction,
}

impl PrefilledTransaction {
    pub fn len(&self) -> usize {
        let txn_len = self.tx.len();
        txn_len + CompactInt::size(txn_len)
    }
}

impl crate::Payload for CompactBlock {
    fn serialized_size(&self) -> usize {
        let mut len = BlockHeader::len()
            + 8
            + CompactInt::size(self.short_ids.len())
            + 8 * self.short_ids.len()
            + CompactInt::size(self.prefilled_txns.len());
        for txn in self.prefilled_txns.iter() {
            len += txn.len();
        }
        len
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}
