pub mod shell {
    use crate::Daemon;
    use std::io::Write;
    fn read_trimmed(mut raw_input: &mut String) -> &str {
        raw_input.truncate(0);
        std::io::stdin().read_line(&mut raw_input).unwrap();
        raw_input.trim_end()
    }

    fn write_prompt(message: &str) {
        print!("{} ", message);
        std::io::stdout()
            .flush()
            .expect("Yikes. Stdout isn't working");
    }

    pub async fn run_shell() -> Result<(), Box<dyn std::error::Error>> {
        let mut daemon = Daemon::new();
        let mut raw_input = String::new();
        loop {
            write_prompt("warp shell>");
            let input = read_trimmed(&mut raw_input);
            match &input[..] {
                "exit" | "q" => break,
                "add" | "a" => {
                    write_prompt("  enter an address:");
                    let input = read_trimmed(&mut raw_input);
                    if let Ok(addr) = input.parse() {
                        println!("  Connecting...");
                        if let Err(e) = daemon.add_peer(addr).await {
                            println!("Could not connect to peer: {}", e.to_string());
                            continue;
                        } else {
                            println!("Peer added at {}", addr);
                        }
                    } else {
                        println!("Could not interpret {} as ip address and port", input);
                    }
                }
                "listen" | "l" => {
                    write_prompt("  enter a port:");
                    let addr = read_trimmed(&mut raw_input);
                    println!("  Listening...");
                    if let Err(e) = daemon.accept_peer(addr).await {
                        println!("Could not accept connection: {}", e.to_string());
                        continue;
                    } else {
                        println!("Peer added at {}", addr);
                    }
                }
                _ => continue,
            }
        }
        Ok(())
    }
}
