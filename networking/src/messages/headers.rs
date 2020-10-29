use crate::block_header::BlockHeader;
use shared::{CompactInt, Deserializable, DeserializationError, Serializable};
pub struct Headers {
    headers: Vec<BlockHeader>,
}

impl Serializable for Headers {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        CompactInt::from(self.headers.len()).serialize(target)?;
        for item in self.headers.iter() {
            item.serialize(target)?;
            0u8.serialize(target)?;
        }
        Ok(())
    }
}

impl Deserializable for Headers {
    fn deserialize<R>(reader: &mut R) -> Result<Headers, DeserializationError>
    where
        R: std::io::Read,
    {
        let count = CompactInt::deserialize(reader)?;
        let mut result = Vec::with_capacity(count.value() as usize);
        for _ in 0..result.len() {
            result.push(BlockHeader::deserialize(reader)?);
            let _ = u8::deserialize(reader)?;
        }
        Ok(Headers { headers: result })
    }
}

impl crate::payload::Payload for Headers {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let len =
            self.headers.len() * (BlockHeader::len() + 1) + CompactInt::size(self.headers.len());
        let mut result = Vec::with_capacity(len);
        self.serialize(&mut result)?;
        Ok(result)
    }
}
