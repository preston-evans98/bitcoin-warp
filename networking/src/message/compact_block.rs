use super::PrefilledTransaction;
use bytes::Buf;
use serde_derive::{Deserializable, Serializable};
use shared::BlockHeader;
use shared::CompactInt;
use shared::Serializable;
#[derive(Serializable, Deserializable, Debug, Clone)]
pub struct CompactBlock {
    header: BlockHeader,
    nonce: u64,
    short_ids: Vec<u64>,
    prefilled_txns: Vec<PrefilledTransaction>,
}

impl super::Payload for CompactBlock {
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

#[cfg(test)]
mod tests {
    use crate::types::PrefilledTransaction;

    use super::super::Payload;
    use shared::BlockHeader;
    #[test]
    fn serial_size() {
        let txs = PrefilledTransaction::_test_txs();
        let header = BlockHeader::_test_header();

        let msg = super::CompactBlock {
            header,
            nonce: 1928712,
            short_ids: Vec::from([8219u64; 7]),
            prefilled_txns: txs,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
}
