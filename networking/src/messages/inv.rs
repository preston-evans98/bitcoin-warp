// use byteorder::{LittleEndian, ReadBytesExt};
use crate::InventoryData;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct Inv {
    inventory: Vec<InventoryData>,
}
impl Inv {
    pub fn new(temp_inventory: Vec<InventoryData>) -> Inv {
        Inv {
            inventory: temp_inventory,
        }
    }
}
impl crate::payload::Payload for Inv {
    fn serialized_size(&self) -> usize {
        let mut size = CompactInt::size(self.inventory.len());
        for inv in self.inventory.iter() {
            size += inv.len();
        }
        size
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}
