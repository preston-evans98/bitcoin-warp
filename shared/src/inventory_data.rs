use crate::u256;
use crate::{Deserializable, DeserializationError, Serializable};
use bytes::{Buf, BytesMut};

#[derive(Copy, Clone, Debug)]
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
impl Deserializable for InventoryType {
    fn deserialize(target: &mut BytesMut) -> Result<Self, DeserializationError> {
        let value = u32::deserialize(target)?;
        match value {
            1 => Ok(InventoryType::Tx),

            2 => Ok(InventoryType::Block),

            3 => Ok(InventoryType::FilteredBlock),

            4 => Ok(InventoryType::CompactBlock),

            5 => Ok(InventoryType::WitnessTx),

            6 => Ok(InventoryType::WitnessBlock),

            7 => Ok(InventoryType::FilteredWitnessBlock),
            _ => Err(DeserializationError::Parse(format!(
                "Unreadable Inventory Type: {}",
                value
            ))),
        }
    }
}
#[derive(Debug)]
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

impl Serializable for InventoryData {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.inventory_type.serialize(target)?;
        self.hash.serialize(target)
    }
}

impl Deserializable for InventoryData {
    fn deserialize(target: &mut BytesMut) -> Result<Self, DeserializationError> {
        Ok(InventoryData {
            inventory_type: InventoryType::deserialize(target)?,
            hash: u256::deserialize(target)?,
        })
    }
}
