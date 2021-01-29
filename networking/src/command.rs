use bytes::BytesMut;
use shared::{Deserializable, DeserializationError, Serializable};

/// A shorthand way of referring to a type of [Message](crate::Message). A `Command` is a single byte, while a [Message](crate::Message) is about 90 bytes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    Version,
    Verack,
    GetBlocks,
    GetData,
    Block,
    GetHeaders,
    Headers,
    Inv,
    MemPool,
    MerkleBlock,
    CmpctBlock,
    GetBlockTxn,
    BlockTxn,
    SendCmpct,
    NotFound,
    Tx,
    Addr,
    Alert,
    FeeFilter,
    FilterAdd,
    FilterClear,
    FilterLoad,
    GetAddr,
    Ping,
    Pong,
    Reject,
    SendHeaders,
}
impl Command {
    pub fn bytes(&self) -> &[u8; 12] {
        match self {
            Command::Version => b"version\0\0\0\0\0",
            Command::Verack => b"verack\0\0\0\0\0\0",
            Command::GetBlocks => b"getblocks\0\0\0",
            Command::GetData => b"getdata\0\0\0\0\0",
            Command::Block => b"block\0\0\0\0\0\0\0",
            Command::GetHeaders => b"getheaders\0\0",
            Command::BlockTxn => b"blocktxn\0\0\0\0",
            Command::CmpctBlock => b"cmpctblock\0\0",
            Command::Headers => b"headers\0\0\0\0\0",
            Command::Inv => b"inv\0\0\0\0\0\0\0\0\0",
            Command::MemPool => b"mempool\0\0\0\0\0",
            Command::MerkleBlock => b"merkleblock\0",
            Command::SendCmpct => b"sendcmpct\0\0\0",
            Command::GetBlockTxn => b"getblocktxn\0",
            Command::NotFound => b"notfound\0\0\0\0",
            Command::Tx => b"tx\0\0\0\0\0\0\0\0\0\0",
            Command::Addr => b"addr\0\0\0\0\0\0\0\0",
            Command::Alert => b"alert\0\0\0\0\0\0\0",
            Command::FeeFilter => b"feefilter\0\0\0",
            Command::FilterAdd => b"filteradd\0\0\0",
            Command::FilterClear => b"filterclear\0",
            Command::FilterLoad => b"filterload\0\0",
            Command::GetAddr => b"getaddr\0\0\0\0\0",
            Command::Ping => b"ping\0\0\0\0\0\0\0\0",
            Command::Pong => b"pong\0\0\0\0\0\0\0\0",
            Command::Reject => b"reject\0\0\0\0\0\0",
            Command::SendHeaders => b"sendheaders\0",
        }
    }
}

impl Serializable for Command {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        target.write_all(self.bytes())
    }
}

impl Deserializable for Command {
    fn deserialize(reader: &mut BytesMut) -> Result<Command, DeserializationError> {
        let buf = &reader.split_to(12)[..12];
        let command = match buf {
            b"version\0\0\0\0\0" => Command::Version,
            b"verack\0\0\0\0\0\0" => Command::Verack,
            b"getblocks\0\0\0" => Command::GetBlocks,
            b"getdata\0\0\0\0\0" => Command::GetData,
            b"block\0\0\0\0\0\0\0" => Command::Block,
            b"getheaders\0\0" => Command::GetHeaders,
            b"blocktxn\0\0\0\0" => Command::BlockTxn,
            b"cmpctblock\0\0" => Command::CmpctBlock,
            b"headers\0\0\0\0\0" => Command::Headers,
            b"inv\0\0\0\0\0\0\0\0\0" => Command::Inv,
            b"mempool\0\0\0\0\0" => Command::MemPool,
            b"merkleblock\0" => Command::MerkleBlock,
            b"sendcmpct\0\0\0" => Command::SendCmpct,
            b"getblocktxn\0" => Command::GetBlockTxn,
            b"notfound\0\0\0\0" => Command::NotFound,
            b"tx\0\0\0\0\0\0\0\0\0\0" => Command::Tx,
            b"addr\0\0\0\0\0\0\0\0" => Command::Addr,
            b"alert\0\0\0\0\0\0\0" => Command::Alert,
            b"feefilter\0\0\0" => Command::FeeFilter,
            b"filteradd\0\0\0" => Command::FilterAdd,
            b"filterclear\0" => Command::FilterClear,
            b"filterload\0\0" => Command::FilterLoad,
            b"getaddr\0\0\0\0\0" => Command::GetAddr,
            b"ping\0\0\0\0\0\0\0\0" => Command::Ping,
            b"pong\0\0\0\0\0\0\0\0" => Command::Pong,
            b"reject\0\0\0\0\0\0" => Command::Reject,
            b"sendheaders\0" => Command::SendHeaders,
            _ => return Err(DeserializationError::parse(buf, "Command")),
        };
        Ok(command)
    }
}
