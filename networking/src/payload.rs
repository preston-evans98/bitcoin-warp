use shared::{u256, Bytes, Serializable};
use std::net::SocketAddr;

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
    fn serialize(&self, target: &mut Vec<u8>) {
        (*self as u32).serialize(target)
    }
}
pub struct Data {
    pub inventory_type: InventoryType,
    pub hash: u256,
}

pub enum Payload<'a> {
    VersionPayload {
        peer_ip: &'a SocketAddr,
        peer_services: u64,
        daemon_ip: &'a SocketAddr,
        best_block: u32,
    },
    GetBlocksPayload {
        block_hashes: Vec<Bytes>,
        inv_message: bool,
    },
    GetDataPayload {
        inventory: Vec<Data>,
    },
}
