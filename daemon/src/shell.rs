pub mod shell {
    use crate::Daemon;
    use networking::Command;
    use networking::Peer;
    use std::io::Write;
    use tokio::io::AsyncBufReadExt;
    async fn read_trimmed(mut raw_input: &mut String) -> &str {
        raw_input.truncate(0);
        tokio::io::BufReader::new(tokio::io::stdin())
            .read_line(&mut raw_input)
            .await
            .unwrap();
        println!("Input: {}", raw_input);
        raw_input.trim_end()
    }

    fn write_prompt(message: &str) {
        print!("{} ", message);
        std::io::stdout()
            .flush()
            .expect("Yikes. Stdout isn't working");
    }
    async fn peer_get_command() -> String {
        let mut raw_input = String::new();
        write_prompt("warp shell - peer mode>");
        tokio::io::BufReader::new(tokio::io::stdin())
            .read_line(&mut raw_input)
            .await
            .unwrap();
        raw_input
    }

    async fn peer_mode(peer: &mut Peer) -> Result<(), networking::PeerError> {
        loop {
            tokio::select! {
                val = peer.receive(None) => {
                    if let Err(e) = val {
                        println!("  Peer is experiencing issues. Disconnecting...");
                        return Err(e)
                    }
                },
                val = peer_get_command() => {
                    let cmd = val.trim_end();
                    match cmd {
                        "b" | "q" => return Ok(()),
                        "l" => {
                            println!("Waiting to receive...");
                                let _ = peer.receive(None).await;},
                        "h" | "help" => {
                            println!("peer shell commands: ");
                            println!("   b: back");
                            println!("   version: send version msg");
                            println!("   verack: send verack");
                        }
                        "Version" | "version" =>  {
                            write_prompt("  Sending... ");
                            peer.send(Command::Version, peer.create_version_msg(None)).await.unwrap();
                            write_prompt("Done\n");
                        }
                        "Verack" | "verack" =>  {
                            write_prompt("  Sending... ");
                            peer.send(Command::Verack, networking::Verack{}).await.unwrap();
                            write_prompt("  Done\n");
                        }
                        _ => {
                            println!("Unrecognized command: {}", cmd);
                            continue;
                        },
                    }
                }
            }
        }
    }

    pub async fn run_shell() -> Result<(), Box<dyn std::error::Error>> {
        let mut daemon = Daemon::new();
        let mut raw_input = String::new();
        loop {
            write_prompt("warp shell>");
            let input = read_trimmed(&mut raw_input).await;
            match &input[..] {
                "exit" | "q" => break,
                "help" | "h" => {
                    println!("warp shell commands: ");
                    println!("   q: quit");
                    println!("   a: add a peer (outbound)");
                    println!("   l: listen for connection (add peer - inbound)");
                    println!("   p: switch to peer mode");
                }
                "add" | "a" | "connect" | "c" => {
                    write_prompt("  enter an address:");
                    let input = read_trimmed(&mut raw_input).await;
                    if let Ok(addr) = input.parse() {
                        println!("  connecting...");
                        if let Err(e) = daemon.add_peer(addr).await {
                            println!("could not connect to peer: {}", e.to_string());
                            continue;
                        } else {
                            println!("peer added at {}", addr);
                            let peer: &mut Peer = daemon
                                .conn_man
                                .peers
                                .last_mut()
                                .expect("Should have just added a peeer");
                            if let Err(_) = peer_mode(peer).await {
                                daemon.conn_man.peers.pop();
                            }
                        }
                    } else {
                        println!("could not interpret {} as ip address and port", input);
                    }
                }
                "listen" | "l" => {
                    write_prompt("  enter a port:");
                    let addr = read_trimmed(&mut raw_input).await;
                    println!("  listening...");
                    if let Err(e) = daemon.accept_peer(addr).await {
                        println!("could not accept connection: {}", e.to_string());
                        continue;
                    } else {
                        println!("peer added at {}", addr);
                    }
                    let peer: &mut Peer = daemon
                        .conn_man
                        .peers
                        .last_mut()
                        .expect("Should have just added a peeer");
                    if let Err(_) = peer_mode(peer).await {
                        daemon.conn_man.peers.pop();
                    }
                }
                "peer" | "p" => match daemon.conn_man.peers.len() {
                    0 => {
                        write_prompt("  No peers found. Press 'a' to add a peer.");
                        continue;
                    }
                    1 => {
                        write_prompt("  1 peer found. autoselecting...\n");
                        let peer = &mut daemon.conn_man.peers[0];
                        if let Err(_) = peer_mode(peer).await {
                            daemon.conn_man.peers.pop();
                        }
                    }
                    _ => loop {
                        write_prompt(&format!(
                            "  select a peer by id (0...{}) or 'b' to go back:",
                            daemon.conn_man.peers.len() - 1
                        ));
                        let input = read_trimmed(&mut raw_input).await;
                        if let Ok(id) = input.parse::<usize>() {
                            let peer = &mut daemon.conn_man.peers[id];
                            if let Err(_) = peer_mode(peer).await {
                                daemon.conn_man.peers.remove(id);
                            }
                            break;
                        }
                        if input == "b" {
                            break;
                        }
                        continue;
                    },
                },
                _ => continue,
            }
        }
        Ok(())
    }
}
