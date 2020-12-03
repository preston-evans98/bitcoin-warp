const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
const MAGIC_TESTNET: u32 = 0x0709110B;
const MAGIC_REGTEST: u32 = 0xDAB5BFFA;

const CORE_PORT_MAINNET: usize = 8333;
const CORE_PORT_TESTNET: usize = 18333;
const CORE_PORT_REGTEST: usize = 18444;

const WARP_PORT_MAINNET: usize = 8333;
const WARP_PORT_TESTNET: usize = 18333;
const WARP_PORT_REGTEST: usize = 18444;

// Max message size: 4 Mb (see https://github.com/bitcoin/bitcoin/blob/master/src/net.h)
const MAX_SIZE_MAINNET: usize = 4 * 1000 * 1000;
const MAX_SIZE_TESTNET: usize = 4 * 1000 * 1000;
const MAX_SIZE_REGTEST: usize = 4 * 1000 * 1000;

const MAX_PEERS_MAINNET: usize = 10;
const MAX_PEERS_TESTNET: usize = 10;
const MAX_PEERS_REGTEST: usize = 10;

#[derive(Debug)]
pub struct Config {
    client_version: String,
    protocol_version: u32,
    services: u64,
    ip_address: std::net::IpAddr,
    user_agent: String,
    network: Network,
    network_config: NetworkConfig,
}

#[derive(Debug)]
pub struct NetworkConfig {
    core_port: usize,
    warp_port: usize,
    magic: u32,
    max_msg_size: usize,
    max_peers: usize,
    max_warp_peers: usize,
}

#[derive(Debug)]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
}

impl NetworkConfig {
    pub fn mainnet() -> NetworkConfig {
        NetworkConfig {
            core_port: CORE_PORT_MAINNET,
            warp_port: WARP_PORT_MAINNET,
            magic: MAGIC_MAINNET,
            max_msg_size: MAX_SIZE_MAINNET,
            max_peers: MAX_PEERS_MAINNET,
            max_warp_peers: MAX_PEERS_MAINNET,
        }
    }
    pub fn testnet() -> NetworkConfig {
        NetworkConfig {
            core_port: CORE_PORT_TESTNET,
            warp_port: WARP_PORT_TESTNET,
            magic: MAGIC_TESTNET,
            max_msg_size: MAX_SIZE_TESTNET,
            max_peers: MAX_PEERS_TESTNET,
            max_warp_peers: MAX_PEERS_TESTNET,
        }
    }
    pub fn regtest() -> NetworkConfig {
        NetworkConfig {
            core_port: CORE_PORT_REGTEST,
            warp_port: WARP_PORT_REGTEST,
            magic: MAGIC_REGTEST,
            max_msg_size: MAX_SIZE_REGTEST,
            max_peers: MAX_PEERS_REGTEST,
            max_warp_peers: MAX_PEERS_REGTEST,
        }
    }
}

impl Network {
    pub fn mainnet() -> Network {
        Network::Mainnet
    }
    pub fn testnet() -> Network {
        Network::Testnet
    }
    pub fn regtest() -> Network {
        Network::Regtest
    }
}

impl Config {
    pub fn mainnet() -> Config {
        Config {
            client_version: String::from(env!("CARGO_PKG_VERSION")),
            protocol_version: 70015,
            services: 0,
            ip_address: "127.0.0.1".parse().unwrap(),
            user_agent: String::from("bitcoin-warp"),
            network: Network::mainnet(),
            network_config: NetworkConfig::mainnet(),
        }
    }
    pub fn magic(&self) -> u32 {
        self.network_config.magic
    }
    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version
    }
    pub fn set_protocol_version(&mut self, version: u32) {
        self.protocol_version = version;
    }
    pub fn get_services(&self) -> u64 {
        self.services
    }
    pub fn get_max_msg_size(&self) -> usize {
        self.network_config.max_msg_size
    }
    pub fn max_core_peers(&self) -> usize {
        self.network_config.max_peers - self.network_config.max_warp_peers
    }
    pub fn max_warp_peers(&self) -> usize {
        self.network_config.max_warp_peers
    }
    pub fn max_peers(&self) -> usize {
        self.network_config.max_peers
    }
}
