use shared::{Deserializable, DeserializationError, Serializable};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Version,
    Verack,
    GetBlocks,
    GetData,
    Block,
    GetHeaders,
}
impl Command {
    pub fn bytes(&self) -> &[u8; 12] {
        match self {
            Command::Version => b"version\0\0\0\0\0",
            Command::Verack => b"verack\0\0\0\0\0\0",
            Command::GetBlocks => b"getblocks\0\0\0",
            Command::GetData => b"getdata\0\0\0\0\0",
            Command::Block => b"block\0\0\0\0\0\0\0",
            Command::GetHeaders => b"getheaders\0\0",
        }
    }
}

// macro_rules! command_bytes {
//     // () => {
//     //     pub fn bytes(&self) -> &[u8; 12] {
//     //         match self {
//     //             Command::Version => b"version\0\0\0\0\0",
//     //             Command::Verack => b"verack\0\0\0\0\0\0",
//     //             Command::GetBlocks => b"getblocks\0\0\0",
//     //             Command::GetData => b"getdata\0\0\0\0\0",
//     //             Command::Block => b"block\0\0\0\0\0\0\0",
//     //             Command::GetHeaders => b"getheaders\0\0",
//     //         }
//     //     }
//     // };
// }

impl Serializable for Command {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(self.bytes())
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
