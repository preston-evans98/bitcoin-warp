use crate::InventoryData;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct NotFound {
    inventory_data: Vec<InventoryData>,
}
impl NotFound {
    pub fn new() -> NotFound {
        NotFound {
            inventory_data: Vec::new(),
        }
    }
}
impl crate::payload::Payload for NotFound {
    fn serialized_size(&self) -> usize {
        let mut size = CompactInt::size(self.inventory_data.len());
        for inv in self.inventory_data.iter() {
            size += inv.len()
        }
        size
    }
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut target = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut target)?;
        Ok(target)
    }
}
