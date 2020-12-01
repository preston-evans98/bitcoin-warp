use serde_derive::{Deserializable, Serializable};
use shared::CompactInt;
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct GetBlockTxn {
    block_hash: [u8; 32],
    indexes: Vec<CompactInt>,
}

impl crate::Payload for GetBlockTxn {
    fn serialized_size(&self) -> usize {
        let mut len = 32;
        for index in self.indexes.iter() {
            len += CompactInt::size(index.value() as usize);
        }
        len
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut out)?;
        Ok(out)
    }
}
