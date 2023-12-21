pub mod communication;

use bincode;
use tokio::net::{TcpListener, TcpStream};

use std::sync::Arc;
use std::sync::Mutex;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

// use tokio::spawn;
use tokio_tungstenite::{accept_async, tungstenite, WebSocketStream};

use futures_util::stream::StreamExt;

use crate::server::communication::client::{
    ClientMessage, JoinChatRoomRequest, MakeChatRoomRequest, SendMessageRequest,
};
// use futures_util::stream::{SplitSink, SplitStream};

use std::{collections::HashMap, net::SocketAddr};
// use self::communication::client::{ClientMessage, Message};

type MyWebSocketStream = WebSocketStream<TcpStream>;
// type MyRead = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
// type MyWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    content: String,
}

pub struct ChatRoom {
    // channel/
    send: Sender<ChatMessage>,
    recv: Receiver<ChatMessage>,
}

impl ChatRoom {
    fn new() -> Self {
        let (send, recv) = broadcast::channel(10);

        Self { send, recv }
    }

    fn send_message(&self, message: ChatMessage) {
        self.send.send(message).expect("Could not send message");
    }
}

pub struct ChatRooms {
    rooms: HashMap<String, ChatRoom>,
}

impl ChatRooms {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    fn new_room(&mut self, name: String) {
        let room = ChatRoom::new();
        self.rooms.insert(name, room);
    }

    fn join_room(&self, name: String) -> &ChatRoom {
        self.rooms.get(&name).expect("Could not find room")
    }
}

fn socket_address_to_websocket_url(socket_address: SocketAddr) -> String {
    format!("{}", socket_address.to_string())
}

pub struct Server {
    ip_addr: String,
    rooms: Arc<Mutex<ChatRooms>>,
    // connections: Vec<i32>,
}

impl Server {
    // TODO lib for parsing ips
    pub fn new(ip_addr: SocketAddr) -> Self {
        return Server {
            ip_addr: socket_address_to_websocket_url(ip_addr),
            rooms: Arc::new(Mutex::new(ChatRooms::new())),
        };
    }

    pub async fn run(&self) {
        println!("Starting server");

        let listener = TcpListener::bind(&self.ip_addr).await.unwrap();
        while let Ok((stream, addr)) = listener.accept().await {
            println!("Connection accepted with address: {addr:?}");

            match accept_async(stream).await {
                Ok(ws) => {
                    println!("handshake completed");

                    let t = self.rooms.clone();

                    tokio::spawn(async move { handle_connection(ws, t).await });
                }
                Err(err) => {
                    println!("Failed to connect {err:?}");
                }
            }
        }
    }
}

// Each chatroom has input and ouput channels over wich messages are broadcast
// Each client process then needs to handle sending the actuall messages

// Client processes can either recv from the channel of the currently joined room or the actuall client

async fn handle_connection(mut ws: MyWebSocketStream, rooms: Arc<Mutex<ChatRooms>>) {
    println!("Handeling");

    // let mut send: Option<Sender<ChatMessage>> = None;
    // let mut recv: Option<Receiver<ChatMessage>> = None;

    let (mut send, mut recv) = broadcast::channel(1);


    loop {
        tokio::select! {
            Some(t) = ws.next() => {
                match t {
                    Ok(tungstenite::Message::Binary(msg)) => {
                        let message: ClientMessage =
                            bincode::deserialize(&msg).expect("Failed to deserialize");
    
                        match message {
                            ClientMessage::SendMessage(SendMessageRequest { content }) => {
                                println!("received send message: {}", content);
                                send.send(ChatMessage{ content }).expect("Could not send to chatroom");
                            }
                            ClientMessage::JoinChatRoom(JoinChatRoomRequest { name }) => {
                                println!("received join chat room: {}", name);
                                let rooms = rooms.lock().unwrap();
                                let room = rooms.join_room(name);
                                
                                recv = room.send.subscribe();
                                send = room.send.clone();
                            }
                            ClientMessage::MakeChatRoom(MakeChatRoomRequest { name }) => {
                                println!("received make chat room: {}", name);
                                let mut rooms = rooms.lock().unwrap();
                                rooms.new_room(name);
                            }
                            ClientMessage::ListChatRooms() => {
                                println!("received list chat rooms");
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
            Ok(t) = recv.recv() => {
                println!("Received message from room channel")
            }
        }
    }

    println!("Finished handeling")
}
