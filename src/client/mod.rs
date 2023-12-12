pub mod frontend;
use std::net::SocketAddr;

use crate::client::frontend::Frontend;
use bincode;
use futures_util::{
    // stream::{SplitSink, SplitStream},
    SinkExt,
    StreamExt,
};

use tokio::net::TcpStream;
use tokio::select;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
// type MyRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
// type MyWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

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
                Some(Ok(m)) = recv_server_msg.next() => handle_server_message(m).await,
                command = frontend.next_command() => {
                    println!("Handeling command");

                    let binary = bincode::serialize(&command).expect("could not serialize");
                    // let message: SendMessage =
                    //     bincode::deserialize(&binary).expect("Failed to deserialize");

                    write.send(Message::Binary(binary)).await.expect("failed to send message");
                },
            }
        }
    }
}

async fn handle_server_message(_message: Message) {
    println!("Received message from server");
}
