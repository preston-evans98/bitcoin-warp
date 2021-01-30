use crate::serializable::Serializable;
use crate::{self as shared, Cached, Deserializable, DeserializationError};
use crate::{u256, CompactInt};
use bytes::Buf;
use serde_derive::{Deserializable, Serializable};
use warp_crypto::sha256d;

#[derive(Serializable, Debug)]
pub struct Transaction {
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    hash: Cached<u256>,
}

/// Deserializes a transaction. Expects to be handed a buffer with at most
impl Deserializable for Transaction {
    fn deserialize<B: Buf>(mut src: B) -> Result<Self, DeserializationError> {
        let mut tx = Transaction {
            version: i32::deserialize(&mut src)?,
            inputs: <Vec<TxInput>>::deserialize(&mut src)?,
            outputs: <Vec<TxOutput>>::deserialize(&mut src)?,
            hash: Cached::new(),
        };
        // Calculate tx hash
        // FIXME: Find a way to avoid this copy
        let mut out = Vec::with_capacity(tx.len());
        tx.serialize(&mut out)
            .expect("Serialization to vec should not fail!");
        let hash_bytes = sha256d(&out[..]);
        let own_hash = u256::from_bytes(hash_bytes);
        tx.hash = Cached::from(own_hash);
        Ok(tx)
    }
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
            hash: Cached::new(),
        }
    }
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].is_coinbase_in()
    }
    pub fn hash(&self) -> Option<&u256> {
        self.hash.ref_value()
    }

    // pub fn deserialize(src: &mut BytesMut) -> Result<Self, DeserializationError> {
    //     // let src = Bytes::from(*src);
    //     let len = std::cmp::min(MAX_TX_LENGTH, src.remaining());
    //     // Note: this op is zero-copy if the underlying is a Bytes or BytesMut object
    //     let mut src = src.copy_to_bytes(len);
    //     let backup = src.clone();
    //     let mut tx = Transaction {
    //         version: i32::deserialize(&mut src)?,
    //         inputs: <Vec<TxInput>>::deserialize(&mut src)?,
    //         outputs: <Vec<TxOutput>>::deserialize(&mut src)?,
    //         hash: Cached::new(),
    //     };
    //     // FIXME: Make sure the offset is correct!

    //     let hash_bytes = sha256d(&backup[..tx.len()]);
    //     let own_hash = u256::from_bytes(hash_bytes);
    //     tx.hash = Cached::from(own_hash);
    //     Ok(tx)
    // }
}

#[derive(Deserializable, Serializable, Debug)]
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
    pub fn new(previous_outpoint: TxOutpoint, signature_script: Vec<u8>, sequence: u32) -> TxInput {
        TxInput {
            previous_outpoint,
            signature_script,
            sequence,
        }
    }
    pub fn is_coinbase_in(&self) -> bool {
        self.previous_outpoint.hash.is_zero() && self.previous_outpoint.index == std::u32::MAX
    }
}
#[derive(Deserializable, Serializable, Debug)]
pub struct TxOutput {
    value: i64,
    pk_script: Vec<u8>,
}
impl TxOutput {
    pub fn len(&self) -> usize {
        8 + CompactInt::size(self.pk_script.len()) + self.pk_script.len()
    }
    pub fn new(value: i64, pk_script: Vec<u8>) -> TxOutput {
        TxOutput { value, pk_script }
    }
}
#[derive(Deserializable, Serializable, Debug)]
pub struct TxOutpoint {
    hash: u256,
    index: u32,
}
impl TxOutpoint {
    pub fn len(&self) -> usize {
        32 + 4
    }
    pub fn new(hash: u256, index: u32) -> TxOutpoint {
        TxOutpoint { hash, index }
    }
}

// #[derive(Deserializable, Serializable)]
// pub struct CoinbaseInput {}
