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

#[test]
fn serial_size() {
    use crate::payload::Payload;
    let addr1 = EncapsulatedAddr {
        time: 1,
        services: 1,
        addr: ([192, 168, 0, 1], 8333).into(),
    };
    let addr2 = EncapsulatedAddr {
        time: 1,
        services: 1,
        addr: ([192, 168, 0, 1], 8333).into(),
    };
    let mut addrs = Vec::with_capacity(2);
    addrs.push(addr1);
    addrs.push(addr2);
    let msg = Addr { addrs };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
