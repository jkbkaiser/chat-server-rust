use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server ...");
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Connection accepted with address: {addr:?}");
        spawn(async { handle_tcp_stream(stream).await });
    }

    Ok(())
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
