use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable)]
pub struct Ping {
    nonce: u64,
}

impl crate::Payload for Ping {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(8);
        self.serialize(&mut result)?;
        Ok(result)
    }
}
