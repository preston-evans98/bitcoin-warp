use daemon::Daemon;

fn main() {
    let daemon = Daemon::new();
    println!("{:?}", daemon);
}
