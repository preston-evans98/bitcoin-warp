pub trait Payload {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>;
}
