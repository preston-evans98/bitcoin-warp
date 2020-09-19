use config::Config;
use log::warn;
use shared::u256;

pub struct GetHeaders {
    protocol_version: u32,
    block_header_hashes: Vec<u256>,
    stop_hash: u256,
}

impl GetHeaders {
    pub fn new(block_hashes: Vec<u256>, inv_message: bool, config: &Config) -> GetHeaders {
        let mut message = GetHeaders {
            protocol_version: config.get_protocol_version(),
            block_header_hashes: block_hashes,
            stop_hash: u256::new(),
        };
        if !inv_message {
            //The header hash of the last header hash being requested; set to all zeroes to request an “inv” message
            //with all subsequent header hashes (a maximum of 2000 will be sent as a reply to this message;
            //if you need more than 2000, you will need to send another "getheaders" message with a higher-height
            //header hash as the first entry in block header hash field).
            match message.block_header_hashes.last() {
                Some(hash) => {} // message.stop_hash = *hash.clone(),
                None => {
                    warn!("GetHeaders: stop hash was empty");
                }
            }
        }
        message
    }
}
