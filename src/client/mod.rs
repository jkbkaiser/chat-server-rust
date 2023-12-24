use bincode;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, select};
use tokio_tungstenite::{
    connect_async, tungstenite, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};

pub mod frontend;

use crate::client::frontend::Frontend;
use crate::server::communication::server::ServerMessage;

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

fn socket_address_to_websocket_url(socket_address: SocketAddr) -> String {
    format!("ws://{}", socket_address.to_string())
}

pub struct Client {
    websocket_url: String,
}

impl Client {
    pub fn new(socket_address: SocketAddr) -> Self {
        return Client {
            websocket_url: socket_address_to_websocket_url(socket_address),
        };
    }

    pub async fn connect_to_server(&self) -> MyWebSocketStream {
        println!("Connecting to server ...");

        match connect_async(&self.websocket_url).await {
            Ok((ws_stream, _)) => {
                println!("Connected to server");
                return ws_stream;
            }
            Err(err) => {
                panic!("Failed to connect to server {err:?}");
            }
        }
    }

    pub async fn run(self) {
        println!("Starting client ...");

        let conn = self.connect_to_server().await;
        let (mut write, mut recv_server_msg) = conn.split();
        let mut frontend = Frontend::new();

        loop {
            select! {
                Some(Ok(tungstenite::Message::Binary(msg))) = recv_server_msg.next() => {
                    let message: ServerMessage =
                        bincode::deserialize(&msg).expect("Failed to deserialize");

                    match message {
                        ServerMessage::NewMessage(m) => {
                            frontend.print_message(m.content, m.user_name);
                        }
                    }

                }
                command = frontend.next_command() => {
                    let binary = bincode::serialize(&command).expect("could not serialize");
                    write.send(Message::Binary(binary)).await.expect("failed to send message");
                },
            }
        }
    }
}
