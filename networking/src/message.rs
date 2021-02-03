// use crate::command::Command;
// use crate::header::Header;
// use shared::Bytes;
// use tokio::net::TcpStream;

use crate::types::{Nonce, PrefilledTransaction, ProtocolVersion, Services};
use crate::Command;
use serde_derive::Serializable;
use shared::BlockHeader;
use shared::EncapsulatedAddr;
use shared::InventoryData;
use shared::Transaction;
use std::net::SocketAddr;

mod block_txn;
pub use block_txn::BlockTxn;

mod compact_block;
pub use compact_block::CompactBlock;

mod filter_load;
pub use filter_load::FilterLoad;

mod get_block_txn;
pub use get_block_txn::GetBlockTxn;

mod get_blocks;
pub use get_blocks::GetBlocks;

mod get_headers;
pub use get_headers::GetHeaders;

mod merkle_block;
pub use merkle_block::MerkleBlock;

mod reject;
pub use reject::Reject;

mod send_compact;
pub use send_compact::SendCompact;

mod version;
pub use version::Version;

pub trait Payload {
    fn serialized_size(&self) -> usize;
    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error>;
}

/// An enumeration of all [Bitcoin Wire Protocol](https://developer.bitcoin.org/reference/p2p_networking.html) messages, (i.e. GetHeaders, Version, Verack).
///
/// Messages are actual messages, as opposed to [`Command`s](crate::Command) which are a shorthand way of referring to a type of Message.
/// A Message takes about 90 bytes of data on the stack, while a Command is a single byte.
#[derive(Debug, Serializable)]
pub enum Message {
    Addr(Vec<EncapsulatedAddr>),
    BlockTxn(BlockTxn),
    Block(shared::Block),
    CompactBlock(CompactBlock),
    FeeFilter(u64),
    FilterAdd(Vec<Vec<u8>>),
    FilterClear,
    FilterLoad(FilterLoad),
    GetAddr,
    GetBlockTxn(GetBlockTxn),
    GetBlocks(GetBlocks),
    GetData(Vec<InventoryData>),
    GetHeaders(GetHeaders),
    Headers(Vec<BlockHeader>),
    Inv(Vec<InventoryData>),
    MemPool,
    MerkleBlock(MerkleBlock),
    NotFound(Vec<InventoryData>),
    Ping(Nonce),
    Pong(Nonce),
    Reject(Reject),
    SendCompact(SendCompact),
    SendHeaders,
    Tx(Transaction),
    Verack,
    Version(Version),
}

impl Message {
    pub fn version(
        peer_ip: SocketAddr,
        peer_services: u64,
        warpd_ip: SocketAddr,
        best_block: u32,
        config: &config::Config,
    ) -> Message {
        Message::Version(Version::new(
            peer_ip,
            peer_services,
            warpd_ip,
            best_block,
            config,
        ))
    }

    pub fn command(&self) -> Command {
        match self {
            Message::Addr { .. } => Command::Addr,
            Message::BlockTxn { .. } => Command::BlockTxn,
            Message::Block { .. } => Command::Block,
            Message::CompactBlock { .. } => Command::CmpctBlock,
            Message::FeeFilter { .. } => Command::FeeFilter,
            Message::FilterAdd { .. } => Command::FilterAdd,
            Message::FilterClear {} => Command::FilterClear,
            Message::FilterLoad { .. } => Command::FilterLoad,
            Message::GetAddr {} => Command::GetAddr,
            Message::GetBlockTxn { .. } => Command::GetBlockTxn,
            Message::GetBlocks { .. } => Command::GetBlocks,
            Message::GetData { .. } => Command::GetData,
            Message::GetHeaders { .. } => Command::GetHeaders,
            Message::Headers { .. } => Command::Headers,
            Message::Inv { .. } => Command::Inv,
            Message::MemPool {} => Command::MemPool,
            Message::MerkleBlock { .. } => Command::MerkleBlock,
            Message::NotFound { .. } => Command::MemPool,
            Message::Ping { .. } => Command::Ping,
            Message::Pong { .. } => Command::Pong,
            Message::Reject { .. } => Command::Reject,
            Message::SendCompact { .. } => Command::SendCmpct,
            Message::SendHeaders {} => Command::SendHeaders,
            Message::Tx { .. } => Command::Tx,
            Message::Verack {} => Command::Verack,
            Message::Version { .. } => Command::Version,
        }
    }
}
