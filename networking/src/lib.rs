mod command;
pub use command::Command;

mod message;
pub use message::Message;

mod peer;
pub use peer::Peer;

mod peer_connect;
pub use peer_connect::outbound_connection;


mod header;

mod messages;
pub use messages::Version;
pub use messages::GetBlocks;
pub use messages::GetData;
pub use messages::InventoryData;
pub use messages::InventoryType;






#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::Message;
    use config::Config;
    use crate::Peer;
    use std::net::{SocketAddr,IpAddr,Ipv4Addr};
    use crate::peer_connect;
    #[test] 
    fn test_verack() {
        let mut message = Message::new();
        message.create_header_for_body(Command::Verack, Config::mainnet().magic());
        assert_eq!(
            message.get_header().hex(),
            "f9beb4d976657261636b000000000000000000005df6e0e2"
        )
    }
    use crate::Version;
    
    #[test]
    fn test_version_serialize(){
        let foreign_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8333);
        let local_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8333);
        let version = Version::new(foreign_address,01 as u64,local_address,0 as u32,&Config::mainnet());
        
        assert_eq!(
            versoin.hex(),
            version.serialize();
        )
        // Peer::at_address(1, address, &Config::mainnet());

        //peer_connect::outbound_connection();
        //println!("The connection returned: {:#?}",result);
    }
    // #[test]
    // fn test_getblocks() {
    //     let mut message = Message::new();
    //     let mut conf = Config::mainnet();
    //     conf.set_protocol_version(70001 as u32);
    //     //message.create_header_for_body(Command::GetBlocks,&conf);
    //     message.create_getblocks_body(&vec![], true, &conf);
    //     assert_eq!(
    //         message.get_body().hex(),
    //         "71110100000000000000000000000000000000000000000000000000000000000000000000"
    //     )
    // }

    // use std::net::Ipv4Addr;
    // #[test]
    // fn test_tcp_message() {
    //     let mut msg = Message::new();
    //         msg.create_version_body(&Config::mainnet());
    //         msg.create_header_for_body(Command::Version, &Config::mainnet());
    //         println!("{:?} {:?}",msg.dump_header(),msg.dump_body());
    //         println!("{:?}",msg.dump_contents());
    //         println!("{:?}",msg.get_contents().get_bytes());

    //     assert_eq!(
    //         msg.dump_body(),
    //         "721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001"
    //     )
    // }
}
