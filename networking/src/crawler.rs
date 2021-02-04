use std::{future, net::SocketAddr, pin::Pin};

use config::Config;
use futures::{
    channel::{mpsc, oneshot},
    stream::{FuturesOrdered, FuturesUnordered},
    Future, FutureExt, StreamExt,
};
use tokio::task::JoinHandle;

use crate::{
    address_book::AddressBook, constants::MAX_PENDING_HANDSHAKES, peer_set::PeerChange, Peer,
    PeerError,
};

pub fn start_crawler(
    needs_peers_rx: mpsc::Receiver<()>,
    discovered_peers_tx: mpsc::Sender<PeerChange>,
    crawl_interval: std::time::Duration,
    address_book: AddressBook,
    config: Config,
) -> JoinHandle<Result<(), Box<dyn std::error::Error + Send>>> {
    tokio::spawn(run(
        needs_peers_rx,
        discovered_peers_tx,
        crawl_interval,
        address_book,
        config,
    ))
}
async fn run(
    mut needs_peers_rx: mpsc::Receiver<()>,
    discovered_peers_tx: mpsc::Sender<PeerChange>,
    crawl_interval: std::time::Duration,
    mut address_book: AddressBook,
    config: Config,
) -> Result<(), Box<dyn std::error::Error + Send>> {
    let mut timer = tokio::time::interval(crawl_interval);
    let mut in_progress_connections: FuturesUnordered<
        Pin<Box<dyn futures::Future<Output = Result<Peer, PeerError>> + Send>>,
    > = FuturesUnordered::new();
    // Push a future which never resolves to prevent the FuturesUnordered stream from terminating
    // in_progress_connections.push(future::pending().boxed());
    loop {
        tokio::select! {
            _ = timer.tick() => {
                try_add_peer(&mut in_progress_connections, &mut address_book, config.clone())
            }
            val = needs_peers_rx.next() => {
                todo!()
            }
        }
        return Ok(());
    }
}

fn try_add_peer(
    pending: &mut FuturesUnordered<
        Pin<Box<dyn futures::Future<Output = Result<Peer, PeerError>> + Send>>,
    >,
    address_book: &mut AddressBook,
    config: Config,
) {
    if pending.len() > MAX_PENDING_HANDSHAKES + 1 {
        return;
    }
    if let Some(candidate_addr) = address_book.next_candidate() {
        pending.push(Box::pin(Peer::at_address(candidate_addr, config)))
    }
}
