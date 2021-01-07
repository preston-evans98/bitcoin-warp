use std::net::SocketAddr;

use crate::{Deserializable, DeserializationError, Serializable};

#[derive(Debug)]
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
    fn deserialize<R>(target: &mut R) -> Result<Self, DeserializationError>
    where
        R: std::io::Read,
    {
        Ok(EncapsulatedAddr {
            time: u32::deserialize(target)?,
            services: u64::deserialize(target)?,
            addr: SocketAddr::deserialize(target)?,
        })
    }
}
