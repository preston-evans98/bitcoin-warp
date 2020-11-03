use serde_derive::{Deserializable, Serializable};
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
