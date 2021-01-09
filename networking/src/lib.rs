//!
//! This Library provides
//! 1. a low level implementation of the Bitcoin Wire Protocol (the [`Message`] struct combined with [`BitcoinCodec`](crate::BitcoinCodec)).  (✅ Completed)
//! 1. a high level system encapsulating 'the rest of the network' as a [`tower::Service`](https://docs.rs/tower/0.4.1/tower/trait.Service.html).  (🟨 Work in Progress)
//!
//! # Overview
//! Following [Zcash Zebra](https://doc.zebra.zfnd.org/zebrad/), we intend to build a stateful Request/Response protocol on top of the existing Wire Protocol and encapsulate
//! the Bitcoin Network into a single service (or small [collection of Services](https://github.com/ZcashFoundation/zebra/blob/main/zebra-network/src/peer_set/set.rs)).
//! To obtain block and transaction information, a caller (in our case, [`warpd`](super::Warpd)) simply passes the relevant Service
//! a high-level Request object specifying the information to be fetched (i.e. Blocks, Headers, etc.), and the
//! Services automatically load balance these requests across available peers. Work in this direction in ongoing.
//! For now, users of the library can use the [`BitcoinCodec`] to easily turn any TcpStream into a stream of Bitcoin [`Message`s](crate::Message).
//!
mod command;
pub use command::Command;

mod message;
pub use message::Message;

mod types;

mod codec;
pub use codec::Codec as BitcoinCodec;

mod peer;
pub use peer::{Peer, PeerError};

mod message_header;
pub use message_header::MessageHeader;

mod server;

mod interface;
pub use interface::{NetworkRequest, NetworkResponse};

// mod messages;
// pub use messages::Addr;
// pub use messages::Block;
// pub use messages::BlockTxn;
// pub use messages::CompactBlock;
// pub use messages::FeeFilter;
// pub use messages::FilterAdd;
// pub use messages::FilterClear;
// pub use messages::FilterLoad;
// pub use messages::GetAddr;
// pub use messages::GetBlockTxn;
// pub use messages::GetBlocks;
// pub use messages::GetData;
// pub use messages::GetHeaders;
// pub use messages::Headers;
// pub use messages::Mempool;
// pub use messages::MerkleBlock;
// pub use messages::NotFound;
// pub use messages::Ping;
// pub use messages::Pong;
// pub use messages::Reject;
// pub use messages::SendCompact;
// pub use messages::SendHeaders;
// pub use messages::Tx;
// pub use messages::Verack;
// pub use messages::Version;
// pub use messages::{Inv, InventoryData, InventoryType};

// #[cfg(test)]
// mod tests {
//     use crate::header::Header;
//     use crate::Command;
//     use config::Config;
//     use shared::Bytes;
//     use std::net::{IpAddr, Ipv4Addr, SocketAddr};
//     #[test]
//     fn test_verack() {
//         let header = Header::from_body(Config::mainnet().magic(), Command::Verack, &Vec::new());
//         assert_eq!(
//             Bytes::from(header.to_bytes()).hex(),
//             "f9beb4d976657261636b000000000000000000005df6e0e2"
//         )
//     }
//     use crate::Version;

//     #[test]
//     fn test_version_serialize() {
//         use crate::payload::Payload;
//         use shared::Serializable;
//         let foreign_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8333);
//         let local_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8333);
//         let version = Version::new(
//             foreign_address,
//             01 as u64,
//             local_address,
//             0 as u32,
//             &std::sync::Arc::new(Config::mainnet()),
//         );
//         let mut target = Vec::new();
//         version.serialize(&mut target).unwrap();
//         assert_eq!(version.to_bytes().unwrap(), target);
//         // Peer::at_address(1, address, &Config::mainnet());

//         //peer_connect::outbound_connection();
//         //println!("The connection returned: {:#?}",result);
//     }

// #[test]
// fn test_getblocks() {
//     let mut message = Message::new();
//     let mut conf = Config::mainnet();
//     conf.set_protocol_version(70001 as u32);
//     //message.create_header_for_body(Command::GetBlocks,&conf);
//     message.create_getblocks_body(&vec![], true, &conf);
//     assert_eq!(
//         message.get_body().hex(),
//         "71110100000000000000000000000000000000000000000000000000000000000000000000"
//     )
// }

// use std::net::Ipv4Addr;
// #[test]
// fn test_tcp_message() {
//     let mut msg = Message::new();
//         msg.create_version_body(&Config::mainnet());
//         msg.create_header_for_body(Command::Version, &Config::mainnet());
//         println!("{:?} {:?}",msg.dump_header(),msg.dump_body());
//         println!("{:?}",msg.dump_contents());
//         println!("{:?}",msg.get_contents().get_bytes());

//     assert_eq!(
//         msg.dump_body(),
//         "721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001"
//     )
// }
// }
