use byteorder::{LittleEndian, ReadBytesExt};
use config::Config;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt, Serializable};
#[derive(Serializable, Deserializable)]
pub struct GetData {
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
impl shared::Serializable for InventoryType {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        (*self as u32).serialize(target)
    }
}
impl shared::Deserializable for InventoryType {
    fn deserialize<R>(target: &mut R) -> Result<Self, shared::DeserializationError>
    where
        R: std::io::Read,
    {
        let value = target.read_u32::<LittleEndian>()?;
        match value {
            1 => Ok(InventoryType::Tx),

            2 => Ok(InventoryType::Block),

            3 => Ok(InventoryType::FilteredBlock),

            4 => Ok(InventoryType::CompactBlock),

            5 => Ok(InventoryType::WitnessTx),

            6 => Ok(InventoryType::WitnessBlock),

            7 => Ok(InventoryType::FilteredWitnessBlock),
            _ => Err(shared::DeserializationError::Parse(format!(
                "Unreadable Inventory Type: {}",
                value
            ))),
        }
    }
}
#[derive(Serializable, Deserializable)]
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
    pub fn len(&self) -> usize {
        4 + 32 //inventory type  and hash
    }
}

impl GetData {
    pub fn new(temp_inventory: Vec<InventoryData>, _config: &Config) -> GetData {
        let message = GetData {
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
impl crate::payload::Payload for GetData {
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

#[test]
fn serial_size() {
    use crate::payload::Payload;
    use shared::u256;
    let int1 = u256::from(567892322);
    let int2 = u256::from(7892322);
    let int3 = u256::from(0);
    let t1 = InventoryType::FilteredBlock;
    let t2 = InventoryType::WitnessBlock;
    let t3 = InventoryType::Tx;
    let d1 = InventoryData {
        inventory_type: t1,
        hash: int1,
    };
    let d2 = InventoryData {
        inventory_type: t2,
        hash: int2,
    };
    let d3 = InventoryData {
        inventory_type: t3,
        hash: int3,
    };

    let inventory = Vec::from([d1, d2, d3]);
    let msg = GetData { inventory };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
