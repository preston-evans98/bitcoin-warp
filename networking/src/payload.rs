pub trait Payload: shared::Deserializable + shared::Deserializable + std::fmt::Debug {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>;
    ////// serialized_size excludes the size of the messsage header
    fn serialized_size(&self) -> usize;
}
