use crate::command::Command;
use config::Config;
use shared::{u256, Bytes, CompactInt};
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::Ipv4Addr;

pub struct Message {
    header: Bytes,
    body: Bytes,
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: Bytes::new(),
            body: Bytes::new(),
        }
    }

    pub fn create_header_for_body(&mut self, command: Command, config: &Config) {
        self.header.append(config.magic());
        self.header.append(command);
        self.header.append(self.body.len() as u32);
        self.header.append(&self.body.double_sha256()[..4])
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

    pub fn create_version_body(&mut self, config: &Config){
        self.body.append(config.get_protocol_version());  //version number
        self.body.append(010 as u64); //services of trasnmitting node 

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64();
        println!("{:?}", since_the_epoch);
        self.body.append(since_the_epoch as u64); //timestamp
        self.body.append(010 as u64); //services of recieving address
        self.body.append(Ipv4Addr::new(192, 168, 1, 2).to_ipv6_mapped());// ip addr of recieving node, need to pass in ip address from another method eventually TODO
        self.body.append(8333 as u16); //receiving node port number
        self.body.append(010 as u64); //services of trasnmitting node 
        self.body.append(Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped());//ip addr of transmitting node, need to pass in ip address from another method eventually TODO
        self.body.append(8333 as u16); //transmitting node port number
        self.body.append(0 as u64); //nonce
        self.body.append(0b0 as u8); //user agent
        //user agent string is optinal depending on number of bytes sent on line above
        self.body.append(0 as u32 ); //best block height
        
        
    }
}
