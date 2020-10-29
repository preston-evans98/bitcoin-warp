use crate::block_header::BlockHeader;

#[derive(Deserializable, Serializable)]
pub struct MerkleBlock{
    block_header: BlockHeader,
    transaction_count: u32,
    //hashCount
    hashes: Vec<u256>, 
    //flagByteCount
    flags: Vec<Bytes>
}

pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>{
    let mut size = 0;
    size += self.block_header.to_bytes()? + 4 + CompactInt::size(self.hashes.len()) + (self.hashes.len() * 32) + CompactInt::size(self.flags.len()) + self.flags.len(); //32 bytes for each "hash" as they are u256
    let mut target = Vec::with_capacity(size);
    self.serialize(&mut target)?;
    Ok(target)
}