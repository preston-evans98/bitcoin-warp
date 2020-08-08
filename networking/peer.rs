use std::net::{ SocketAddr};

pub enum VersionMessageType{
    
}
pub struct Peer{
    pub peer_id: usize,
    pub services: u32,
    pub version: u32,
    pub version_type: VersionMessageType,
    pub ip_address: SocketAddr,
}