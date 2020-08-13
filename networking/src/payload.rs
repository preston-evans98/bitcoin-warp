use std::net::SocketAddr;
use crate::command::Command;
use shared::{Bytes};


pub enum Payload{
    VersionPayload{
        command: Command,
		peer_ip: &SocketAddr,
		daemon_ip: &SocketAddr
    },
    GetBlocks{
        command: Command,
        block_hashes: Vec<Bytes>,
    }
}