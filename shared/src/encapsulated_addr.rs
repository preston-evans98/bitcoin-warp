use crate::{Deserializable, DeserializationError, Serializable};
use bytes::Buf;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct EncapsulatedAddr {
    time: u32,
    services: u64,
    addr: SocketAddr,
}

impl EncapsulatedAddr {
    pub fn new(time: u32, services: u64, addr: SocketAddr) -> EncapsulatedAddr {
        EncapsulatedAddr {
            time,
            services,
            addr,
        }
    }
    pub fn time(&self) -> u32 {
        self.time
    }
    pub fn services(&self) -> u64 {
        self.services
    }
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}

impl Serializable for EncapsulatedAddr {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.time.serialize(target)?;
        self.services.serialize(target)?;
        self.addr.serialize(target)
    }
}

impl Deserializable for EncapsulatedAddr {
    fn deserialize<B: Buf>(mut target: B) -> Result<Self, DeserializationError> {
        Ok(EncapsulatedAddr {
            time: u32::deserialize(&mut target)?,
            services: u64::deserialize(&mut target)?,
            addr: SocketAddr::deserialize(&mut target)?,
        })
    }
}
