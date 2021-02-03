use bytes::Buf;
use serde_derive::{Deserializable, Serializable};
use shared::{BlockHash, Serializable, Transaction};
#[derive(Serializable, Deserializable, Debug)]
pub struct BlockTxn {
    block_hash: BlockHash,
    txs: Vec<Transaction>,
}

impl super::Payload for BlockTxn {
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

#[cfg(test)]
mod tests {
    use super::super::Payload;
    use super::BlockTxn;
    use shared::BlockHash;

    #[test]
    fn serial_size_empty() {
        let txs = Vec::with_capacity(2);
        let msg = BlockTxn {
            block_hash: BlockHash::from([1u8; 32]),
            txs,
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }

    #[test]
    fn serial_size_full() {
        let msg = BlockTxn {
            block_hash: BlockHash::from([1u8; 32]),
            txs: shared::Transaction::_test_txs(),
        };
        let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
        assert_eq!(serial.len(), msg.serialized_size());
        assert_eq!(serial.len(), serial.capacity())
    }
}
