use config::Config;
use shared::{u256, CompactInt, Serializable};
pub struct GetData {
    hash_count: CompactInt,
    inventory: Vec<InventoryData>,
}
#[derive(Copy, Clone)]
pub enum InventoryType {
    Tx = 1,
    Block = 2,
    FilteredBlock = 3,
    CompactBlock = 4,
    WitnessTx = 5,
    WitnessBlock = 6,
    FilteredWitnessBlock = 7,
}
impl Serializable for InventoryType {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        (*self as u32).serialize(target)
    }
}
pub struct InventoryData {
    pub inventory_type: InventoryType,
    pub hash: u256,
}
impl InventoryData {
    pub fn from(inv: InventoryType, hash: u256) -> InventoryData {
        InventoryData {
            inventory_type: inv,
            hash: hash,
        }
    }
}

impl GetData {
    pub fn new(temp_inventory: Vec<InventoryData>, config: &Config) -> GetData {
        let mut message = GetData {
            hash_count: CompactInt::from(temp_inventory.len()),
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
}
