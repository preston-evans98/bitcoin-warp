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

    pub fn create_header(&mut self, command: Command, config: &Config) {
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
}
