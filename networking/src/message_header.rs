use crate::command::Command;
use bytes::BytesMut;
use serde_derive::Deserializable;
use shared::{Deserializable, DeserializationError};

/// A [Bitcoin Wire Protocol](https://developer.bitcoin.org/reference/p2p_networking.html) [Message Header](https://developer.bitcoin.org/reference/p2p_networking.html#message-headers).
///
/// This struct is encapsulated by the [`BitcoinCodec`](crate::BitcoinCodec), which automatically creates and sends Message Headers at serialization time.
/// Most users should not need to interact with this struct, but
/// it is is exported as a convencience to those who don't wish to use the Codec.
#[derive(Deserializable, Debug)]
pub struct MessageHeader {
    magic: u32,
    command: Command,
    payload_size: u32,
    checksum: [u8; 4],
}

impl MessageHeader {
    pub fn deserialize(
        target: &mut BytesMut,
        expected_magic: u32,
    ) -> Result<MessageHeader, DeserializationError> {
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
        24
    }
}
