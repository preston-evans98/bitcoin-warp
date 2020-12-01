use crate::block_header::BlockHeader;
use byteorder::WriteBytesExt;
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
            target.write_u8(0)?;
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
    fn serialized_size(&self) -> usize {
        self.headers.len() * (BlockHeader::len() + 1) + CompactInt::size(self.headers.len())
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}

#[test]
fn serial_size() {
    use crate::payload::Payload;

    let h1 = BlockHeader::new(
        23,
        shared::u256::from(12345678),
        shared::u256::from(9876543),
        2342,
        crate::block_header::Nbits::new(shared::u256::from(8719)),
        99,
    );
    let h2 = BlockHeader::new(
        0,
        shared::u256::from(2),
        shared::u256::from(88),
        2198321,
        crate::block_header::Nbits::new(shared::u256::from(0xf32231)),
        82,
    );

    let headers = Vec::from([h1, h2]);

    let msg = Headers { headers };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
