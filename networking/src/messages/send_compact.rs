use serde_derive::{Deserializable, Serializable};
use shared::Serializable;

#[derive(Serializable, Deserializable, Debug)]
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

#[test]
fn serial_size() {
    use crate::Payload;
    let msg = SendCompact {
        announce: true,
        version: 32381,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
