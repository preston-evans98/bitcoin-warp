use crate::transaction::Transaction;
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable, Debug)]
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

#[test]
fn serial_size_empty() {
    use crate::payload::Payload;
    let txs = Vec::with_capacity(2);
    let msg = BlockTxn {
        block_hash: [1u8; 32],
        txs,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}

#[test]
fn serial_size_full() {
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
    let tx1 = Transaction::new(0, txins, outputs);
    let tx2 = Transaction::new(1, Vec::new(), Vec::new());

    let mut txs = Vec::with_capacity(2);
    txs.push(tx1);
    txs.push(tx2);
    let msg = BlockTxn {
        block_hash: [1u8; 32],
        txs,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
