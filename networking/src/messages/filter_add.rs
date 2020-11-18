use crate::Payload;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct FilterAdd {
    elements: Vec<Vec<u8>>,
}

impl Payload for FilterAdd {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let size = self.elements.iter().fold(0, |total, elt| {
            total + elt.len() + CompactInt::size(elt.len())
        }) + CompactInt::size(self.elements.len());
        let mut result = Vec::with_capacity(size);
        self.serialize(&mut result)?;
        Ok(result)
    }
}
