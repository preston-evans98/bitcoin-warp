use crate::command::Command;
use config::Config;
use shared::{u256, Bytes, CompactInt};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

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

    pub fn create_header_for_body(&mut self, command: Command, config: &Config) {
        self.header.append(config.magic());
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

    pub fn create_getblocks_body(
        &mut self,
        block_hashes: &Vec<Bytes>,
        request_inventory: bool,
        config: &Config,
    ) {
        self.body.append(config.get_protocol_version()); //version number
        self.body.append(CompactInt::from(block_hashes.len())); //hash count
        for hash in block_hashes.iter() {
            self.body.append(hash)
        }
        if request_inventory {
            self.body.append(u256::new());
        }
    }

    pub fn create_version_body(
        &mut self,
        self_addr: &SocketAddr,
        addr: &SocketAddr,
        config: &Config,
    ) {
        // Should be 85 bytes (no user agent)
        self.body.append(config.get_protocol_version()); //version number 4
        self.body.append(01 as u64); //services of trasnmitting node 12

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64();

        self.body.append(since_the_epoch as u64); //timestamp 20
        self.body.append(01 as u64); //services of recieving address 28
        println!("Target ip: {}", addr.ip());
        self.body.append(addr.ip()); // ip addr of recieving node, need to pass in ip address from another method eventually TODO
        self.body.append_big_endian(addr.port() as u16); //receiving node port number
        self.body.append(01 as u64); //services of trasnmitting node
        println!("Own ip: {}", self_addr.ip());
        self.body.append(self_addr.ip()); //ip addr of transmitting node, need to pass in ip address from another method eventually TODO

        self.body.append_big_endian(self_addr.port() as u16); //transmitting node port number
        self.body.append(0 as u64); //nonce
        self.body.append(CompactInt::from(0)); //user agent
                                               //user agent string is optinal depending on number of bytes sent on line above
        self.body.append(1 as u32); //best block height
                                    //relay flag
    }
}
