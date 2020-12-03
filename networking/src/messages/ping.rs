use serde_derive::{Deserializable, Serializable};
use shared::Serializable;
#[derive(Serializable, Deserializable, Debug)]
pub struct Ping {
    nonce: u64,
}

impl crate::Payload for Ping {
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
    let msg = Ping { nonce: 34567 };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
