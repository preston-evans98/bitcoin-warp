use crate::interface::{NetworkRequest, NetworkResponse};
use std::{collections::{HashMap, HashSet}, future::Future, net::SocketAddr, pin::Pin, time::Instant};
use futures::{channel::{mpsc, oneshot}, stream::FuturesUnordered};
use tokio::{sync::broadcast, task::JoinHandle};
use tower::{BoxError, Service, discover::Discover, load::Load, ready_cache::ReadyCache};
use shared::{Block, BlockHash, Transaction, TxID, u256};
pub enum NetworkError {
    Disconnected,
}
/// The high level abstraction representing 'the rest of the network'
pub struct Peers<DiscoverableService>
where
    DiscoverableService: Discover<Key = SocketAddr>,
{
    node_discovery: DiscoverableService,
    peers: DiscoverableService,
    ready: ReadyCache<DiscoverableService::Key, DiscoverableService::Service, NetworkRequest>,
    // not_ready: FuturesUnordered<UnreadyCache<DiscoverableService::Key, DiscoverableService::Service, NetworkRequest>>,
    cancel_handles: HashMap<DiscoverableService::Key, oneshot::Sender<()>>,
    demand_signal: mpsc::Sender<()>,
    handle_rx: tokio::sync::oneshot::Receiver<Vec<JoinHandle<Result<(), BoxError>>>>,
    p2c_next_peer_index: Option<usize>,
    guards: futures::stream::FuturesUnordered<JoinHandle<Result<(), BoxError>>>,
    // inventory_registry: InventoryRegistry,
    /// The last time we logged a message about the peer set size
    last_peer_log: Option<Instant>,

}

impl<DiscoverableService> Service<NetworkRequest> for Peers<DiscoverableService>
where
    DiscoverableService: Discover<Key = SocketAddr> + Unpin,
    DiscoverableService::Service: Service<NetworkRequest, Response = NetworkResponse> + Load,
{
    type Response = NetworkResponse;

    type Error = NetworkError;

    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: NetworkRequest) -> Self::Future {
        match req{
            NetworkRequest::BlocksByHash(hash_set) => {
                self.route_to_peer_with_inv(req,hash_set)
            }
            NetworkRequest::TransactionsByHash(hash_set) => {
                self.route_to_peer_with_inv(req,hash_set)
            }
            NetworkRequest::PushTransaction(_) => {
                self.route_to_all_peers(req)
            }
            NetworkRequest::AdvertiseTransactions(_) => {
                self.route_to_all_peers(req)
            }
            NetworkRequest::AdvertiseBlock(_) => {
                self.route_to_all_peers(req)
            }
            _ => {
                self.route_to_one_peer(req)
            }
        }
    }
    
}
impl<DiscoverableService> Peers<DiscoverableService>
where
    DiscoverableService: Discover<Key = SocketAddr> + Unpin,
    DiscoverableService::Service: Service<NetworkRequest, Response = NetworkResponse> + Load,
{
    pub fn new(
        discover: DiscoverableService,
        demand_signal: mpsc::Sender<()>,
        handle_rx: tokio::sync::oneshot::Receiver<Vec<JoinHandle<Result<(), BoxError>>>>,
        inv_stream: broadcast::Receiver<(InventoryHash, SocketAddr)>,
    ) -> Self {
        Self {
            cancel_handles: HashMap::new(),
            demand_signal,
            guards: futures::stream::FuturesUnordered::new(),
            handle_rx,
            last_peer_log: None,
            node_discovery: (),
            peers: (),
            ready: (),
            p2c_next_peer_index: (),
        }
    }
    fn route_to_one_peer(&mut self, req: NetworkRequest) -> <Self as tower::Service<NetworkRequest>>::Future{
        let index = self
            .preselected_p2c_index
            .take()
            .expect("ready service must have valid preselected index");

        let (key, mut svc) = self
            .ready_services
            .swap_remove_index(index)
            .expect("preselected index must be valid");

        let fut = svc.call(req);
        self.push_unready(key, svc);
        fut.map_err(Into::into).boxed()
    }
    fn route_to_all_peers(&mut self, req: NetworkRequest) -> <Self as tower::Service<NetworkRequest>>::Future{
        
    }
    fn route_to_peer_with_inv(&mut self, req: NetworkRequest, hash_set: HashSet<BlockHash>) -> <Self as tower::Service<NetworkRequest>>::Future{
        
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum InventoryHash {
    Error,
    Tx(TxID),
    Block(BlockHash),
    FilteredBlock(),
}
impl From<BlockHash> for InventoryHash {
    fn from(tx: Transaction) -> InventoryHash {
        InventoryHash::Tx(tx)
    }
}

impl From<TxID> for InventoryHash {
    fn from(hash: BlockHash) -> InventoryHash {
        InventoryHash::Block(hash)
    }
}

//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {

//         Poll::Pending
//     }

//     fn call(&mut self, req: NetworkRequest) -> Self::Future {
//         // self.
//         let idx = self.ready.ready_len().saturating_sub(1);
//         self.ready.call_ready_index(idx)
//         // if self.ready.check_ready_index(cx, key) {

//         // }
//     }
// }
