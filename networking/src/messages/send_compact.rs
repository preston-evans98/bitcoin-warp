use serde_derive::{Deserializable, Serializable};
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct SendCompact {
    announce: bool,
    version: u64,
}

impl crate::Payload for SendCompact {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::with_capacity(9);
        self.serialize(&mut out)?;
        Ok(out)
    }
}
