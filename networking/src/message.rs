// use crate::command::Command;
// use crate::header::Header;
// use shared::Bytes;
// use tokio::net::TcpStream;

use crate::types::{Nonce, PrefilledTransaction, ProtocolVersion, Services};
use serde_derive::Serializable;
use shared::BlockHeader;
use shared::EncapsulatedAddr;
use shared::InventoryData;
use shared::Transaction;
use shared::{u256, CompactInt};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

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
        protocol_version: ProtocolVersion,
        block_header_hashes: Vec<u256>,
        stop_hash: u256,
    },
    GetData {
        inventory: Vec<InventoryData>,
    },
    GetHeaders {
        protocol_version: ProtocolVersion,
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
    Reject {
        message: String,
        code: u8,
        reason: String,
        extra_data: Option<[u8; 32]>,
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
        protocol_version: ProtocolVersion,
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

impl Message {
    pub fn version(
        peer_ip: SocketAddr,
        peer_services: u64,
        daemon_ip: SocketAddr,
        best_block: u32,
        config: &config::Config,
    ) -> Message {
        Message::Version {
            protocol_version: config.get_protocol_version(),
            services: config.get_services(),
            timestamp: secs_since_the_epoch(),
            receiver_services: peer_services,
            receiver: peer_ip,
            transmitter_services: config.get_services(),
            transmitter_ip: daemon_ip,
            nonce: 0 as u64,
            user_agent: Vec::new(),
            best_block: best_block,
            relay: true,
        }
    }
}

fn secs_since_the_epoch() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs_f64() as u64
}
