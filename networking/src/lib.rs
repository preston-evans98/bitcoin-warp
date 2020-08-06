mod command;
pub use command::Command;

mod message;
pub use message::Message;

#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::Message;
    use config::Config;
    #[test]
    fn test_verack() {
        let mut message = Message::new();
        message.create_header_for_body(Command::Verack, &Config::mainnet());
        assert_eq!(
            message.get_header().hex(),
            "f9beb4d976657261636b000000000000000000005df6e0e2"
        )
    }
    #[test]
    fn test_getblocks() {
        let mut message = Message::new();
        let mut conf = Config::mainnet();
        conf.set_protocol_version(70001 as u32);
        //message.create_header_for_body(Command::GetBlocks,&conf);
        message.create_getblocks_body(&vec![], true, &conf);
        assert_eq!(
            message.get_body().hex(),
            "71110100000000000000000000000000000000000000000000000000000000000000000000"
        )
    }
}
