use daemon::Daemon;
use networking::{Command, Message};

fn main() {
    let daemon = Daemon::new();
    println!("{:?}", daemon);

    let command = Command::Version; 
    execute_command(command, daemon);
    

}
fn execute_command(command: Command, daemon: Daemon){
    match command {
        Command::Version => {
            let mut msg = Message::new();
            msg.create_header_for_body(Command::Version, &daemon.config);
            msg.create_version_body(&daemon.config);
            println!("{:?} {:?}",msg.dump_header(),msg.dump_body());
        }
        _ => {

        }
    }

}