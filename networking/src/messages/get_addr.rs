pub struct GetAddr {}

impl crate::Payload for GetAddr {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
