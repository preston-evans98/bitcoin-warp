use crate::command::Command;
use crate::header::Header;
use crate::message::Message;
use crate::payload::Payload;
use config::Config;
use shared::DeserializationError;
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
                Message::from(&self.version_payload(), &Config::mainnet());
            }
            Command::Verack => {}
            Command::GetBlocks => {
                // msg.create_getblocks_body(block_hashes: &Vec<Bytes>, request_inventory: false, config: &Config)
            }
            Command::GetData => {}
        }
        msg.create_header_for_body(command, self.config.magic());
        self.connection.write(msg.get_header().get_bytes()).await?;
        self.connection.write(msg.get_body().get_bytes()).await?;
        Ok(())
    }

    pub async fn receive(&mut self, timeoutDuration: Option<Duration>) -> Result<Command> {
        let mut buf = [0u8; 24];
        if let Some(duration) = timeoutDuration {
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
    pub fn version_payload(&self) -> Payload {
        Payload::VersionPayload {
            peer_ip: &self.ip_address,
            peer_services: self.services,
            daemon_ip: &self.daemon_address,
            best_block: self.get_best_block(),
        }
    }

    pub fn get_best_block(&self) -> u32 {
        1
    }
}
