use crate::InventoryData;
use serde_derive::{Deserializable, Serializable};
use shared::{CompactInt, Serializable};
#[derive(Serializable, Deserializable, Debug)]
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

#[test]
fn serial_size() {
    use crate::payload::Payload;
    use crate::{InventoryData, InventoryType};
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
    let msg = NotFound {
        inventory_data: inventory,
    };
    let serial = msg.to_bytes().expect("Serializing into vec shouldn't fail");
    assert_eq!(serial.len(), msg.serialized_size());
    assert_eq!(serial.len(), serial.capacity())
}
