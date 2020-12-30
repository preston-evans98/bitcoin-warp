// use crate::command::Command;
// use crate::header::Header;
// use shared::Bytes;
// use tokio::net::TcpStream;

use crate::block_header::BlockHeader;
use crate::transaction::Transaction;
use crate::InventoryData;
use serde_derive::{Deserializable, Serializable};
use shared::{u256, CompactInt};
use std::net::SocketAddr;

#[derive(Serializable, Deserializable, Debug)]
pub struct EncapsulatedAddr {
    time: u32,
    services: u64,
    addr: SocketAddr,
}

#[derive(Serializable, Deserializable, Debug)]
pub struct PrefilledTransaction {
    index: CompactInt,
    pub tx: Transaction,
}

type Services = u64;
type Nonce = u64;
type Version = u32;

#[derive(Debug, Serializable)]
pub enum Message {
    Addr {
        addrs: Vec<EncapsulatedAddr>,
    },
    BlockTxn {
        block_hash: [u8; 32],
        txs: Vec<Transaction>,
    },
    Block {
        block_header: BlockHeader,
        transactions: Vec<Transaction>,
    },
    CompactBlock {
        header: BlockHeader,
        nonce: Nonce,
        short_ids: Vec<u64>,
        prefilled_txns: Vec<PrefilledTransaction>,
    },
    FeeFilter {
        feerate: u64,
    },
    FilterAdd {
        elements: Vec<Vec<u8>>,
    },
    FilterClear {},
    FilterLoad {
        filter: Vec<u8>,
        n_hash_funcs: u32,
        n_tweak: u32,
        n_flags: u8,
    },
    GetAddr {},
    GetBlockTxn {
        block_hash: [u8; 32],
        indexes: Vec<CompactInt>,
    },
    GetBlocks {
        protocol_version: Version,
        block_header_hashes: Vec<u256>,
        stop_hash: u256,
    },
    GetData {
        inventory: Vec<InventoryData>,
    },
    GetHeaders {
        protocol_version: Version,
        block_header_hashes: Vec<u256>,
        stop_hash: u256,
    },
    Headers {
        headers: Vec<BlockHeader>,
    },
    Inv {
        inventory: Vec<InventoryData>,
    },
    MemPool {},
    MerkleBlock {
        block_header: BlockHeader,
        transaction_count: u32,
        hashes: Vec<u256>,
        flags: Vec<u8>,
    },
    NotFound {
        inventory_data: Vec<InventoryData>,
    },
    Ping {
        nonce: Nonce,
    },
    Pong {
        nonce: Nonce,
    },
    // FIXME: write special deserialize
    Reject {
        message: String,
        code: char,
        reason: String,
        // extra_data: Option<[u8; 32]>,
    },
    SendCompact {
        announce: bool,
        version: u64,
    },
    SendHeaders {},
    Tx {
        transaction: Transaction,
    },
    Verack {},
    Version {
        protocol_version: Version,
        services: Services,
        timestamp: u64,
        receiver_services: Services,
        receiver: SocketAddr,
        transmitter_services: Services,
        transmitter_ip: SocketAddr,
        nonce: Nonce,
        user_agent: Vec<u8>,
        best_block: u32,
        relay: bool,
    },
}

// impl shared::Serializable for Message {
//     fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
//     where
//         W: std::io::Write,
//     {
//         match *self {
//             _ => self.serialize(target),
//         }
//     }
// }
