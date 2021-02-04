use crate::{
    constants,
    interface::{NetworkRequest, NetworkResponse},
    Peer, PeerError,
};
use futures::{
    channel::{mpsc, oneshot},
    future,
    stream::{FuturesUnordered, StreamExt},
};
use shared::{u256, Block, BlockHash, Transaction, TxID};
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    net::SocketAddr,
    pin::Pin,
    time::Instant,
};
use tokio::{sync::broadcast, task::JoinHandle};
use tower::{
    discover::{Change, Discover},
    load::{Load, PeakEwmaDiscover},
    ready_cache::ReadyCache,
    BoxError, Service,
};
pub enum NetworkError {
    Disconnected,
}
pub type PeerChange = Result<Change<SocketAddr, Peer>, PeerError>;
/// The high level abstraction representing 'the rest of the network'
pub struct PeerSet<DiscoverableService>
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

impl<D> PeerSet<D>
where
    D: Discover<Key = SocketAddr> + Unpin,
    D::Service: Service<NetworkRequest, Response = NetworkResponse> + Load,
{
    pub fn new() -> PeerSet<D> {
        // let p2c = MakeBalance::new(tower::discover::ServiceList::new(vec![svc1, svc2]));
        let (discovered_peers_tx, discovered_peers_rx) = mpsc::channel::<PeerChange>(50);
        let (needs_peers_tx, needs_peers_rx) = mpsc::channel::<()>(1);
        let peerset = PeakEwmaDiscover::new(
            discovered_peers_rx.filter(|result| future::ready(result.is_ok())),
            constants::DEFAULT_EWMA_RTT,
            constants::EWMA_DECAY_RATE,
            tower::load::CompleteOnResponse::default(),
        );

        todo!()
    }
}

impl<DiscoverableService> Service<NetworkRequest> for PeerSet<DiscoverableService>
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
        match req {
            NetworkRequest::BlocksByHash(_) => self.route_to_peer_with_inv(req),
            NetworkRequest::TransactionsByHash(_) => self.route_to_peer_with_inv(req),
            NetworkRequest::PushTransaction(_) => self.route_to_all_peers(req),
            NetworkRequest::AdvertiseTransactions(_) => self.route_to_all_peers(req),
            NetworkRequest::AdvertiseBlock(_) => self.route_to_all_peers(req),
            _ => self.route_to_one_peer(req),
        }
    }
}
impl<DiscoverableService> PeerSet<DiscoverableService>
where
    DiscoverableService: Discover<Key = SocketAddr> + Unpin,
    DiscoverableService::Service: Service<NetworkRequest, Response = NetworkResponse> + Load,
{
    // pub fn new(
    //     discover: DiscoverableService,
    //     demand_signal: mpsc::Sender<()>,
    //     handle_rx: tokio::sync::oneshot::Receiver<Vec<JoinHandle<Result<(), BoxError>>>>,
    //     inv_stream: broadcast::Receiver<(InventoryHash, SocketAddr)>,
    // ) -> Self {
    //     Self {
    //         cancel_handles: HashMap::new(),
    //         demand_signal,
    //         guards: futures::stream::FuturesUnordered::new(),
    //         handle_rx,
    //         last_peer_log: None,
    //         node_discovery: (),
    //         peers: (),
    //         ready: (),
    //         p2c_next_peer_index: (),
    //     }
    // }
    fn route_to_one_peer(
        &mut self,
        req: NetworkRequest,
    ) -> <Self as tower::Service<NetworkRequest>>::Future {
        let index = self
            .p2c_next_peer_index
            .take()
            .expect("ready service must have valid preselected index");

        let (key, mut svc) = self
            .ready
            .get_ready_index_mut(index)
            .expect("chosen index must be in range");

        let fut = svc.call(req);
        todo!();
        // FIXME: Uncomment
        // self.push_unready(key, svc);
        // fut.map_err(Into::into).boxed()
    }
    fn route_to_all_peers(
        &mut self,
        req: NetworkRequest,
    ) -> <Self as tower::Service<NetworkRequest>>::Future {
        todo!()
    }
    fn route_to_peer_with_inv(
        &mut self,
        req: NetworkRequest,
    ) -> <Self as tower::Service<NetworkRequest>>::Future {
        todo!()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum InventoryHash {
    Error,
    Tx(TxID),
    Block(BlockHash),
    FilteredBlock(),
}
impl From<TxID> for InventoryHash {
    fn from(tx: TxID) -> InventoryHash {
        InventoryHash::Tx(tx)
    }
}

impl From<BlockHash> for InventoryHash {
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
