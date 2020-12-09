use crate::block_header::BlockHeader;
use crate::transaction::Transaction;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Deserializable, Serializable, Debug)]
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
impl crate::payload::Payload for Block {
    fn serialized_size(&self) -> usize {
        let mut size = CompactInt::size(self.transactions.len());
        size += BlockHeader::len();
        for transaction in self.transactions.iter() {
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
    let tx1 = Transaction::new(0, txins, outputs);
    let tx2 = Transaction::new(1, Vec::new(), Vec::new());

    let mut txs = Vec::with_capacity(2);
    txs.push(tx1);
    txs.push(tx2);
    let block_header = BlockHeader::new(
        23,
        shared::u256::from(12345678),
        shared::u256::from(9876543),
        2342,
        crate::block_header::Nbits::new(shared::u256::from(8719)),
        99,
    );

    let msg = Block {
        block_header,
        transactions: txs,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
