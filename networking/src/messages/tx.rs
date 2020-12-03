use crate::transaction::Transaction;
use crate::transaction::{TxInput, TxOutput};
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;

#[derive(Serializable, Deserializable, Debug)]
pub struct Tx {
    transaction: Transaction,
}
impl Tx {
    pub fn new(magic: i32, ins: Vec<TxInput>, outs: Vec<TxOutput>) -> Tx {
        Tx {
            transaction: Transaction::new(magic, ins, outs),
        }
    }
}
impl crate::payload::Payload for Tx {
    fn serialized_size(&self) -> usize {
        self.transaction.len()
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
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
    let tx = Transaction::new(0, txins, outputs);

    let msg = Tx { transaction: tx };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
