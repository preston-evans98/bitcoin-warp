use crate::command::Command;
use shared::Bytes;
use std::net::SocketAddr;

pub enum InventoryTypes{
    Tx = 1,
    Block = 2,
    FilteredBlock = 3,
    CompactBlock = 4,
    WitnessTx = 5,
    WitnessBlock = 6,
    FilteredWitnessBlock = 7,
}
pub struct Data{
    pub inventory_type : InventoryTypes,
    pub hash: Vec<Bytes>,
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
    GetDataPayload{
        inventory: Vec<Data>,
    }
}
