use config::Config;

#[derive(Debug)]
pub struct Daemon {
    config: Config,
}

#[derive(Debug)]
struct Peer {
    ip_address: std::net::IpAddr,
    services: u64,
}

impl Daemon {
    pub fn new() -> Daemon {
        Daemon {
            config: Config::mainnet(),
        }
    }
}
