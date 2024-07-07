use clap::Parser;
use miette::Result;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use chat_server::server::Server;

/// A chat server written in Rust
#[derive(Parser)]
struct Args {
    /// Server port
    #[arg(
        short,
        long,
        default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080))
    )]
    socket_addr: SocketAddr,
}

/// Entry point
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let server = Server::new(args.socket_addr);
    server.run().await;

    Ok(())
}
