pub struct FilterClear {}

impl crate::payload::Payload for FilterClear {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        Ok(Vec::new())
    }
}
