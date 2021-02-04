use bytes::Buf;
use serde_derive::Serializable;
use shared::{CompactInt, Deserializable, DeserializationError, Transaction};

#[derive(Serializable, Debug, Clone)]
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
    pub fn len(&self) -> usize {
        let txn_len = self.tx.len();
        txn_len + CompactInt::size(txn_len)
    }
    pub fn _test_txs() -> Vec<PrefilledTransaction> {
        let first = PrefilledTransaction {
            index: CompactInt::from(0),
            tx: Transaction::_test_coinbase(),
        };
        let second = PrefilledTransaction {
            index: CompactInt::from(0),
            tx: Transaction::_test_normal(),
        };
        vec![first, second]
    }
}

pub type Services = u64;
pub type Nonce = u64;
pub type ProtocolVersion = u32;
