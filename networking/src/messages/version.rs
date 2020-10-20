use crate::message::Message;
use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{Bytes, CompactInt, Deserializable, DeserializationError, Serializable};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

type Services = u64;
#[derive(Deserializable, Serializable)]
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
            user_agent: Vec::new(),
            best_block: best_block,
            relay: true,
        }
    }
}

impl crate::payload::Payload for Version {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let size = 85 + CompactInt::size(self.user_agent.len()) + self.user_agent.len();
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}

//Body of message.rs for reference
//Can be deleted after from method is made
// let mut msg = Message::new();
//                 // Should be 85 bytes (no user agent)
//                 // Generic info
//                 msg.body.append(config.get_protocol_version());
//                 msg.body.append(config.get_services());
//                 msg.body.append(secs_since_the_epoch());
//                 // Peer services and network info
//                 msg.body.append(*peer_services);
//                 msg.body.append(*peer_ip);
//                 // Self services and network info
//                 msg.body.append(config.get_services());
//                 msg.body.append(*daemon_ip);
//                 // Nonce and user agent
//                 msg.body.append(0 as u64);
//                 msg.body.append(CompactInt::from(0));
//                 //(OPTIONAL - Omitted) user agent string is optinal depending on number of bytes sent on line above
//                 // Best Block and Relay
//                 msg.body.append(*best_block);
//                 //(OPTIONAL - Omitted) relay flag
//                 msg.create_header_for_body(Command::Version, config.magic());
//                 return msg;

fn secs_since_the_epoch() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64() as u64
}
