use crate::command::Command;
use shared::Bytes;
use std::net::SocketAddr;

pub enum Payload<'a> {
    VersionPayload {
        peer_ip: &'a SocketAddr,
        peer_services: u64,
        daemon_ip: &'a SocketAddr,
        best_block: u32,
    },
    GetBlocks {
        command: Command,
        block_hashes: Vec<Bytes>,
    },
}
