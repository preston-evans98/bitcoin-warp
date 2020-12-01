use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt};
#[derive(Deserializable, Serializable)]
pub struct Transaction {
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}
impl Transaction {
    pub fn len(&self) -> usize {
        let mut size = 0;
        size += 4 + CompactInt::size(self.inputs.len());
        for input in self.inputs.iter() {
            size += input.len();
        }
        size += CompactInt::size(self.outputs.len());
        for output in self.outputs.iter() {
            size += output.len();
        }
        size
    }
    pub fn new(version: i32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
        }
    }
}
#[derive(Deserializable, Serializable)]
pub struct TxInput {
    previous_outpoint: TxOutpoint,
    signature_script: Vec<u8>,
    sequence: u32, // Sequence number. Default for Bitcoin Core and almost all other programs is 0xffffffff.
}
impl TxInput {
    pub fn len(&self) -> usize {
        self.previous_outpoint.len()
            + CompactInt::size(self.signature_script.len())
            + self.signature_script.len()
            + 4
    }
}
#[derive(Deserializable, Serializable)]
pub struct TxOutput {
    value: i64,
    pk_script: Vec<u8>,
}
impl TxOutput {
    pub fn len(&self) -> usize {
        8 + CompactInt::size(self.pk_script.len()) + self.pk_script.len()
    }
}
#[derive(Deserializable, Serializable)]
pub struct TxOutpoint {
    hash: u256,
    index: u32,
}
impl TxOutpoint {
    pub fn len(&self) -> usize {
        32 + 4
    }
}

#[derive(Deserializable, Serializable)]
pub struct CoinbaseInput {}
