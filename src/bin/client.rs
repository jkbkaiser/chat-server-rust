use clap::Parser;
use miette::Result;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use chat_server::client::Client;

/// Terminal chat client implemented in Rust
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
    let client = Client::setup(args.socket_addr).await?;
    client.run().await?;

    Ok(())
}
