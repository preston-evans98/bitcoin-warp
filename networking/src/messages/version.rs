use shared::{Bytes, CompactInt, Deserializable, DeserializationError};
use std::net::SocketAddr;

type Services = u64;
// #[derive(Deserializable)]
pub struct Version {
    protocol_version: u32,
    services: Services,
    timestamp: u64,
    receiver_services: Services,
    receiver: SocketAddr,
    transmitter_services: Services,
    transmitter_ip: SocketAddr,
    user_agent_size: CompactInt,
    user_agent: Bytes,
    best_block: u32,
    relay: bool,
}
