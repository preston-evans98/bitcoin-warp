use crate::command::Command;
use serde_derive::Deserializable;
use shared::{Deserializable, DeserializationError};

#[derive(Deserializable, Debug)]
pub struct MessageHeader {
    magic: u32,
    command: Command,
    payload_size: u32,
    checksum: [u8; 4],
}

impl MessageHeader {
    pub fn deserialize<T>(
        target: &mut T,
        expected_magic: u32,
    ) -> Result<MessageHeader, DeserializationError>
    where
        T: std::io::Read,
    {
        let magic = u32::deserialize(target)?;
        if magic != expected_magic {
            return Err(DeserializationError::parse(&magic.to_le_bytes(), "magic"));
        }
        let command = Command::deserialize(target)?;
        let payload_size = u32::deserialize(target)?;
        let checksum = <[u8; 4]>::deserialize(target)?;
        Ok(MessageHeader {
            magic,
            command,
            payload_size,
            checksum,
        })
    }
    pub fn get_command(&self) -> Command {
        self.command.clone()
    }

    pub fn get_payload_size(&self) -> usize {
        self.payload_size as usize
    }

    pub fn len() -> usize {
        20
    }
}
