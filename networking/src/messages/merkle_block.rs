use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};
#[derive(Deserializable, Serializable, Debug)]
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

#[test]
fn serial_size() {
    use crate::payload::Payload;
    let int1 = u256::from(567892322);
    let int2 = u256::from(7892322);
    let int3 = u256::from(1);
    let block_header = BlockHeader::new(
        23,
        shared::u256::from(12345678),
        shared::u256::from(9876543),
        2342,
        crate::block_header::Nbits::new(shared::u256::from(8719)),
        99,
    );

    let msg = MerkleBlock {
        block_header,
        transaction_count: 113,
        hashes: vec![int1, int2, int3],
        flags: Vec::from([232u8, 11]),
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
