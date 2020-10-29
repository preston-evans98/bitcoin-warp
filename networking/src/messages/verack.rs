pub struct Verack {}

impl crate::payload::Payload for Verack {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}