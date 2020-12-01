use serde_derive::{Deserializable, Serializable};
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct SendCompact {
    announce: bool,
    version: u64,
}

impl crate::Payload for SendCompact {
    fn serialized_size(&self) -> usize {
        9
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut out)?;
        Ok(out)
    }
}
