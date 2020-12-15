use crate::command::Command;
use crate::header::Header;
use crate::messages::{InventoryData, Verack, Version};
use crate::payload::Payload;
use config::Config;
use log::{debug, info, trace};
use shared::{u256, DeserializationError};
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::Duration;
use std::{fmt, sync::Arc};
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
    Malicious(String),
}
impl From<DeserializationError> for PeerError {
    fn from(kind: DeserializationError) -> PeerError {
        PeerError::Deserialzation(kind)
    }
}
impl From<tokio::time::error::Elapsed> for PeerError {
    fn from(kind: tokio::time::error::Elapsed) -> PeerError {
        PeerError::Timeout(kind.to_string())
    }
}
impl From<std::io::Error> for PeerError {
    fn from(kind: std::io::Error) -> PeerError {
        PeerError::Io(kind)
    }
}

impl fmt::Display for PeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            PeerError::Timeout(cause) => cause.fmt(f),
            PeerError::Deserialzation(cause) => cause.fmt(f),
            PeerError::Io(cause) => cause.fmt(f),
            PeerError::Message(cause) => cause.fmt(f),
            PeerError::Malicious(cause) => cause.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct Peer {
    peer_id: usize,
    ip_address: SocketAddr,
    nonce: u64,
    daemon_address: SocketAddr,
    daemon_protocol_version: u32,
    services: u64,
    connection: tokio::io::BufStream<TcpStream>,
    config: Arc<Config>,
}

impl Peer {
    pub async fn at_address(id: usize, address: SocketAddr, config: Arc<Config>) -> Result<Peer> {
        info!("Peer {}: Opening connection to {:?}...", id, address.ip());
        let connection = timeout(Duration::from_secs(5), TcpStream::connect(address)).await??;
        info!("Peer {}: Connected", id);
        Ok(Peer {
            peer_id: id,
            ip_address: address,
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            services: 0,
            connection: tokio::io::BufStream::new(connection),
            config,
        })
    }
    pub async fn from_connection(id: usize, connection: TcpStream, config: Arc<Config>) -> Peer {
        info!("Receiving from {:?}", connection.peer_addr());
        Peer {
            peer_id: id,
            services: 0,
            ip_address: connection.peer_addr().unwrap(),
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            connection: tokio::io::BufStream::new(connection),
            config,
        }
    }
    pub async fn send<T>(&mut self, command: Command, payload: T) -> Result<()>
    where
        T: Payload,
    {
        trace!("Peer {}: Sending {:?}", self.peer_id, command);
        let raw_msg = payload.to_bytes()?;
        let raw_header = Header::from_body(self.config.magic(), command, &raw_msg).to_bytes();
        self.connection.write_all(&raw_header).await?;
        self.connection.write_all(&raw_msg).await?;
        self.connection.flush().await?;
        Ok(())
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

    pub async fn receive(
        &mut self,
        timeout_duration: Option<Duration>,
    ) -> Result<(Header, Vec<u8>)> {
        let mut header_buf = [0u8; 24];
        let ttl = timeout_duration.unwrap_or(Duration::from_secs(60 * 89)); // Timeout after at  89 minutes by default.
        timeout(ttl, self.connection.read_exact(&mut header_buf)).await??;
        let header = Header::deserialize(&mut Cursor::new(header_buf), self.config.magic())?;
        trace!("Peer {}: receiving {:?}", self.peer_id, header);
        if header.get_payload_size() > self.config.get_max_msg_size() {
            return Err(PeerError::Malicious(format!(
                "Peer sent message of length {} (max allowed {}).",
                header.get_payload_size(),
                self.config.get_max_msg_size()
            )));
        }
        let mut payload = Vec::with_capacity(header.get_payload_size());
        payload.resize(header.get_payload_size(), 0);
        timeout(
            Duration::from_secs(1),
            self.connection.read_exact(&mut payload),
        )
        .await??;
        if log::log_enabled!(log::Level::Debug) {
            debug!(
                "Body: {:#?}",
                header
                    .get_command()
                    .deserialize_body(&mut std::io::Cursor::new(&payload))
            );
        }
        Ok((header, payload))
    }
    pub async fn receive_expected(
        &mut self,
        expected: Command,
        timeout_duration: Option<Duration>,
    ) -> Result<(Header, Vec<u8>)> {
        trace!("Peer {}: waiting for {:?}", self.peer_id, expected);
        let (header, body) = self.receive(timeout_duration).await?;
        if header.get_command() != expected {
            return Err(PeerError::Message(format!(
                "Expected {:?} but got {:?}",
                expected,
                header.get_command()
            )));
        }
        Ok((header, body))
    }

    pub async fn perform_handshake(&mut self, best_block: Option<u32>) -> Result<()> {
        self.send(Command::Version, self.create_version_msg(best_block))
            .await?;
        self.receive_expected(Command::Version, Some(Duration::from_secs(60)))
            .await?;
        self.receive_expected(Command::Verack, Some(Duration::from_secs(60)))
            .await?;
        self.send(Command::Verack, Verack {}).await?;
        info!("Peer {}: HandShake complete", self.peer_id);
        Ok(())
    }
    pub async fn accept_handshake(&mut self, best_block: Option<u32>) -> Result<()> {
        self.receive_expected(Command::Version, Some(Duration::from_secs(60)))
            .await?;
        self.send(Command::Version, self.create_version_msg(best_block))
            .await?;
        self.send(Command::Verack, Verack {}).await?;
        self.receive_expected(Command::Verack, None).await?;
        info!("Peer {}: HandShake complete", self.peer_id);
        Ok(())
    }
    pub fn create_version_msg(&self, best_block: Option<u32>) -> Version {
        Version::new(
            self.ip_address.clone(),
            self.services,
            self.daemon_address,
            best_block.unwrap_or(0),
            &self.config,
        )
    }
}
