pub struct SendCompact{
    announce: u8,
    version: u64
}
impl crate::payload::Payload for SendCompact{
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        size += 1 + 8;
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}