use crate::command::Command;
use crate::message::Message;
use config::Config;
use std::net::SocketAddr;

pub enum VersionMessageType {}
pub struct Peer {
    peer_id: usize,
    services: u32,
    version: u32,
    version_type: VersionMessageType,
    ip_address: SocketAddr,
    nonce: u64,
    daemon_address: SocketAddr,
    magic: u32,
    protocol_version: u32,
}

impl Peer {
    pub fn send(&self, command: Command) {
        let mut msg = Message::new();
        match command {
            Command::Version => {
                msg.create_version_body(
                    &self.daemon_address,
                    &self.ip_address,
                    self.protocol_version,
                );
            }
            Command::Verack => {}
            Command::GetBlocks => {
                // msg.create_getblocks_body(block_hashes: &Vec<Bytes>, request_inventory: false, config: &Config)
            }
        }
        msg.create_header_for_body(command, self.magic);
    }

    pub async fn receive(&self) -> Command {
        //deserialization call here and return the message
        Command::Verack
    }
}
