use tokio::net::{TcpListener, TcpStream};
// use tokio::spawn;
use tokio_tungstenite::accept_async;

pub struct Server {
    server_ip_addr: &'static str,
    // connections: Vec<i32>,
}

impl Server {
    // TODO lib for parsing ips
    pub fn new(server_ip_addr: &'static str) -> Self {
        return Server {
            server_ip_addr,
            // connections: Vec::new(),
        };
    }

    pub async fn run(&self) {
        println!("Starting server");

        let listener = TcpListener::bind(self.server_ip_addr).await.unwrap();
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
