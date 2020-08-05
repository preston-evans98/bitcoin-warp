use crate::command::Command;
use config::Config;
use shared::Bytes;

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

    pub fn create_getblocks_body(&mut self, _block_hashes: &Vec<Bytes>, config: &Config){  
        self.body.append(config.get_protocol_version());  //version number
        self.body.append(00 as u8); //hash count
        //self.body.append(0000000000000000000000000000000000000000000000000000000000000000); //last block header hashes from heighest height to lowest-height 
        self.body.append(0 as u64); //the header hash of the last header hash being requested
        self.body.append(0 as u64); //the header hash of the last header hash being requested
        self.body.append(0 as u64); //the header hash of the last header hash being requested
        self.body.append(0 as u64); //the header hash of the last header hash being requested

        // Will need to get a list of the last blocks from the database and insert them here TODO
    }


}
