const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
const MAGIC_TESTNET: u32 = 0x0709110B;
const MAGIC_REGTEST: u32 = 0xDAB5BFFA;

const CORE_PORT_MAINNET: usize = 8333;
const CORE_PORT_TESTNET: usize = 18333;
const CORE_PORT_REGTEST: usize = 18444;

const WARP_PORT_MAINNET: usize = 8333;
const WARP_PORT_TESTNET: usize = 18333;
const WARP_PORT_REGTEST: usize = 18444;

#[derive(Debug)]
pub struct Config {
    client_version: String,
    protocol_version: usize,
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
        }
    }
    pub fn testnet() -> NetworkConfig {
        NetworkConfig {
            core_port: CORE_PORT_TESTNET,
            warp_port: WARP_PORT_TESTNET,
            magic: MAGIC_TESTNET,
        }
    }
    pub fn regtest() -> NetworkConfig {
        NetworkConfig {
            core_port: CORE_PORT_REGTEST,
            warp_port: WARP_PORT_REGTEST,
            magic: MAGIC_REGTEST,
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
