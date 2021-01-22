// //! ![BitcoinWarp Logo](/Users/prestonevans/Downloads/BitcoinWarpLogoMock.png)

mod shell;
use config::Config;
use networking::{Peer, PeerError};
pub use shell::shell::run_shell;
use std::net::SocketAddr;
use std::sync::Arc;

/// The Bitcoin Warp Daemon
#[derive(Debug)]
pub struct Warpd {
    pub config: Arc<Config>,
    conn_man: ConnectionManager,
}

/// An initial pass at a connection manager. Soon to be deprecated. Its replacement will live in the `networking` crate.
#[derive(Debug)]
pub struct ConnectionManager {
    peers: Vec<Peer>,
}
impl ConnectionManager {
    pub fn new() -> ConnectionManager {
        ConnectionManager { peers: Vec::new() }
    }
    pub async fn add(&mut self, addr: SocketAddr, config: &Arc<Config>) -> Result<(), PeerError> {
        let peer = Peer::at_address(self.num_peers(), addr, config.clone()).await?;
        self.peers.push(peer);
        Ok(())
    }
    pub async fn accept(&mut self, port: &str, config: &Arc<Config>) -> Result<(), PeerError> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect(&format!("Could not create listener on {}", addr));
        let (connection, _) = listener.accept().await?;
        let peer = Peer::from_connection(self.num_peers(), connection, config.clone()).await;
        self.peers.push(peer);
        Ok(())
    }

    pub fn num_peers(&self) -> usize {
        self.peers.len()
    }
}
// #[derive(Debug)]
// struct Peer {
//     ip_address: std::net::IpAddr,
//     services: u64,
// }

impl Warpd {
    pub fn new() -> Warpd {
        Warpd {
            config: Arc::new(Config::mainnet()),
            conn_man: ConnectionManager::new(),
        }
    }

    pub async fn add_peer(&mut self, addr: SocketAddr) -> Result<(), PeerError> {
        if self.conn_man.num_peers() >= self.config.max_peers() {
            unimplemented!()
        }
        self.conn_man.add(addr, &self.config).await
    }
    pub async fn accept_peer(&mut self, port: &str) -> Result<(), PeerError> {
        if self.conn_man.num_peers() >= self.config.max_peers() {
            unimplemented!()
        }
        self.conn_man.accept(port, &self.config).await
    }
}
