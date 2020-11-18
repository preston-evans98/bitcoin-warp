pub struct SendHeaders {}

impl crate::payload::Payload for SendHeaders {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
