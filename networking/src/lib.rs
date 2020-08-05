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
        message.create_header(Command::Verack, &Config::mainnet());
        assert_eq!(
            message.get_header().hex(),
            "f9beb4d976657261636b000000000000000000005df6e0e2"
        )
    }
}
