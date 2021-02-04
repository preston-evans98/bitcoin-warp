use crate::{command::Command, BitcoinCodec, Message, NetworkRequest, NetworkResponse};
use config::Config;
use futures::{prelude::*, FutureExt};
use shared::DeserializationError;
use std::time::Duration;
use std::{fmt, sync::Arc};
use std::{net::SocketAddr, pin::Pin};
use tower::Service;
// use tower::Service;
// use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::Framed;
use tracing::{info, span::Span, trace, trace_span};

type Result<T> = std::result::Result<T, PeerError>;

/// An enumeration of errors that a Peer connection can encounter, including malicious behavior.
#[derive(Debug)]
pub enum PeerError {
    Timeout(String),
    Io(std::io::Error),
    Deserialzation(DeserializationError),
    Message(String),
    Malicious(String),
    Unexpected(String),
    ConnectionClosed,
    MessageRejected(String),
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
            PeerError::ConnectionClosed => Ok(()),
            PeerError::Unexpected(cause) => cause.fmt(f),
            PeerError::MessageRejected(cause) => cause.fmt(f),
        }
    }
}

/// A [`tower::Service`](https://docs.rs/tower/0.4.1/tower/trait.Service.html) representing an individual peer.
///
/// Relies on a backing `NodeDataStore` [`Service`](https://docs.rs/tower/0.4.1/tower/trait.Service.html) to obtain block and transaction information.
/// If the `NodeDataStore` polls unready, this information will be propagated to the `ConnectionManager` struct, which may drop the the corresponding peer
/// to relieve backpressure.
#[derive(Debug)]
pub struct Peer {
    peer_id: usize,
    ip_address: SocketAddr,
    nonce: u64,
    daemon_address: SocketAddr,
    daemon_protocol_version: u32,
    services: u64,
    connection: Framed<TcpStream, BitcoinCodec>,
    config: Config,
    span: Span,
}

impl Peer {
    pub fn at_address(
        address: SocketAddr,
        config: Config,
    ) -> Pin<Box<dyn Future<Output = Result<Peer>> + Send>> {
        async move {
            info!("Peer: Opening connection to {:?}...", address.ip());
            let connection = timeout(Duration::from_secs(5), TcpStream::connect(address)).await??;
            let codec = BitcoinCodec::new(config.magic());
            info!("Peer: Connected");
            Ok(Peer {
                peer_id: 0,
                ip_address: address,
                nonce: 0,
                daemon_address: connection
                    .local_addr()
                    .expect("Connection should have a local address"),
                daemon_protocol_version: config.get_protocol_version(),
                services: 0,
                connection: Framed::new(connection, codec),
                config,
                span: trace_span!("peer"),
            })
        }
        .boxed()
    }
    pub async fn from_connection(id: usize, connection: TcpStream, config: Config) -> Peer {
        let codec = BitcoinCodec::new(config.magic());
        info!("Receiving from {:?}", connection.peer_addr());
        Peer {
            peer_id: id,
            services: 0,
            ip_address: connection.peer_addr().unwrap(),
            nonce: 0,
            daemon_address: connection.local_addr().unwrap(),
            daemon_protocol_version: config.get_protocol_version(),
            connection: Framed::new(connection, codec),
            config,
            span: trace_span!("peer", id = id),
        }
    }
    pub async fn send(&mut self, msg: Message) -> Result<()> {
        trace!("Peer {}: Sending {:?}", self.peer_id, &msg);
        self.connection.send(msg).await?;
        Ok(())
    }

    pub fn get_ip_address(&self) -> SocketAddr {
        self.ip_address
    }
    pub fn get_daemon_address(&self) -> SocketAddr {
        self.daemon_address
    }

    pub fn get_best_block(&self) -> u32 {
        0
    }
    pub async fn receive(&mut self, _timeout_duration: Option<Duration>) -> Result<Message> {
        let result = match self.connection.next().await {
            None => Err(PeerError::ConnectionClosed),
            Some(contents) => Ok(contents?),
        };
        trace!("Peer {}: Received {:?}", self.peer_id, &result);
        result
    }
    pub async fn receive_expected(
        &mut self,
        expected: Command,
        timeout_duration: Option<Duration>,
    ) -> Result<Message> {
        trace!("Peer {}: waiting for {:?}", self.peer_id, expected);
        let msg = self.receive(timeout_duration).await?;

        if msg.command() != expected {
            return Err(PeerError::Message(format!(
                "Expected {:?} but got {:?}",
                expected,
                msg.command()
            )));
        }
        Ok(msg)
    }

    pub async fn perform_handshake(&mut self, best_block: Option<u32>) -> Result<()> {
        self.send(self.create_version_msg(best_block)).await?;
        self.receive_expected(Command::Version, Some(Duration::from_secs(60)))
            .await?;
        self.receive_expected(Command::Verack, Some(Duration::from_secs(60)))
            .await?;
        self.send(Message::Verack {}).await?;
        info!("Peer {}: HandShake complete", self.peer_id);
        Ok(())
    }
    pub async fn accept_handshake(&mut self, best_block: Option<u32>) -> Result<()> {
        self.receive_expected(Command::Version, Some(Duration::from_secs(60)))
            .await?;
        self.send(self.create_version_msg(best_block)).await?;
        self.send(Message::Verack {}).await?;
        self.receive_expected(Command::Verack, None).await?;
        info!("Peer {}: HandShake complete", self.peer_id);
        Ok(())
    }
    pub fn create_version_msg(&self, best_block: Option<u32>) -> Message {
        Message::version(
            self.ip_address.clone(),
            self.services,
            self.daemon_address,
            best_block.unwrap_or(0),
            &self.config,
        )
    }
}

impl Service<NetworkRequest> for Peer {
    type Response = NetworkResponse;

    type Error = PeerError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<()>> {
        todo!()
    }

    fn call(&mut self, req: NetworkRequest) -> Self::Future {
        todo!()
    }
}
