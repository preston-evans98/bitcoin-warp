pub struct SendHeaders {}

impl crate::payload::Payload for SendHeaders {
    fn serialized_size(&self) -> usize {
        0
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}

#[test]
fn serial_size() {
    use crate::Payload;
    let msg = SendHeaders {};
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
