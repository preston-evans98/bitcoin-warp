use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable)]
pub struct Pong {
    nonce: u64,
}

impl crate::Payload for Pong {
    fn serialized_size(&self) -> usize {
        8
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}

#[test]
fn serial_size() {
    use crate::Payload;
    let msg = Pong { nonce: 34567 };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
