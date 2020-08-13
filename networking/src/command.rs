use shared::Serializable;

#[derive(Debug)]
pub enum Command {
    Version,
    Verack,
    GetBlocks
}
impl Command {
    pub fn bytes(&self) -> &[u8; 12] {
        match self {
            Command::Version => b"version\0\0\0\0\0",
            Command::Verack => b"verack\0\0\0\0\0\0",
            Command::GetBlocks => b"getblocks\0\0\0",
            Command::Ge
        }
    }
}

impl Serializable for Command {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self.bytes())
    }
}
