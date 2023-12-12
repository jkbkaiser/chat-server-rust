pub mod communication;

use bincode;
use tokio::net::{TcpListener, TcpStream};
// use tokio::spawn;
use tokio_tungstenite::{accept_async, tungstenite, WebSocketStream};

use futures_util::stream::StreamExt;

use crate::server::communication::client::{ClientMessage, Message};
// use futures_util::stream::{SplitSink, SplitStream};

// use self::communication::client::{ClientMessage, Message};

type MyWebSocketStream = WebSocketStream<TcpStream>;
// type MyRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
// type MyWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

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

            match accept_async(stream).await {
                Ok(ws) => {
                    println!("handshake completed");

                    tokio::spawn(async move { handle_connection(ws).await });
                }
                Err(err) => {
                    println!("Failed to connect {err:?}");
                }
            }
        }
    }
}

async fn handle_connection(mut ws: MyWebSocketStream) {
    println!("Handeling");

    while let Some(t) = ws.next().await {
        match t {
            Ok(tungstenite::Message::Binary(msg)) => {
                let message: ClientMessage =
                    bincode::deserialize(&msg).expect("Failed to deserialize");

                match message {
                    ClientMessage::SendMessage(Message { content }) => {
                        println!("received {}", content);
                    }
                }
            }
            Ok(_) => {
                println!("Recv different type msg");
            }
            Err(e) => {
                println!("Something went wrong receiving {}", e);
            }
        }
    }

    println!("Finished handeling")
}
