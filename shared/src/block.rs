use crate::block_header::BlockHeader;
pub use crate::hashes::BlockHash as Hash;
use crate::transaction::Transaction;
use crate::TxID;
use crate::{self as shared, Deserializable, DeserializationError, MerkleRoot};
use crate::{CompactInt, Serializable};
use bytes::BytesMut;
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
    pub fn header(&self) -> &BlockHeader {
        &self.block_header
    }
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
    pub fn txids(&self) -> Vec<&TxID> {
        self.transactions
            .iter()
            .map(|tx| tx.txid())
            .collect::<Vec<&TxID>>()
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
    pub fn deserialize(mut src: &mut BytesMut) -> Result<Self, DeserializationError> {
        let header = BlockHeader::deserialize(src.split_to(80))?;
        let tx_count = {
            let tx_count = CompactInt::deserialize(&mut src)?;
            tx_count.value()
        };

        // Reject empty blocks
        if tx_count == 0 {
            return Err(DeserializationError::Parse(String::from(
                "Block contains no transactions",
            )));
        }

        // Deserialize and structurally validate Coinbase
        let first_tx = Transaction::deserialize(&mut src)?;
        if !first_tx.is_coinbase() {
            return Err(DeserializationError::Parse(String::from(
                "Block did not contain Coinbase in first position",
            )));
        }
        // TODO: Parse block height
        if header.version() >= 2 {}
        let mut transactions = Vec::with_capacity(tx_count as usize);
        transactions.push(first_tx);

        // Parse and validate remaining transactions
        for _ in 1..tx_count {
            let next = Transaction::deserialize(&mut src)?;
            if next.is_coinbase() {
                return Err(DeserializationError::Parse(String::from(
                    "Block contained second Coinbase",
                )));
            }
            transactions.push(next);
        }
        let actual_merkle_root = MerkleRoot::from_iter(transactions.iter().map(|tx| tx.txid()));
        if !(&actual_merkle_root == header.merkle_root()) {
            return Err(DeserializationError::Parse(String::from(
                "Invalid Merkle Root",
            )));
        }
        Ok(Block::new(header, transactions))
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
        shared::block::Hash::from_u64(12345678),
        MerkleRoot::from_u64(9876543),
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
