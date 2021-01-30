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
                self.handle_ready(msg).await
            }
            ServerState::AwaitingBlocks(_, _) => {
                self.handle_inbound_blocks(msg).await
            }
            ServerState::AwaitingTransactions(_) => {
                todo!()
            }
            ServerState::AwaitingPeers(_) => {
                //If we want to return the error or result back up we need to recieve it from the handle_inbound function call
                let result = self.handle_inbound_peers(msg).await;
                Ok(())
            }
            ServerState::AwaitingHeaders(_) => {
                let result = self.handle_inbound_headers(msg).await;
                Ok(())
            }
        }
    }
    ///This function handles inbound unsolicited messages
    async fn handle_ready(&mut self, response: Message) -> Result<(), PeerError> {
        match response {
            Message::FilterLoad { filter,n_hash_funcs,n_flags,n_tweak } => {
                self.load_filter(filter,n_hash_funcs,n_flags,n_tweak);
                Ok(())
            }
            Message::FilterAdd { elements} => {
                self.add_filter(elements);
                Ok(())
            }
            Message::FilterClear { } => {
                self.clear_filter();
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
    ///This function handles inbound blocks when the Warp node has requested and is awaiting blocks
    async fn handle_inbound_blocks(&mut self, response: Message) -> Result<(), PeerError> {
        if let ServerState::AwaitingBlocks(ref mut requested_blocks, ref mut accumulated_blocks) =
            self.state
        {
            match response {
                Message::BlockTxn { block_hash, txs } => {
                    unimplemented!()
                }
                Message::Block { block } => {
                    // If the block is one we requested, remove it from our pending set and add it to the response
                    if requested_blocks.remove(block.header().hash()) {
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
        //TODO might need an if to check the serverstate as the other ones do.
        if let ServerState::AwaitingPeers(ref mut accumulated_headers) =
            self.state
        {
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
        else {
            unreachable!("Must only call handle_inbound_peers while in AwaitingPeers state");
        }
    }
    ///This function handles messages when the Warp node has requested and is awaiting headers from the peer.
    ///Header messages are accepted as valid and any other message is discarded
    async fn handle_inbound_headers(&mut self, response: Message) -> Result<(), PeerError> {
        if let ServerState::AwaitingHeaders(ref mut accumulated_headers) =
            self.state
        {
            match response {
                Message::Headers { headers } => {
                    //assuming headers are the ones we want, may want to come back and check them here first before passing them back to peer. TODO
                    self.state = ServerState::Ready;
                    match self.peer_tx.send(NetworkResponse::Headers(headers)).await{
                        Ok(_) => {Ok(())}
                        Err(e) => {
                            panic!("Server should not outlive peer_tx! {}", e)               
                        }
                    }
                }
                Message::Reject {message, code, reason ,extra_data }=>{
                    //need to log the reject message and then return the error back up
                    Err(PeerError::MessageRejected(reason))
                }
                _ => unimplemented!(),
            }
        } else {
            unreachable!("Must only call handle_inbound_blocks while in AwaitingBlocks state");
        }
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
    async fn load_filter(&mut self,filter:Vec<u8>,n_hash_funcs:u32,n_flags:u8,n_tweak:u32){

    }
    async fn add_filter(&mut self,elements:Vec<Vec<u8>>){

    }
    async fn clear_filter(&mut self){

    }

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
