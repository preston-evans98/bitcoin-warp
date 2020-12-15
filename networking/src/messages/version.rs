use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{net::SocketAddr, sync::Arc};

type Services = u64;
#[derive(Deserializable, Serializable, Debug)]
pub struct Version {
    protocol_version: u32,
    services: Services,
    timestamp: u64,
    receiver_services: Services,
    receiver: SocketAddr,
    transmitter_services: Services,
    transmitter_ip: SocketAddr,
    nonce: u64,
    user_agent: Vec<u8>,
    best_block: u32,
    relay: bool,
}
impl Version {
    pub fn new(
        peer_ip: SocketAddr,
        peer_services: u64,
        daemon_ip: SocketAddr,
        best_block: u32,
        config: &Arc<Config>,
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
            user_agent: Vec::new(),
            best_block: best_block,
            relay: true,
        }
    }
}

impl crate::payload::Payload for Version {
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
    use crate::payload::Payload;

    let msg = Version::new(
        ([192, 168, 0, 1], 8333).into(),
        2371,
        ([192, 168, 0, 2], 8333).into(),
        0x2329381,
        &Arc::new(config::Config::mainnet()),
    );
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
