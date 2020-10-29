use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
use shared::CompactInt;
use shared::Serializable;
#[derive(Deserializable, Serializable)]
pub struct Headers {
    headers: Vec<BlockHeader>,
}

impl crate::payload::Payload for Headers {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let len = self.headers.len() * BlockHeader::len() + CompactInt::size(self.headers.len());
        let mut result = Vec::with_capacity(len);
        self.serialize(&mut result)?;
        Ok(result)
    }
}
