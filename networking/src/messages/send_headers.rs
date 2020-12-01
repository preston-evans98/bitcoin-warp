pub struct SendHeaders {}

impl crate::payload::Payload for SendHeaders {
    fn serialized_size(&self) -> usize {
        0
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
