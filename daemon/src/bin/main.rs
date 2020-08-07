use daemon::Daemon;
use networking::{Command, Message};
use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::Duration;
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
            msg.create_version_body(&daemon.config);
            msg.create_header_for_body(Command::Version, &daemon.config);
            println!("{:?} {:?}",msg.dump_header(),msg.dump_body());
            println!("{:?}",msg.dump_contents());
            println!("{:?}",msg.get_contents().get_bytes());



            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8333") {
                println!("Connected to the server!");
                println!("{:?}",stream.local_addr().unwrap().ip());
                stream.set_read_timeout(Some(Duration::new(3, 0))).unwrap();
                stream.write(msg.get_contents().get_bytes()).unwrap();
                //stream.write(msg.get_body().get_bytes()).unwrap();
                let mut response = [0; 128];
                stream.read(&mut response).unwrap();
                for byte in response.iter() {
                    println!("{}", byte);
                }
                
                
            } else {
                println!("Couldn't connect to server...");
            }

        }
        _ => {

        }
    }

}