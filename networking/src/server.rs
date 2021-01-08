use crate::Message;
use crate::{BitcoinCodec, PeerError};
use std::{future::Future, pin::Pin, result::Result, task::Poll};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tower::Service;

pub struct Server<NodeDataStore> {
    node_state: NodeDataStore,
    state: ServerState,
    connection: Framed<TcpStream, BitcoinCodec>,
}

enum ServerError {
    Io(String),
}

enum ServerState {
    Ready,
    AwaitingResponse,
    ConnectionClosed,
}
// TODO: Find permanent home for these
pub struct NodeStateRequest;
pub struct NodeStateResponse;
pub struct NodeStateError;

// pub enum

// struct ServerFuture<R> {

// };

// impl Future for ServerFuture<R> {
//     type Output = Result<R, ServerError>;

//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//         todo!()
//     }
// }

impl<NodeState> Service<Message> for Server<NodeState>
where
    NodeState: Service<NodeStateRequest, Response = NodeStateResponse, Error = NodeStateError>
        + Clone
        + Send
        + 'static,
    NodeState::Future: Send,
{
    type Response = Message;

    type Error = PeerError;

    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.state {
            ServerState::Ready => Poll::Ready(Ok(())),
            ServerState::ConnectionClosed => Poll::Ready(Err(PeerError::ConnectionClosed)),
            ServerState::AwaitingResponse => Poll::Pending,
        }
    }

    fn call(&mut self, req: Message) -> Self::Future {
        match req.command() {
            crate::Command::Version => {}
            crate::Command::Verack => {}
            crate::Command::GetBlocks => {}
            crate::Command::GetData => {}
            crate::Command::Block => {}
            crate::Command::GetHeaders => {}
            crate::Command::Headers => {}
            crate::Command::Inv => {}
            crate::Command::MemPool => {}
            crate::Command::MerkleBlock => {}
            crate::Command::CmpctBlock => {}
            crate::Command::GetBlockTxn => {}
            crate::Command::BlockTxn => {}
            crate::Command::SendCmpct => {}
            crate::Command::NotFound => {}
            crate::Command::Tx => {}
            crate::Command::Addr => {}
            crate::Command::Alert => {}
            crate::Command::FeeFilter => {}
            crate::Command::FilterAdd => {}
            crate::Command::FilterClear => {}
            crate::Command::FilterLoad => {}
            crate::Command::GetAddr => {}
            crate::Command::Ping => {}
            crate::Command::Pong => {}
            crate::Command::Reject => {}
            crate::Command::SendHeaders => {}
        }
        // match req {
        //     Message::Addr { ref addrs } => if self.state == ServerState::AwaitingAddr {
        //         self.db.
        //     },
        //     Message::BlockTxn { block_hash, txs } => {}
        //     Message::Block {
        //         block_header,
        //         transactions,
        //     } => {}
        //     Message::CompactBlock {
        //         header,
        //         nonce,
        //         short_ids,
        //         prefilled_txns,
        //     } => {}
        //     Message::FeeFilter { feerate } => {}
        //     Message::FilterAdd { elements } => {}
        //     Message::FilterClear {} => {}
        //     Message::FilterLoad {
        //         filter,
        //         n_hash_funcs,
        //         n_tweak,
        //         n_flags,
        //     } => {}
        //     Message::GetAddr {} => {}
        //     Message::GetBlockTxn {
        //         block_hash,
        //         indexes,
        //     } => {}
        //     Message::GetBlocks {
        //         protocol_version,
        //         block_header_hashes,
        //         stop_hash,
        //     } => {}
        //     Message::GetData { inventory } => {}
        //     Message::GetHeaders {
        //         protocol_version,
        //         block_header_hashes,
        //         stop_hash,
        //     } => {}
        //     Message::Headers { headers } => {}
        //     Message::Inv { inventory } => {}
        //     Message::MemPool {} => {}
        //     Message::MerkleBlock {
        //         block_header,
        //         transaction_count,
        //         hashes,
        //         flags,
        //     } => {}
        //     Message::NotFound { inventory_data } => {}
        //     Message::Ping { nonce } => {}
        //     Message::Pong { nonce } => {}
        //     Message::Reject {
        //         message,
        //         code,
        //         reason,
        //         extra_data,
        //     } => {}
        //     Message::SendCompact { announce, version } => {}
        //     Message::SendHeaders {} => {}
        //     Message::Tx { transaction } => {}
        //     Message::Verack {} => {}
        //     Message::Version {
        //         protocol_version,
        //         services,
        //         timestamp,
        //         receiver_services,
        //         receiver,
        //         transmitter_services,
        //         transmitter_ip,
        //         nonce,
        //         user_agent,
        //         best_block,
        //         relay,
        //     } => {}
        // todo!()
        // }
        todo!()
    }
}
