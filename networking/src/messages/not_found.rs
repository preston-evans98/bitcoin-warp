use crate::get_data::{InventoryData};
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct NotFound{
    inventory_data: Vec<InventoryData>
}
impl NotFound{
    pub fn new() -> NotFound{
        NotFound{
            inventory_data: Vec::new()
        }
    }
}
impl crate::payload::Payload for NotFound{
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        size += CompactInt::size(self.inventory_data.len());
        let mut target = Vec::with_capacity(size);
        self.serialize(&mut target)?;
        Ok(target)
    }
}