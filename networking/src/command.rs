use shared::{Deserializable, DeserializationError, Serializable};
use DeserializationError::Parse;

#[derive(Debug, Clone, PartialEq)]
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
    pub fn deserialize_body<R>(
        &self,
        msg: &mut R,
    ) -> Result<Box<dyn crate::payload::Payload>, DeserializationError>
    where
        R: std::io::Read,
    {
        match self {
            Command::Version => Ok(Box::new(crate::Version::deserialize(msg)?)),
            Command::Verack => Ok(Box::new(crate::Verack::deserialize(msg)?)),
            Command::GetBlocks => Ok(Box::new(crate::GetBlocks::deserialize(msg)?)),
            Command::GetData => Ok(Box::new(crate::GetData::deserialize(msg)?)),
            Command::Block => Ok(Box::new(crate::Block::deserialize(msg)?)),
            Command::GetHeaders => Ok(Box::new(crate::GetHeaders::deserialize(msg)?)),
            Command::BlockTxn => Ok(Box::new(crate::BlockTxn::deserialize(msg)?)),
            Command::CmpctBlock => Ok(Box::new(crate::CompactBlock::deserialize(msg)?)),
            Command::Headers => Ok(Box::new(crate::Headers::deserialize(msg)?)),
            Command::Inv => Ok(Box::new(crate::Inv::deserialize(msg)?)),
            Command::MemPool => Ok(Box::new(crate::Mempool::deserialize(msg)?)),
            Command::MerkleBlock => Ok(Box::new(crate::MerkleBlock::deserialize(msg)?)),
            Command::SendCmpct => Ok(Box::new(crate::SendCompact::deserialize(msg)?)),
            Command::GetBlockTxn => Ok(Box::new(crate::GetBlockTxn::deserialize(msg)?)),
            Command::NotFound => Ok(Box::new(crate::NotFound::deserialize(msg)?)),
            Command::Tx => Ok(Box::new(crate::Tx::deserialize(msg)?)),
            Command::Addr => Ok(Box::new(crate::Addr::deserialize(msg)?)),
            // Command::Alert => println!("{:#?}", crate::Alert::deserialize(msg)?),
            Command::FeeFilter => Ok(Box::new(crate::FeeFilter::deserialize(msg)?)),
            Command::FilterAdd => Ok(Box::new(crate::FilterAdd::deserialize(msg)?)),
            Command::FilterClear => Ok(Box::new(crate::FilterClear::deserialize(msg)?)),
            Command::FilterLoad => Ok(Box::new(crate::FilterLoad::deserialize(msg)?)),
            Command::GetAddr => Ok(Box::new(crate::GetAddr::deserialize(msg)?)),
            Command::Ping => Ok(Box::new(crate::Ping::deserialize(msg)?)),
            Command::Pong => Ok(Box::new(crate::Pong::deserialize(msg)?)),
            // Command::Reject => Ok(Box::new(crate::Reject::deserialize(msg)?)),
            Command::SendHeaders => Ok(Box::new(crate::SendHeaders::deserialize(msg)?)),
            _ => Err(Parse(format!(
                "Command::deserialize_body not implemented for {:?}",
                self
            ))),
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
    fn deserialize<T>(reader: &mut T) -> Result<Command, DeserializationError>
    where
        T: std::io::Read,
    {
        let mut buf = [0u8; 12];
        reader.read_exact(&mut buf)?;
        let command = match &buf {
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
            _ => return Err(DeserializationError::parse(&buf, "Command")),
        };
        Ok(command)
    }
}
