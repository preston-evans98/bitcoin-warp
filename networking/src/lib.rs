use config::Config;
use shared::{Bytes, Serializable};

pub struct Message {
    header: Bytes,
    body: Bytes,
}

#[derive(Debug)]
pub enum Command {
    Version,
    Verack,
}
impl Command {
    fn bytes(&self) -> &[u8; 12] {
        match self {
            Command::Version => b"version\0\0\0\0\0",
            Command::Verack => b"verack\0\0\0\0\0\0",
        }
    }
}

impl Serializable for Command {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self.bytes())
    }
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
}

#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::Message;
    use config::Config;
    #[test]
    fn test_verack() {
        let mut message = Message::new();
        message.create_header(Command::Verack, &Config::mainnet());
        eprintln!("Header: {:?} ", message.header);
    }
}
