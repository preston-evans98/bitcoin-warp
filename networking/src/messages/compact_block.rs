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

#[test]
fn serial_size() {
    use crate::payload::Payload;
    let previous_outpoint = crate::TxOutpoint::new(shared::u256::from(1), 438);
    let txin1 = crate::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
    let txin2 = crate::TxInput::new(
        crate::TxOutpoint::new(shared::u256::new(), 0),
        Vec::new(),
        2,
    );
    let mut txins = Vec::new();
    txins.push(txin1);
    txins.push(txin2);
    let mut outputs = Vec::new();
    let out1 = crate::TxOutput::new(1, Vec::from([3u8; 11]));
    let out2 = crate::TxOutput::new(0, Vec::new());
    outputs.push(out1);
    outputs.push(out2);
    let tx1 = PrefilledTransaction {
        index: shared::CompactInt::from(1),
        tx: Transaction::new(0, txins, outputs),
    };
    let tx2 = PrefilledTransaction {
        index: shared::CompactInt::from(1),
        tx: Transaction::new(1, Vec::new(), Vec::new()),
    };

    let mut txs = Vec::with_capacity(2);
    txs.push(tx1);
    txs.push(tx2);
    let header = BlockHeader::new(
        23,
        shared::u256::from(12345678),
        shared::u256::from(9876543),
        2342,
        crate::block_header::Nbits::new(shared::u256::from(8719)),
        99,
    );

    let msg = CompactBlock {
        header,
        nonce: 1928712,
        short_ids: Vec::from([8219u64; 7]),
        prefilled_txns: txs,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
