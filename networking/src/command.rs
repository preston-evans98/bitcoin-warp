// use crate::payload::Payload;
use shared::{Deserializable, DeserializationError, Serializable};
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
    pub fn print_body<R>(&self, msg: &mut R) -> Result<(), DeserializationError>
    where
        R: std::io::Read,
    {
        match self {
            Command::Version => println!("{:#?}", crate::Version::deserialize(msg)?),
            Command::Verack => println!("{:#?}", crate::Verack::deserialize(msg)?),
            Command::GetBlocks => println!("{:#?}", crate::GetBlocks::deserialize(msg)?),
            Command::GetData => println!("{:#?}", crate::GetData::deserialize(msg)?),
            Command::Block => println!("{:#?}", crate::Block::deserialize(msg)?),
            Command::GetHeaders => println!("{:#?}", crate::GetHeaders::deserialize(msg)?),
            Command::BlockTxn => println!("{:#?}", crate::BlockTxn::deserialize(msg)?),
            Command::CmpctBlock => println!("{:#?}", crate::CompactBlock::deserialize(msg)?),
            Command::Headers => println!("{:#?}", crate::Headers::deserialize(msg)?),
            Command::Inv => println!("{:#?}", crate::Inv::deserialize(msg)?),
            Command::MemPool => println!("{:#?}", crate::Mempool::deserialize(msg)?),
            Command::MerkleBlock => println!("{:#?}", crate::MerkleBlock::deserialize(msg)?),
            Command::SendCmpct => println!("{:#?}", crate::SendCompact::deserialize(msg)?),
            Command::GetBlockTxn => println!("{:#?}", crate::GetBlockTxn::deserialize(msg)?),
            Command::NotFound => println!("{:#?}", crate::NotFound::deserialize(msg)?),
            Command::Tx => println!("{:#?}", crate::Tx::deserialize(msg)?),
            Command::Addr => println!("{:#?}", crate::Addr::deserialize(msg)?),
            // Command::Alert => println!("{:#?}", crate::Alert::deserialize(msg)?),
            Command::FeeFilter => println!("{:#?}", crate::FeeFilter::deserialize(msg)?),
            Command::FilterAdd => println!("{:#?}", crate::FilterAdd::deserialize(msg)?),
            Command::FilterClear => println!("{:#?}", crate::FilterClear::deserialize(msg)?),
            Command::FilterLoad => println!("{:#?}", crate::FilterLoad::deserialize(msg)?),
            Command::GetAddr => println!("{:#?}", crate::GetAddr::deserialize(msg)?),
            Command::Ping => println!("{:#?}", crate::Ping::deserialize(msg)?),
            Command::Pong => println!("{:#?}", crate::Pong::deserialize(msg)?),
            // Command::Reject => println!("{:#?}", crate::Reject::deserialize(msg)?),
            Command::SendHeaders => println!("{:#?}", crate::SendHeaders::deserialize(msg)?),
            _ => panic!(),
        }
        Ok(())
    }

    // pub fn full_type(&self) -> T {
    //     match self {
    //         Command::Version => crate::Version,
    //         // Command::Verack =>crate::Verack,
    //         // Command::GetBlocks => crate::GetBlocks,
    //         // Command::GetData => crate::GetData,
    //         // Command::Block => crate::Block,
    //         // Command::GetHeaders => crate::GetHeaders,
    //         // Command::BlockTxn => crate::BlockTxn,
    //         // Command::CmpctBlock => crate::CmpctBlock,
    //         // Command::Headers => crate::Headers,
    //         // Command::Inv => crate::Inv,
    //         // Command::MemPool => crate::MemPool,
    //         // Command::MerkleBlock => crate::MerkleBlock,
    //         // Command::SendCmpct => crate::SendCmpct,
    //         // Command::GetBlockTxn => crate::GetBlockTxn,
    //         // Command::NotFound => crate::NotFound,
    //         // Command::Tx => crate::Tx:,
    //         // Command::Addr => crate::Addr,
    //         // Command::Alert => crate::Alert,
    //         // Command::FeeFilter => crate::FeeFilter,
    //         // Command::FilterAdd => crate::FilterAdd,
    //         // Command::FilterClear => crate::FilterClear,
    //         // Command::FilterLoad => crate::FilterLoad,
    //         // Command::GetAddr => crate::GetAddr,
    //         // Command::Ping => crate::Ping,
    //         // Command::Pong => crate::Pong,
    //         // Command::Reject => crate::Reject,
    //         // Command::SendHeaders => crate::SendHeaders,
    //         _ => panic!(),
    //     }
    // }
}

// macro_rules! command_bytes {
//     // () => {
//     //     pub fn bytes(&self) -> &[u8; 12] {
//     //         match self {
//     //             Command::Version => b"version\0\0\0\0\0",
//     //             Command::Verack => b"verack\0\0\0\0\0\0",
//     //             Command::GetBlocks => b"getblocks\0\0\0",
//     //             Command::GetData => b"getdata\0\0\0\0\0",
//     //             Command::Block => b"block\0\0\0\0\0\0\0",
//     //             Command::GetHeaders => b"getheaders\0\0",
//     //         }
//     //     }
//     // };

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
