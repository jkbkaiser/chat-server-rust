use bincode;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, select};
use tokio_tungstenite::{
    connect_async, tungstenite, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};

pub mod frontend;

use crate::client::frontend::{Command, Frontend};
use crate::server::communication::server::ServerMessage;

type MyWebSocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

fn socket_address_to_websocket_url(socket_address: SocketAddr) -> String {
    format!("ws://{}", socket_address)
}

pub struct Client {
    websocket_url: String,
}

impl Client {
    pub fn new(socket_address: SocketAddr) -> Self {
        Client {
            websocket_url: socket_address_to_websocket_url(socket_address),
        }
    }

    pub async fn connect_to_server(&self) -> MyWebSocketStream {
        println!("Connecting to server ...");

        match connect_async(&self.websocket_url).await {
            Ok((ws_stream, _)) => {
                println!("Connected to server");
                ws_stream
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
                        ServerMessage::JoinedChatRoom(m) => {
                            frontend.current_chatroom = m.name;
                            frontend.print_prompt();
                        }
                        ServerMessage::ListChatRooms(m) => {
                            frontend.print_rooms(m.names);
                            frontend.print_prompt();
                        }
                    }

                }
                command = frontend.next_command() => {
                    match command {
                        Some(Command::JoinChatRoom(_)) => {
                            let t = command.unwrap();
                            let binary = bincode::serialize(&t).expect("could not serialize");
                            write.send(Message::Binary(binary)).await.expect("failed to send message");
                            frontend.current_chatroom = String::from("Connecting...");
                            // println!("Sending join chat room")
                        },
                        Some(Command::MakeChatRoom(_)) => {
                            let t = command.unwrap();
                            let binary = bincode::serialize(&t).expect("could not serialize");
                            write.send(Message::Binary(binary)).await.expect("failed to send message");
                            // println!("Sending make chat room")
                        },
                        Some(Command::ListChatRooms()) => {
                            let t = command.unwrap();
                            let binary = bincode::serialize(&t).expect("could not serialize");
                            write.send(Message::Binary(binary)).await.expect("failed to send message");
                            // println!("Sending list chat rooms")
                        }
                        Some(Command::ChangeName(_)) => {
                            let t = command.unwrap();
                            let binary = bincode::serialize(&t).expect("could not serialize");
                            write.send(Message::Binary(binary)).await.expect("failed to send message");
                            // println!("Sending Change name")
                        }
                        Some(Command::SendMessage(_)) => {
                            let t = command.unwrap();
                            let binary = bincode::serialize(&t).expect("could not serialize");
                            write.send(Message::Binary(binary)).await.expect("failed to send message");
                            // println!("Sending send message")
                        }
                        Some(Command::Help()) => {
                            frontend.print_help()
                        }
                        None => {
                            // println!("Not sending anything")
                        }
                        // _ => {
                        //     let binary = bincode::serialize(&command).expect("could not serialize");
                        //     write.send(Message::Binary(binary)).await.expect("failed to send message");
                        // }
                    }
                },
            }
        }
    }
}
