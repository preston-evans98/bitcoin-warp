use shared::{Deserializable, DeserializationError, Serializable};

#[derive(Debug, Clone)]
pub enum Command {
    Version,
    Verack,
    GetBlocks,
}
impl Command {
    pub fn bytes(&self) -> &[u8; 12] {
        match self {
            Command::Version => b"version\0\0\0\0\0",
            Command::Verack => b"verack\0\0\0\0\0\0",
            Command::GetBlocks => b"getblocks\0\0\0",
            // Command::Ge
        }
    }
}

impl Serializable for Command {
    fn serialize(&self, target: &mut Vec<u8>) {
        target.extend_from_slice(self.bytes())
    }
}

impl Deserializable for Command {
    fn deserialize<T>(reader: &mut T) -> Result<Command, DeserializationError>
    where
        T: std::io::Read,
    {
        let mut buf = [0u8; 12];
        reader.read_exact(&mut buf)?;
        let command = match &buf {
            b"version\0\0\0\0\0" => Command::Version,
            b"verack\0\0\0\0\0\0" => Command::Verack,
            b"getblocks\0\0\0" => Command::GetBlocks,
            _ => return Err(DeserializationError::parse(&buf, "Command")),
        };
        Ok(command)
    }
}
