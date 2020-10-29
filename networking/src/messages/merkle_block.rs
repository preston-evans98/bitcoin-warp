use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Deserializable, Serializable)]
pub struct MerkleBlock{
    block_header: BlockHeader,
    transaction_count: u32,
    //hashCount
    hashes: Vec<u256>, 
    //flagByteCount
    flags: Vec<Bytes>
}
impl crate::payload::Payload{
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>{
        let mut size = 0;
        size += BlockHeader::len() + 4 + CompactInt::size(self.hashes.len()) + (self.hashes.len() * 32) + CompactInt::size(self.flags.len()) + self.flags.len(); //32 bytes for each "hash" as they are u256
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}
