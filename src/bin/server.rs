// TODO:
// - Error handling
use clap::Parser;
use chat_server::server::Server;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

#[derive(Parser)]
struct Args {
    // Server port
    #[arg(short, long, default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080)))]
    socket_address: SocketAddr,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let server = Server::new(args.socket_address);
    server.run().await;
}
