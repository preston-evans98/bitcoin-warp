use hex;

use byteorder::{LittleEndian, WriteBytesExt};
use crypto::double_sha256;

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
}

pub trait Serializable {
    fn serialize(&self, target: &mut Vec<u8>);
}

impl Serializable for u32 {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.write_u32::<LittleEndian>(*self).unwrap();
    }
}

impl Serializable for &[u8] {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self)
    }
}
