use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server");
    Server::run().await;
    Ok(())
}

struct Server {
    connections: Vec<i32>,
}

impl Server {
    async fn run() {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        while let Ok((stream, addr)) = listener.accept().await {
            println!("Connection accepted with address: {addr:?}");
            handle_tcp_stream(stream).await;
        }
    }
}

async fn handle_tcp_stream(stream: TcpStream) {
    match accept_async(stream).await {
        Ok(_) => {
            println!("handshake completed");
            loop {}
        }
        Err(err) => {
            println!("Failed to connect {err:?}");
        }
    }
}
