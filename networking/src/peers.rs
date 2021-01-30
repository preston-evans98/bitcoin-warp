use crate::interface::{NetworkRequest, NetworkResponse};
use std::{future::Future, net::SocketAddr, pin::Pin};
use tower::{discover::Discover, load::Load, ready_cache::ReadyCache, Service};

pub enum NetworkError {
    Disconnected,
}
/// The high level abstraction representing 'the rest of the network'
pub struct Peers<DiscoverableService>
where
    DiscoverableService: Discover<Key = SocketAddr>,
{
    // node_state: D,
    peers: DiscoverableService,
    ready: ReadyCache<DiscoverableService::Key, DiscoverableService::Service, NetworkRequest>,
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
        todo!()
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
