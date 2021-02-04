use super::ProtocolVersion;
use bytes::Buf;
use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{BlockHash, CompactInt, Serializable};
use tracing::warn;
#[derive(Serializable, Deserializable, Debug, Clone)]
pub struct GetHeaders {
    protocol_version: ProtocolVersion,
    block_header_hashes: Vec<BlockHash>,
    stop_hash: BlockHash,
}

impl GetHeaders {
    pub fn new(block_hashes: Vec<BlockHash>, inv_message: bool, config: &Config) -> GetHeaders {
        let message = GetHeaders {
            protocol_version: config.get_protocol_version(),
            block_header_hashes: block_hashes,
            stop_hash: BlockHash::from([0u8; 32]),
        };
        if !inv_message {
            //The header hash of the last header hash being requested; set to all zeroes to request an “inv” message
            //with all subsequent header hashes (a maximum of 2000 will be sent as a reply to this message;
            //if you need more than 2000, you will need to send another "getheaders" message with a higher-height
            //header hash as the first entry in block header hash field).
            match message.block_header_hashes.last() {
                Some(_) => {} // message.stop_hash = *hash.clone(),
                None => {
                    warn!("GetHeaders: stop hash was empty");
                }
            }
        }
        message
    }
}
impl super::Payload for GetHeaders {
    fn serialized_size(&self) -> usize {
        4 + CompactInt::size(self.block_header_hashes.len())
            + (self.block_header_hashes.len() * 32)
            + 32 //protocol version, block header hashes, and stop_hash
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}

#[test]
fn serial_size() {
    use super::Payload;
    let int1 = BlockHash::from_u64(567892322);
    let int2 = BlockHash::from_u64(7892322);
    let int3 = BlockHash::from_u64(1);
    let msg = GetHeaders {
        protocol_version: 32371,
        block_header_hashes: Vec::from([int1, int2, int3]),
        stop_hash: BlockHash::from([0u8; 32]),
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
