use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};
#[derive(Deserializable, Serializable)]
pub struct MerkleBlock {
    block_header: BlockHeader,
    transaction_count: u32,
    //hashCount
    hashes: Vec<u256>,
    //flagByteCount
    flags: Vec<u8>,
}
impl crate::payload::Payload for MerkleBlock {
    fn serialized_size(&self) -> usize {
        BlockHeader::len()
        + 4
        + CompactInt::size(self.hashes.len())
        + (self.hashes.len() * 32) //32 bytes for each "hash" as they are u256
        + CompactInt::size(self.flags.len())
        + self.flags.len()
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}
