// TODO:
// - Error handling
use chat_server::client::Client;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

#[derive(Parser)]
struct Args {
    // Server port
    #[arg(short, long, default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080)))]
    socket_address: SocketAddr,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = Client::new(args.socket_address);
    client.run().await;
}
