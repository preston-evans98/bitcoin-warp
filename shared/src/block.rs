use crate::block_header::BlockHeader;
use crate::transaction::Transaction;
use crate::{self as shared, Deserializable, DeserializationError};
use crate::{CompactInt, Serializable};
use serde_derive::Serializable;
#[derive(Serializable, Debug)]
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
    /// Deserializes a block. Attempts to make structurally invalid blocks unrepresentable by enforcing that...
    /// 1. The block contains exactly one Coinbase transaction, and it's in the first position.
    /// 1. The block does not contain duplicate transactions
    /// 1. The transactions merkle-ize to the root in the block header
    fn deserialize<R>(src: &mut R) -> Result<Self, DeserializationError>
    where
        R: std::io::Read,
    {
        let header = BlockHeader::deserialize(src)?;
        let tx_count = CompactInt::deserialize(src)?;

        // Reject empty blocks
        if tx_count.value() == 0 {
            return Err(DeserializationError::Parse(String::from(
                "Block contains no transactions",
            )));
        }
        
        let first_tx = Transaction::deserialize(src)?;
        // if !first_tx.
        // if header.version >= 2 {
        //     header.reported_height = 
        // }
        // FIXME: finish implementing

        todo!()
    }
}

#[test]
fn serial_size() {
    let previous_outpoint = crate::TxOutpoint::new(crate::u256::from(1), 438);
    let txin1 = crate::TxInput::new(previous_outpoint, Vec::from([8u8; 21]), 1);
    let txin2 = crate::TxInput::new(crate::TxOutpoint::new(crate::u256::new(), 0), Vec::new(), 2);
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
        crate::u256::from(12345678),
        crate::u256::from(9876543),
        2342,
        crate::block_header::Nbits::new(crate::u256::from(8719)),
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
