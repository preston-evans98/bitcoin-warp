use crate::command::Command;
use crate::header::Header;
use crate::message::Message;
use crate::messages::{GetBlocks, GetData, InventoryData, Version, Block};
use config::Config;
use shared::{u256, DeserializationError};
use std::fmt;
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

type Result<T> = std::result::Result<T, PeerError>;

#[derive(Debug)]
pub enum PeerError {
    Timeout(String),
    Io(std::io::Error),
    Deserialzation(DeserializationError),
    Message(String),
}
impl From<DeserializationError> for PeerError {
    fn from(kind: DeserializationError) -> PeerError {
        PeerError::Deserialzation(kind)
    }
}
impl From<tokio::time::Elapsed> for PeerError {
    fn from(kind: tokio::time::Elapsed) -> PeerError {
        PeerError::Timeout(kind.to_string())
    }
}
impl From<std::io::Error> for PeerError {
    fn from(kind: std::io::Error) -> PeerError {
        PeerError::Io(kind)
    }
}
// impl From<tokio::time::Elapsed> for PeerError {
//     fn from(kind: tokio::time::Elapsed) -> PeerError {
//         PeerError::Timeout(kind.to_string())
//     }
// }

impl fmt::Display for PeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PeerError::Timeout(cause) => cause.fmt(f),
            PeerError::Deserialzation(cause) => cause.fmt(f),
            PeerError::Io(cause) => cause.fmt(f),
            PeerError::Message(cause) => cause.fmt(f),
        }
    }
}

pub struct Peer<'a> {
    peer_id: usize,
    ip_address: SocketAddr,
    nonce: u64,
    daemon_address: SocketAddr,
    daemon_protocol_version: u32,
    services: u64,
    connection: TcpStream,
    config: &'a Config,
}

impl<'a> Peer<'a> {
    pub async fn at_address(
        id: usize,
        address: SocketAddr,
        config: &'a Config,
    ) -> Result<Peer<'a>> {
        let connection = timeout(Duration::from_secs(5), TcpStream::connect(address)).await??;
        Ok(Peer {
            peer_id: id,
            ip_address: address,
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            services: 0,
            connection: connection,
            config,
        })
    }
    pub async fn from_connection(id: usize, connection: TcpStream, config: &'a Config) -> Peer<'a> {
        Peer {
            peer_id: id,
            services: 0,
            ip_address: connection.peer_addr().unwrap(),
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            connection: connection,
            config,
        }
    }
    pub async fn send(&mut self, command: Command) -> Result<()> {
        let mut msg = Message::new();
        match command {
            Command::Version => {
                let message = Version::new(
                    self.ip_address,
                    self.services,
                    self.daemon_address,
                    self.get_best_block(),
                    &Config::mainnet(),
                );
            }
            Command::Verack => {}
            Command::GetBlocks => {
                let message: GetBlocks =
                    GetBlocks::new(self.get_block_hashes(), true, &Config::mainnet());
            }
            Command::GetData => {
                let message: GetData = GetData::new(self.get_inventory_data(), &Config::mainnet());
            }
            Command::Block => {
                // let message: Block = Block::new();
            }
        }
        msg.create_header_for_body(command, self.config.magic());
        self.connection.write(msg.get_header().get_bytes()).await?;
        self.connection.write(msg.get_body().get_bytes()).await?;
        Ok(())
    }

    pub async fn receive(&mut self, timeout_duration: Option<Duration>) -> Result<Command> {
        let mut buf = [0u8; 24];
        if let Some(duration) = timeout_duration {
            timeout(duration, self.connection.read_exact(&mut buf)).await??;
        } else {
            self.connection.read_exact(&mut buf).await?;
        }
        let header = Header::deserialize(&mut Cursor::new(buf), self.config.magic())?;
        Ok(header.get_command())
    }

    pub fn get_ip_address(&self) -> SocketAddr {
        self.ip_address
    }
    pub fn get_daemon_address(&self) -> SocketAddr {
        self.daemon_address
    }

    pub fn get_best_block(&self) -> u32 {
        1
    }
    pub fn get_block_hashes(&self) -> Vec<u256> {
        Vec::new()
        //One or more block header hashes (32 bytes each) in internal byte order.
        //Hashes should be provided in reverse order of block height,
        //so highest-height hashes are listed first and lowest-height hashes are listed last.
        //should get from the database the block headers needed and hash them here and put them in a vector
    }
    pub fn get_inventory_data(&self) -> Vec<InventoryData> {
        //needs to get the actual data that we want to request from peer and put it in an InventoryData object
        Vec::new()
    }
    pub async fn perform_handshake(&mut self) -> Result<()> {
        self.send(Command::Version).await?; //sending version message
        let version_response = self.receive(Some(Duration::from_secs(60))).await?;
        match version_response {
            Command::Version => {
                self.send(Command::Verack).await?;
            }
            _ => {
                return Err(PeerError::Message(format!(
                    "Expected Version but got {:?}",
                    version_response
                )))
            }
        }
        let verack_response = self.receive(Some(Duration::from_secs(60))).await?;
        match verack_response {
            Command::Verack => return Ok(()),
            _ => {
                return Err(PeerError::Message(format!(
                    "Expected Verack message but got {:?}",
                    verack_response
                )))
            }
        }
    }
    pub async fn accept_handshake(&mut self) -> Result<()> {
        let version_response = self.receive(Some(Duration::from_secs(60))).await?;
        match version_response {
            Command::Version => {
                self.send(Command::Version).await?; //sending version message
            }
            _ => {
                return Err(PeerError::Message(format!(
                    "Expected Version message but got {:?}",
                    version_response
                )))
            }
        }
        self.send(Command::Verack).await?;
        let verack_response = self.receive(Some(Duration::from_secs(60))).await?;
        match verack_response {
            Command::Verack => return Ok(()),
            _ => {
                return Err(PeerError::Message(format!(
                    "Expected Verack message but got {:?}",
                    verack_response
                )))
            }
        }
    }
}
