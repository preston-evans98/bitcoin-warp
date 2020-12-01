use serde_derive::Deserializable;
use shared::CompactInt;
use shared::Serializable;
#[derive(Deserializable)]
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
impl crate::Payload for FilterLoad {
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let size = CompactInt::size(self.filter.len()) + self.filter.len() + 9;
        let mut result = Vec::with_capacity(size);
        self.serialize(&mut result)?;
        Ok(result)
    }
}