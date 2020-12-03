use crate::command::Command;
use serde_derive::{Deserializable, Serializable};
use shared::{Deserializable, DeserializationError, Serializable};

#[derive(Deserializable, Serializable)]
pub struct Header {
    magic: u32,
    command: Command,
    payload_size: u32,
    checksum: [u8; 4],
}

impl Header {
    pub fn deserialize<T>(
        target: &mut T,
        expected_magic: u32,
    ) -> Result<Header, DeserializationError>
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
        Ok(Header {
            magic,
            command,
            payload_size,
            checksum,
        })
    }
    pub fn get_command(&self) -> Command {
        self.command.clone()
    }
    pub fn from_body(magic: u32, command: Command, body: &Vec<u8>) -> Header {
        let hash = warp_crypto::double_sha256(body);
        let checksum = [hash[0], hash[1], hash[2], hash[3]];
        Header {
            magic,
            command,
            payload_size: body.len() as u32,
            checksum,
        }
    }
    pub fn get_payload_size(&self) -> usize {
        self.payload_size as usize
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(24);
        self.serialize(&mut result)
            .expect("Serializing to vec should not fail!");
        result
    }
}
