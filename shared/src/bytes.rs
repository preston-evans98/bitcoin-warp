use crate::Serializable;
use hex;
use warp_crypto::double_sha256;

#[derive(Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new() -> Bytes {
        Bytes(vec![])
    }

    pub fn from(target: Vec<u8>) -> Bytes {
        Bytes(target)
    }

    pub fn append<T>(&mut self, item: T)
    where
        T: Serializable,
    {
        item.serialize(&mut self.0)
            .expect("Serializing into a vec should not fail!");
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn double_sha256(&self) -> Vec<u8> {
        double_sha256(&self.0)
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.0
    }
    // pub fn append_big_endian<T>(&mut self, item: T)
    // where
    //     T: BigEndianSerializable,
    // {
    //     item.serialize(&mut self.0)
    // }
}

impl Serializable for &Bytes {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(&self.0)
    }
}
