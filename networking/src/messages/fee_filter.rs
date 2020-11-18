use crate::Payload;
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable)]
pub struct FeeFilter {
    feerate: u64,
}

impl Payload for FeeFilter {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(8);
        self.serialize(&mut result)?;
        Ok(result)
    }
}
