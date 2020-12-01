use crate::Payload;
use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable)]
pub struct FeeFilter {
    feerate: u64,
}

impl Payload for FeeFilter {
    fn serialized_size(&self) -> usize {
        8
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}
