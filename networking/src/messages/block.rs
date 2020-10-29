use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};

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
#[derive(Deserializable, Serializable)]
pub struct Block {
    block_header: BlockHeader,
    transaction_count: CompactInt,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, txs: Vec<Transaction>) -> Block {
        let message = Block {
            block_header: header,
            transaction_count: CompactInt::from(txs.len()),
            transactions: txs,
        };
        message
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        for transaction in self.transactions.iter() {
            size += transaction.len();
        }
        let mut target = Vec::with_capacity(size + CompactInt::size(self.transactions.len()));
        self.serialize(&mut target)?;
        Ok(target)
    }
}
