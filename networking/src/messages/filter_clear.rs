use serde_derive::{Deserializable, Serializable};
#[derive(Serializable, Deserializable, Debug)]
pub struct FilterClear {}

impl crate::payload::Payload for FilterClear {
    fn serialized_size(&self) -> usize {
        0
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
#[test]
fn serial_size() {
    use crate::payload::Payload;
    let msg = FilterClear {};
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
