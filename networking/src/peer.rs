use crate::command::Command;
use crate::message::Message;
use config::Config;
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
#[derive(Debug)]
pub struct PeerError {
    message: String,
}
impl PeerError {
    pub fn new(message: String) -> PeerError {
        PeerError { message }
    }
}
impl fmt::Display for PeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "A peer inccured the following error:{}", self.message);
        Ok(())
    }
}

pub struct Peer {
    peer_id: usize,
    services: u64,
    version: u32,
    ip_address: SocketAddr,
    nonce: u64,
    daemon_address: SocketAddr,
    magic: u32,
    daemon_protocol_version: u32,
    connection: TcpStream,
}

impl Peer {
    pub async fn at_address(
        id: usize,
        address: SocketAddr,
        config: &Config,
    ) -> Result<Peer, PeerError> {
        match timeout(Duration::from_secs(5), TcpStream::connect(address)).await {
            Ok(Ok(connection)) => Ok(Peer {
                magic: config.magic(),
                peer_id: id,
                services: config.get_services(),
                version: 0,
                ip_address: address,
                nonce: 0,
                daemon_address: connection.local_addr().unwrap(),
                daemon_protocol_version: config.get_protocol_version(),
                connection: connection,
            }),
            Ok(Err(e)) => Err(PeerError::new(format!(
                "Error connecting to {:?}: {}",
                address, e
            ))),
            Err(_) => Err(PeerError::new(format!(
                "Error connecting to {:?}: Timeout",
                address
            ))),
        }
    }
    pub async fn from_connection(id: usize, connection: TcpStream, config: &Config) -> Peer {
        Peer {
            magic: config.magic(),
            peer_id: id,
            services: config.get_services(),
            version: 0,
            ip_address: connection.peer_addr().unwrap(),
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            connection: connection,
        }
    }
    pub async fn send(&mut self, command: Command) {
        let mut msg = Message::new();
        match command {
            Command::Version => {
                Message::from(Command::Version, &self, &Config::mainnet());
            }
            Command::Verack => {}
            Command::GetBlocks => {
                // msg.create_getblocks_body(block_hashes: &Vec<Bytes>, request_inventory: false, config: &Config)
            }
        }
        msg.create_header_for_body(command, self.magic);
        self.connection.write(msg.get_header().get_bytes());
        self.connection.write(msg.get_body().get_bytes()).await;
    }

    pub async fn receive(&self) -> Command {
        //deserialization call here and return the message
        Command::Verack
    }

    pub fn get_ip_address(&self) -> SocketAddr {
        self.ip_address
    }
    pub fn get_daemon_address(&self) -> SocketAddr {
        self.daemon_address
    }
}
