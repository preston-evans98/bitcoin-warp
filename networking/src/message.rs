use crate::command::Command;
use crate::peer::Peer;
use config::Config;
use shared::{u256, Bytes, CompactInt};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::payload::Payload;

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
        match payload{
            Payload::VersionPayload{ref command, peer_id, daemon_ip}=> {
                let mut msg = Message::new();

                // Should be 85 bytes (no user agent)
                msg.body.append(config.get_protocol_version()); //version number 4
                msg.body.append(01 as u64); //services of trasnmitting node 12

                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs_f64();

                msg.body.append(since_the_epoch as u64); //timestamp 20
                msg.body.append(01 as u64); //services of recieving address 28
                println!("Target ip: {}", payload.peer_ip.ip());
                msg.body.append(payload.peer_ip.ip()); // ip addr of recieving node, need to pass in ip address from another method eventually TODO
                msg.body
                    .append_big_endian(payload.peer_ip.port() as u16); //receiving node port number
                msg.body.append(01 as u64); //services of trasnmitting node
                println!("Own ip: {}", payload.daemon_ip.ip());
                msg.body.append(payload.daemon_ip.ip()); //ip addr of transmitting node, need to pass in ip address from another method eventually TODO

                msg.body
                    .append_big_endian(payload.daemon_ip.port() as u16); //transmitting node port number
                msg.body.append(0 as u64); //nonce
                msg.body.append(CompactInt::from(0)); //user agent
                                                      //user agent string is optinal depending on number of bytes sent on line above
                msg.body.append(1 as u32); //best block height
                                           //relay flag
                msg.create_header_for_body(payload.command, config.magic());
                return msg;
            }
            Command::GetBlocks => {
                let msg = Message::new();
                
                msg.body.append(config.get_protocol_version()); //version number
                msg.body.append(CompactInt::from(payload.block_hashes.len())); //hash count
                // for hash in block_hashes.iter() {
                //     msg.body.append(hash)
                // }
                // if request_inventory {
                //     msg.body.append(u256::new());
                // }
                // msg.create_header_for_body(Command::GetBlocks, config.magic());
                return msg;
            }
            _ => Message::new(), // Command::
                                 // _ => {
                                 //     let msg = Message::new();
                                 //     msg
                                 // }
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
