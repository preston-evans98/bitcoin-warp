use crate::block_header::BlockHeader;

#[derive(Deserializable, Serializable)]
pub struct MerkleBlock{
    block_header: BlockHeader,
    transaction_count: u32,
    hash_count: CompactInt,
    hashes: Vec<u256>
    //need to add the rest here
}