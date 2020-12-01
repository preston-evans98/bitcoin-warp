use crate::Payload;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct FilterAdd {
    elements: Vec<Vec<u8>>,
}

impl Payload for FilterAdd {
    fn serialized_size(&self) -> usize {
        self.elements.iter().fold(0, |total, elt| {
            total + elt.len() + CompactInt::size(elt.len())
        }) + CompactInt::size(self.elements.len())
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}

#[test]
fn serial_size() {
    let inner1 = Vec::from([1u8; 32]);
    let inner2 = Vec::from([7u8; 11]);
    let outer = Vec::from([inner1, inner2]);
    let msg = FilterAdd { elements: outer };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
