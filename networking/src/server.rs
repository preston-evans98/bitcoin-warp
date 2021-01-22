use crate::{BitcoinCodec, Message, NetworkRequest, NetworkResponse, PeerError};
use futures::{SinkExt, StreamExt};
use shared::{u256, Block, BlockHeader, EncapsulatedAddr, Transaction};
use std::{
    collections::HashSet, future::Future, pin::Pin, result::Result, task::Poll, unreachable,
};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::codec::Framed;
use tower::Service;

pub struct Server<NodeDataStore> {
    node_state: NodeDataStore,
    state: ServerState,
    connection: Framed<TcpStream, BitcoinCodec>,
    peer_tx: Sender<NetworkResponse>,
    peer_rx: Receiver<NetworkRequest>,
    shutdown_rx: Receiver<()>,
    current_request: Option<NetworkRequest>,
}

enum ServerError {
    Io(String),
}

enum ServerState {
    Ready,
    AwaitingBlocks(HashSet<u256>, Vec<Block>),
    AwaitingTransactions(Vec<Transaction>),
    AwaitingPeers(Vec<EncapsulatedAddr>),
    AwaitingHeaders(Vec<BlockHeader>),
    ConnectionClosed,
}
// TODO: Find permanent home for these
pub struct NodeStateRequest;
pub struct NodeStateResponse;
pub struct NodeStateError;

impl<NodeDataStore> Server<NodeDataStore> {
    pub async fn run(&mut self) -> Result<(), PeerError> {
        loop {
            tokio::select! {
                response = self.connection.next() => {
                    match response {
                        Some(Ok(msg)) => self.handle_msg(msg).await?,
                        None => return Err(PeerError::ConnectionClosed),
                        // FIXME: We can recover from a deserialization error. Do so.
                        Some(Err(e)) => Err(PeerError::from(e))?,
                    }
                }
                request = self.peer_rx.recv() => {
                    let request = request.expect("Must have received value");
                    self.handle_request(request).await
                }
                shutdown = self.shutdown_rx.recv() => break
            }
        }
        Ok(())
    }

    async fn handle_msg(&mut self, msg: Message) -> Result<(), PeerError> {
        match self.state {
            ServerState::ConnectionClosed => Err(PeerError::ConnectionClosed),
            ServerState::Ready => {
                todo!()
            }
            ServerState::AwaitingBlocks(_, _) => self.handle_inbound_blocks(msg).await,
            ServerState::AwaitingTransactions(_) => {
                todo!()
            }
            ServerState::AwaitingPeers(_) => {
                todo!()
            }
            ServerState::AwaitingHeaders(_) => {
                todo!()
            }
        }
    }

    async fn handle_inbound_blocks(&mut self, response: Message) -> Result<(), PeerError> {
        if let ServerState::AwaitingBlocks(ref mut requested_blocks, ref mut accumulated_blocks) =
            self.state
        {
            match response {
                Message::BlockTxn { block_hash, txs } => {
                    unimplemented!()
                }
                Message::Block {
                    mut block_header,
                    transactions,
                } => {
                    // If the block is one we requested, remove it from our pending set and add it to the response
                    if requested_blocks.remove(block_header.hash()) {
                        let block = Block::new(block_header, transactions);
                        accumulated_blocks.push(block);
                    }
                    // Drop unsolicited Blocks
                    Ok(())
                }
                _ => unimplemented!(),
            }
        } else {
            unreachable!("Must only call handle_inbound_blocks while in AwaitingBlocks state");
        }
    }

    async fn handle_inbound_peers(&mut self, response: Message) -> Result<(), Message> {
        match response {
            Message::Addr { addrs } => {
                match self.peer_tx.send(NetworkResponse::Peers(addrs)).await {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Server should not outlive peer_tx! {}", e)
                    }
                }
            }

            _ => return Err(response),
        };
        Ok(())
    }
    // async fn handle_msg_as_response(
    //     &mut self,
    //     request: NetworkRequest,
    //     response: Message,
    // ) -> Result<(), Message> {
    //     match request {
    //         NetworkRequest::Peers => self.handle_inbound_peers(response),
    //         NetworkRequest::BlocksByHash(ref requested_blocks) => {
    //             self.handle_inbound_blocks(request, response)
    //         }

    //         _ => {
    //             todo!()
    //         }
    //         Message::FeeFilter { feerate } => {}
    //         Message::FilterAdd { elements } => {}
    //         Message::FilterClear {} => {}
    //         Message::FilterLoad {
    //             filter,
    //             n_hash_funcs,
    //             n_tweak,
    //             n_flags,
    //         } => {}
    //         Message::GetAddr {} => {}
    //         Message::GetBlockTxn {
    //             block_hash,
    //             indexes,
    //         } => {}
    //         Message::GetBlocks {
    //             protocol_version,
    //             block_header_hashes,
    //             stop_hash,
    //         } => {}
    //         Message::GetData { inventory } => {}
    //         Message::GetHeaders {
    //             protocol_version,
    //             block_header_hashes,
    //             stop_hash,
    //         } => {}
    //         Message::Headers { headers } => {}
    //         Message::Inv { inventory } => {}
    //         Message::MemPool {} => {}
    //         Message::MerkleBlock {
    //             block_header,
    //             transaction_count,
    //             hashes,
    //             flags,
    //         } => {}
    //         Message::NotFound { inventory_data } => {}
    //         Message::Ping { nonce } => {}
    //         Message::Pong { nonce } => {}
    //         Message::Reject {
    //             message,
    //             code,
    //             reason,
    //             extra_data,
    //         } => {}
    //         Message::SendCompact { announce, version } => {}
    //         Message::SendHeaders {} => {}
    //         Message::Tx { transaction } => {}
    //         Message::Verack {} => {}
    //         Message::Version {
    //             protocol_version,
    //             services,
    //             timestamp,
    //             receiver_services,
    //             receiver,
    //             transmitter_services,
    //             transmitter_ip,
    //             nonce,
    //             user_agent,
    //             best_block,
    //             relay,
    //         } => {}
    //     }
    // }

    async fn handle_request(&mut self, request: NetworkRequest) {}
}

// pub enum

// struct ServerFuture<R> {

// };

// impl Future for ServerFuture<R> {
//     type Output = Result<R, ServerError>;

//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//         todo!()
//     }
// }

// impl<T> Server<T> {
//     fn set_state(&mut self, state: ServerState) {
//         self.state = state;
//     }
// }

// impl<NodeState> Service<NetworkRequest> for Server<NodeState>
// where
//     NodeState: Service<NodeStateRequest, Response = NodeStateResponse, Error = NodeStateError>
//         + Clone
//         + Send
//         + 'static,
//     NodeState::Future: Send,
// {
//     type Response = NetworkResponse;

//     type Error = PeerError;

//     type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(
//         &mut self,
//         _cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         match self.state {
//             ServerState::Ready => Poll::Ready(Ok(())),
//             ServerState::ConnectionClosed => Poll::Ready(Err(PeerError::ConnectionClosed)),
//             ServerState::AwaitingResponse => Poll::Pending,
//         }
//     }

//     fn call(&mut self, req: NetworkRequest) -> Self::Future {
//         // Box::pin(async move {
//         //     match req {
//         //         NetworkRequest::Peers => {
//         //             self.set_state(ServerState::AwaitingResponse);
//         //             self.connection.send(Message::GetAddr {}).await;
//         //         }
//         //         NetworkRequest::BlocksByHash(_) => {}
//         //         NetworkRequest::TransactionsByHash(_) => {}
//         //         NetworkRequest::Headers {
//         //             last_known_headers,
//         //             max_responses,
//         //         } => {}
//         //         NetworkRequest::PushTransaction(_) => {}
//         //         NetworkRequest::AdvertiseTransactions(_) => {}
//         //         NetworkRequest::AdvertiseBlock(_) => {}
//         //         NetworkRequest::Mempool => {}
//         //     }
//         //     Ok(NetworkResponse::Success)
//         // })
//         todo!()
//     }
// }
