use bincode;
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::{self, net::TcpStream};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Serialize, Deserialize)]
struct ListChatRoomsCommand {}

#[derive(Debug, Serialize, Deserialize)]
struct MakeChatRoomCommand {
    room_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JoinChatRoomCommand {
    room_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    ListChatRooms(ListChatRoomsCommand),
    JoinChatRoom(JoinChatRoomCommand),
    MakeChatRoom(MakeChatRoomCommand),
}

impl Command {
    fn from_arguments(arguments: Vec<String>) -> Self {
        println!("parsing command: {arguments:?}");
        let mut arguments = arguments.into_iter();

        let command = arguments.next().expect("expected args");

        match &command[..] {
            "make" => Command::MakeChatRoom(MakeChatRoomCommand {
                room_name: arguments.next().expect("expected args"),
            }),
            "list" => Command::ListChatRooms(ListChatRoomsCommand {}),
            "join" => Command::JoinChatRoom(JoinChatRoomCommand {
                room_name: arguments.next().expect("expected args"),
            }),
            // implement help command
            _ => {
                // TODO: handle + print help
                panic!("not a valid commend")
            }
        }
    }
}

#[derive(Debug, Serialize)]
enum ParsedInput {
    Command(Command),
    Text(String),
}

impl ParsedInput {
    fn from_line(line: String) -> Self {
        if line.starts_with("/") {
            let args: Vec<String> = line
                .get(1..)
                .expect("No command was passed")
                .split(" ")
                .map(str::to_string)
                .collect();

            return ParsedInput::Command(Command::from_arguments(args));
        } else {
            return ParsedInput::Text(line);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting client ...");
    println!("Connecting to server ...");

    match connect_async("ws://0.0.0.0:8080").await {
        Ok((ws_stream, _)) => {
            println!("Connected to server");
            run(Handler::new(ws_stream)).await;
        }
        Err(err) => {
            println!("Failed to connect to server {err:?}");
        }
    }
}

async fn run(mut handler: Handler) {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                let input = ParsedInput::from_line(line);
                handler.handle_input(input).await;
            }
            Err(_) => {
                println!("Could not read line");
            }
        }
    }
}

struct Handler {
    ws_stream: MyWebSocketStream,
}

impl Handler {
    fn new(ws_stream: MyWebSocketStream) -> Self {
        Handler { ws_stream }
    }

    async fn handle_input(&mut self, input: ParsedInput) {
        println!("parsed input: {input:?}");

        let t = bincode::serialize(&input).unwrap();
        self.ws_stream.send(Message::Binary(t)).await.unwrap();

        println!("sent message");
    }
}
