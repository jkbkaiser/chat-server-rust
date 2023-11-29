use crate::client::commands::Command;
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type MyRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type MyWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

// TODO
// - better error handeling

pub struct FrontEnd {
    server_ip_addr: &'static str,
}

impl FrontEnd {
    pub fn new(server_ip_addr: &'static str) -> Self {
        println!("Starting client ...");

        return FrontEnd { server_ip_addr };
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
        let conn = self.connect_to_server().await;
        let (_write, mut recv_server_msg) = conn.split();
        let mut user_command_reader = CommandLineReader::new();

        loop {
            select! {
                Some(Ok(m)) = recv_server_msg.next() => handle_server_message(m).await,
                input = user_command_reader.next() => handle_user_command(input).await,
            }
        }
    }
}

struct CommandLineReader {
    reader: tokio::io::BufReader<tokio::io::Stdin>,
}

impl CommandLineReader {
    pub fn new() -> Self {
        let reader: tokio::io::BufReader<tokio::io::Stdin> = BufReader::new(tokio::io::stdin());
        CommandLineReader { reader }
    }

    pub async fn next(&mut self) -> Command {
        let mut buffer = Vec::new();
        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Could not read from input");

        let line = String::from_utf8(buffer).expect("Could not decode input");
        Command::new(line).expect("Could not build command")
    }
}

async fn handle_server_message(_message: Message) {
    println!("Received message from server");
}

async fn handle_user_command(_cmd: Command) {
    println!("Handeling command");
}
