pub struct Mempool {}

impl crate::payload::Payload for Mempool {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
