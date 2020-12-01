use serde_derive::{Deserializable, Serializable};
use shared::CompactInt;
use shared::Serializable;

#[derive(Serializable, Deserializable)]
pub struct GetBlockTxn {
    block_hash: [u8; 32],
    indexes: Vec<CompactInt>,
}

impl crate::Payload for GetBlockTxn {
    fn serialized_size(&self) -> usize {
        let mut len = 32 + CompactInt::size(self.indexes.len());
        for index in self.indexes.iter() {
            len += CompactInt::size(index.value() as usize);
        }
        len
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut out)?;
        Ok(out)
    }
}

#[test]
fn serial_size() {
    use crate::payload::Payload;
    use shared::CompactInt;
    let int1 = CompactInt::from(567892322);
    let int2 = CompactInt::from(7892322);
    let int3 = CompactInt::from(0);
    let msg = GetBlockTxn {
        block_hash: [242u8; 32],
        indexes: Vec::from([int1, int2, int3]),
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
