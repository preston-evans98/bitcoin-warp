use crate::command::Command;
use config::Config;
use shared::{u256, Bytes, CompactInt};

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
}
