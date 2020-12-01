use serde_derive::{Deserializable, Serializable};
use shared::CompactInt;
use shared::Serializable;
use std::net::SocketAddr;

#[derive(Serializable, Deserializable)]
pub struct Addr {
    addrs: Vec<EncapsulatedAddr>,
}

#[derive(Serializable, Deserializable)]
pub struct EncapsulatedAddr {
    time: u32,
    services: u64,
    addr: SocketAddr,
}

impl crate::payload::Payload for Addr {
    fn serialized_size(&self) -> usize {
        CompactInt::size(self.addrs.len()) + self.addrs.len() * (4 + 8 + 18)
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}
