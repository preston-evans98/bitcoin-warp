use bytes::Buf;
use serde_derive::Deserializable;
use shared::CompactInt;
use shared::Serializable;
#[derive(Deserializable, Debug)]
#[allow(non_snake_case)]
pub struct FilterLoad {
    filter: Vec<u8>,
    nHashFuncs: u32,
    nTweak: u32,
    nFlags: u8,
}

impl Serializable for FilterLoad {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.filter.serialize(target)?;
        self.nHashFuncs.serialize(target)?;
        self.nTweak.serialize(target)?;
        target.write_all(&[self.nFlags])?;
        Ok(())
    }
}
impl super::Payload for FilterLoad {
    fn serialized_size(&self) -> usize {
        CompactInt::size(self.filter.len()) + self.filter.len() + 4 + 4 + 1
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut result = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut result)?;
        Ok(result)
    }
}

#[test]
fn serial_size() {
    use super::Payload;
    let msg = FilterLoad {
        filter: Vec::from([23u8; 22]),
        nHashFuncs: 0x129381,
        nTweak: 0xf2391381,
        nFlags: 32,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
