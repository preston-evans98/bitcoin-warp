use crate::command::Command;
use crate::payload::Payload;
use crate::peer::Peer;
use config::Config;
use shared::{u256, Bytes, CompactInt};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};
use log::{warn};

pub struct Message {
    header: Bytes,
    body: Bytes,
    contents: Bytes,
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: Bytes::new(),
            body: Bytes::new(),
            contents: Bytes::new(),
        }
    }

    pub fn from(payload: &Payload, config: &Config) -> Message {
        match payload {
            Payload::VersionPayload {
                peer_ip,
                peer_services,
                daemon_ip,
                best_block,
            } => {
                let mut msg = Message::new();
                // Should be 85 bytes (no user agent)
                // Generic info
                msg.body.append(config.get_protocol_version());
                msg.body.append(config.get_services());
                msg.body.append(secs_since_the_epoch());
                // Peer services and network info
                msg.body.append(*peer_services);
                msg.body.append(*peer_ip);
                // Self services and network info
                msg.body.append(config.get_services());
                msg.body.append(*daemon_ip);
                // Nonce and user agent
                msg.body.append(0 as u64);
                msg.body.append(CompactInt::from(0));
                //(OPTIONAL - Omitted) user agent string is optinal depending on number of bytes sent on line above
                // Best Block and Relay
                msg.body.append(*best_block);
                //(OPTIONAL - Omitted) relay flag
                msg.create_header_for_body(Command::Version, config.magic());
                return msg;
            }
            Payload::GetBlocksPayload{
                block_hashes,
                inv_message
            } => {
                let mut msg = Message::new();
                msg.body.append(config.get_protocol_version()); //version number
                msg.body.append(CompactInt::from(block_hashes.len())); //hash count
                for hash in block_hashes.iter() {
                    msg.body.append(hash)
                }
                if *inv_message{
                    msg.body.append(u256::new());
                }
                else{
                    match block_hashes.last(){
                        Some(hash) =>{
                            msg.body.append(hash)
                        }
                        None => {
                            warn!("GetBlocks: stop hash was empty");
                            msg.body.append(u256::new());
                        }
                    } 
                }
                msg.create_header_for_body(Command::GetBlocks, config.magic());
                return msg;
            }

            // _ => Message::new(),
        }
    }
    pub fn create_header_for_body(&mut self, command: Command, magic: u32) {
        self.header.append(magic);
        self.header.append(command);
        self.header.append(self.body.len() as u32);
        self.header.append(&self.body.double_sha256()[..4]);

        self.contents.append(&self.header); //concatenating the header and body together
        self.contents.append(&self.body);
    }

    pub fn dump_header(&self) -> String {
        self.header.hex()
    }

    pub fn get_header(&self) -> &Bytes {
        &self.header
    }
    pub fn get_body(&self) -> &Bytes {
        &self.body
    }
    pub fn dump_body(&self) -> String {
        self.body.hex()
    }
    pub fn get_contents(&self) -> &Bytes {
        &self.contents
    }
    pub fn dump_contents(&self) -> String {
        self.contents.hex()
    }

    // pub fn create_getblocks_body(
    //     &mut self,
    //     block_hashes: &Vec<Bytes>,
    //     request_inventory: bool,
    //     config: &Config,
    // ) {
    // }

    // pub fn create_version_body(
    //     &mut self,
    //     self_addr: &SocketAddr,
    //     addr: &SocketAddr,
    //     protocol_version: u32,
    // ) {
    // }
}

fn secs_since_the_epoch() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64() as u64
}
