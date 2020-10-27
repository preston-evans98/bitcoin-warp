use byteorder::{LittleEndian, ReadBytesExt};
use serde_derive::{Deserializable, Serializable};
use shared::{u256, Deserializable};
#[derive(Deserializable)]
pub struct BlockHeader {
    version: u32,
    prev_hash: u256,
    merkle_root: u256,
    time: u32,
    target: nBits,
    nonce: u32,
}

pub struct nBits {
    target: u256,
}
impl shared::Deserializable for nBits {
    fn deserialize<R>(target: &mut R) -> Result<nBits, shared::DeserializationError>
    where
        R: std::io::Read,
    {
        let mut mantissa = [0u8; 3];
        target.read_exact(&mut mantissa)?;
        // If the high order bit is set, return nbits 0;
        if mantissa[0] & 0b10000000 != 0 {
            return Ok(nBits {
                target: u256::new(),
            });
        }
        let exponent = target.read_u8()?;
        let offset = (exponent as usize).saturating_sub(3);
        let mut result = [0u8; 32];
        result[offset] = mantissa[0];
        result[offset + 1] = mantissa[1];
        result[offset + 2] = mantissa[2];

        Ok(nBits {
            target: u256::deserialize(&mut std::io::Cursor::new(result))?,
        })
    }
}
