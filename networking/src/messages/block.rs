use shared::{CompactInt,Bytes, u256};
use config::Config;
use log::warn;

pub struct Transaction{
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}
pub struct TxInput{
    previous_outpoint: TxOutpoint,
    signature_script: Vec<u8>,
    sequence: u32,    // Sequence number. Default for Bitcoin Core and almost all other programs is 0xffffffff.
}
pub struct TxOutput{
    value: i64,
    pk_script: Vec<u8>,
}
pub struct TxOutpoint{
    hash: u256,
    index: u32,
}
pub struct CoinbaseInput{
    
}
pub struct Block{
    transactions: Vec<Transaction>,
}

impl Block{
    pub fn new(txs: Vec<Transaction>) -> Block{
        let message = Block {
            transactions: txs,
        };
        message
    }

}
// impl Deserializable for Block{
//     fn deserialize<R>(target: &mut R) -> Result<u32>
//     where
//         R: std::io::Read,
//     {
//         Ok(target.read_u32::<LittleEndian>()?)
//     }
// }
