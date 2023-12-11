pub mod frontend;

use crate::client::frontend::{Command, CommandLineReader};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use tokio::net::TcpStream;
use tokio::select;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type MyRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type MyWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct Client {
    server_ip_addr: &'static str,
}

impl Client {
    pub fn new(server_ip_addr: &'static str) -> Self {
        return Client { server_ip_addr };
    }

    pub async fn connect_to_server(&self) -> MyWebSocketStream {
        println!("Connecting to server ...");

        match connect_async(self.server_ip_addr).await {
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
        let (_write, mut recv_server_msg) = conn.split();
        let mut user_command_reader = CommandLineReader::new();

        loop {
            select! {
                Some(Ok(m)) = recv_server_msg.next() => handle_server_message(m).await,
                input = user_command_reader.next() => {
                    println!("Handeling command");
                },
            }
        }
    }
}

async fn handle_server_message(_message: Message) {
    println!("Received message from server");
}
