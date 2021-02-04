use super::{Nonce, ProtocolVersion, Services};
use bytes::Buf;
use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserializable, Serializable, Debug, Clone)]
pub struct Version {
    protocol_version: ProtocolVersion,
    services: Services,
    timestamp: u64,
    receiver_services: Services,
    receiver: SocketAddr,
    transmitter_services: Services,
    transmitter_ip: SocketAddr,
    nonce: Nonce,
    user_agent: String,
    best_block: u32,
    relay: bool,
}
impl Version {
    pub fn new(
        peer_ip: SocketAddr,
        peer_services: u64,
        daemon_ip: SocketAddr,
        best_block: u32,
        config: &Config,
    ) -> Version {
        Version {
            protocol_version: config.get_protocol_version(),
            services: config.get_services(),
            timestamp: secs_since_the_epoch(),
            receiver_services: peer_services,
            receiver: peer_ip,
            transmitter_services: config.get_services(),
            transmitter_ip: daemon_ip,
            nonce: 0 as u64,
            user_agent: String::new(),
            best_block: best_block,
            relay: true,
        }
    }
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
    pub fn services(&self) -> Services {
        self.services
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn receiver_services(&self) -> Services {
        self.receiver_services
    }
    pub fn receiver(&self) -> &SocketAddr {
        &self.receiver
    }
    pub fn transmitter_services(&self) -> Services {
        self.transmitter_services
    }
    pub fn transmitter_ip(&self) -> &SocketAddr {
        &self.transmitter_ip
    }
    pub fn nonce(&self) -> Nonce {
        self.nonce
    }
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }
    pub fn best_block(&self) -> u32 {
        self.best_block
    }
    pub fn relay(&self) -> bool {
        self.relay
    }
}

impl super::Payload for Version {
    fn serialized_size(&self) -> usize {
        85 + CompactInt::size(self.user_agent.len()) + self.user_agent.len()
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}

fn secs_since_the_epoch() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64() as u64
}

#[test]
fn serial_size() {
    use super::Payload;

    let msg = Version::new(
        ([192, 168, 0, 1], 8333).into(),
        2371,
        ([192, 168, 0, 2], 8333).into(),
        0x2329381,
        &config::Config::mainnet(),
    );
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
