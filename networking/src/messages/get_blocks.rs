use shared::{CompactInt,Bytes, u256};
use config::Config;
use log::warn;

pub struct GetBlocks{
    protocol_version: u32,
    hash_count: CompactInt,
    block_header_hashes: Vec<u256>,
    stop_hash: u256
}

impl GetBlocks{
    pub fn new(block_hashes: Vec<u256>,
        inv_message: bool,
        config: &Config) -> GetBlocks{
        let mut message = GetBlocks {
            protocol_version: config.get_protocol_version(),
            hash_count: CompactInt::from(block_hashes.len()),
            block_header_hashes: block_hashes,
            stop_hash: u256::new(),
        };
        if !inv_message{
            //The header hash of the last header hash being requested; set to all zeroes to request an “inv” message
            //with all subsequent header hashes (a maximum of 500 will be sent as a reply to this message; 
            //if you need more than 500, you will need to send another “getblocks” message with a higher-height 
            //header hash as the first entry in block header hash field).
            match message.block_header_hashes.last() {
                Some(hash) => message.stop_hash = *hash.clone(),
                None => {
                    warn!("GetBlocks: stop hash was empty");
                }
            }
        }
        message
    }

    // pub fn new(payload: Payload::GetBlocksPayload,config: &Config) -> GetBlocks {
    //     let mut message = GetBlocks {
    //         protocol_version: config.get_protocol_version(),
    //         hash_count: CompactInt::from(payload.block_hashes.len()),
    //         block_header_hashes: Vec.new(),
    //         stop_hash: Vec.new(),
    //     };
    //     for hash in payload.block_hashes.iter() {
    //         message.block_header_hashes.append(hash)
    //     }
    //     if *inv_message {
    //         message.body.append(u256::new());
    //     } else {
    //         match payload.block_hashes.last() {
    //             Some(hash) => message.body.append(hash),
    //             None => {
    //                 warn!("GetBlocks: stop hash was empty");
    //                 message.body.append(u256::new());
    //             }
    //         }
    //     }
    //     //msg.create_header_for_body(Command::GetBlocks, config.magic());
    //     return message;    
    //}

}
