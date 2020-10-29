use byteorder::{LittleEndian, ReadBytesExt};
use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};
use crate::get_data::{InventoryType,InventoryData};
#[derive(Serializable, Deserializable)]
pub struct Inv {
    inventory: Vec<InventoryData>,
}
impl Inv {
    pub fn new(temp_inventory: Vec<InventoryData>, config: &Config) -> Inv {
        let mut message = Inv {
            inventory: temp_inventory,
        };
        // for x in inventory.iter() {
        //     message.inventory.append(
        //         InventoryData{
        //             x.inventory_type,
        //             x.hash,
        //         }
        //     );
        // }
        //message.create_header_for_body(Command::GetData, config.magic());
        message
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut size = 0;
        for inv in self.inventory.iter() {
            size += inv.len();
        }
        let mut target = Vec::with_capacity(size + CompactInt::size(self.inventory.len()));
        self.serialize(&mut target)?;
        Ok(target)
    }
}
