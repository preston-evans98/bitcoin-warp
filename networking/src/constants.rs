use std::time::Duration;

/// The default round-trip-time estimate for new peers
///
/// The higher this value is, the more heavily loaded new peers will appear to be.
/// A high value causes the load balancer to prefer routing requests to more established peers
/// until such time as it can establish a reliable estimate of the peer's true RTT.
///
/// Note: Zcash Zebra sets their default RTT to 1 second longer than the request timeout duration.
/// This makes the load balancer avoid routing to new peers unless absolutely necessary.
pub const DEFAULT_EWMA_RTT: Duration = Duration::from_secs(15);

/// The decay rate of the EWMA load estimate
///
/// The larger this value, the larger load estimate will be for each peer
pub const EWMA_DECAY_RATE: Duration = Duration::from_secs(5 * 60);

/// The maximum number of handshakes that can be pending at any time.
///
/// Since adding peers is a high-latency operation, we expect lots of them to be added at once
/// when backpressure on the network gets too hight. This constant keeps the crawler from going too far overboard
pub const MAX_PENDING_HANDSHAKES: usize = 20;
