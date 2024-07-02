// Current idea:
// - User only connected to a single chat room
// - Single central service that handles clients
//   - Connects them to rooms
//   - Handles usernames
//   - Creates new rooms
//   - List of chatrooms
// - Each room handles messages sent to that room
//
// TODO
// - Implement
// - Document
// - Test in local network
use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use miette::{miette, IntoDiagnostic, Result};
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use uuid::Uuid;

use super::communication::{handler::{HandlerMessage, HandlerMakeChatRoomRequest}, client::{ClientMakeChatRoomRequest, ClientMessage}};

fn deserialize_client_msg(msg: Message) -> Result<ClientMessage> {
    match msg {
        Message::Binary(bytes) => {
            let message: ClientMessage = bincode::deserialize(&bytes).into_diagnostic()?;

            Ok(message)
        }
        _ => Err(miette!("Received an invalid websocket message encoding")),
    }
}


/// Contains the logic for running the server
pub struct Server {
    /// The socket the server listens on
    socket_addr: SocketAddr,
    // rooms: Arc<Mutex<ChatRooms>>,
}

impl Server {
    /// Instantiates a new server that listens on the given socket
    pub fn new(socket_addr: SocketAddr) -> Self {
        return Server { socket_addr };
    }

    /// Starts the server and handles connection to the socket
    pub async fn run(&self) {
        println!("Starting server");

        // Start the backend that has control over storage
        let (backend, handle_prod) = Backend::new();
        tokio::spawn(async move { backend.run().await });

        // Instatiate listener for incoming connections
        let listener = TcpListener::bind(self.socket_addr).await.unwrap();

        while let Ok((conn, addr)) = listener.accept().await {
            println!("Connection accepted with address: {addr:?}");

            // Setup websocket connection and handler
            match accept_async(conn).await {
                Ok(ws) => {
                    let uuid = Uuid::new_v4();
                    let hp = handle_prod.clone();
                    let handler = Handler::new(uuid, hp, ws);
                    tokio::spawn(async move { handler.run().await });
                }
                Err(err) => {
                    println!("Failed to connect {err:?}");
                }
            }
        }
    }
}

/// Controls storage
pub struct Backend {
    /// Channel for recieving messages from handler
    handle_cons: Receiver<HandlerMessage>,
}

impl Backend {
    /// Instantiates a new backend that owns the storage
    pub fn new() -> (Self, Sender<HandlerMessage>) {
        let (handle_prod, handle_cons) = mpsc::channel(32);
        return (Backend { handle_cons }, handle_prod);
    }

    /// Starts the backend
    pub async fn run(&self) {
        println!("Backend up");
    }
}

/// Handler for a single client
pub struct Handler {
    /// ID for the client handled by this handler
    uuid: Uuid,
    /// Channel for commincating with the backend
    handle_prod: Sender<HandlerMessage>,
    /// Websocket sender
    ws_send: SplitSink<WebSocketStream<TcpStream>, Message>,
    /// Websocket receiver
    ws_recv: SplitStream<WebSocketStream<TcpStream>>,
}

impl Handler {
    /// Instantiates a new handler
    pub fn new(
        uuid: Uuid,
        handle_prod: Sender<HandlerMessage>,
        ws: WebSocketStream<TcpStream>,
    ) -> Self {
        let (ws_send, ws_recv) = ws.split();

        Handler {
            uuid,
            handle_prod,
            ws_send,
            ws_recv,
        }
    }

    async fn handle_client_msg(&self, msg: Message) -> Result<()> {
        let message = deserialize_client_msg(msg)?;
        println!("{message:?}");

        match message {
            ClientMessage::MakeChatRoom(ClientMakeChatRoomRequest { name }) => {
                self.handle_prod.send(Message)
                // let mut rooms = rooms.lock().unwrap();
                // rooms.new_room(name);
                Ok(())
            }
            _ => Err(miette!("Not implemented")),
        }
    }

    async fn run(mut self) {
        println!("Started handler {}", self.uuid);

        loop {
            tokio::select! {
                Some(Ok(msg)) = self.ws_recv.next() => {
                    self.handle_client_msg(msg).await;
                    // handle_client_msg(t, rooms, send, recv, ws_send);
                },
                // Ok(m) = recv.recv() => {
                //     if m.client_id != client_id {
                //         let client_msg = ServerMessage::NewMessage(NewMessageRequest::new(m.content, m.client_name));
                //         let client_msg = bincode::serialize(&client_msg).expect("Could not serialize msg");
                //         ws_send.send(tungstenite::Message::Binary(client_msg)).await.expect("failed send message to client");
                //     }
                // }
            }
        }
    }
}

// Each chatroom has input and ouput channels over wich messages are broadcast
// Each client process then needs to handle sending the actuall messages
// Client processes can either recv from the channel of the currently joined room or the actuall client
// async fn handle_connection(ws: MyWebSocketStream, rooms: Arc<Mutex<ChatRooms>>, client_id: i32) {
//     let (mut ws_send, mut ws_recv) = ws.split();
//     let mut client_name = format!("User{client_id}");
//     let (mut send, mut recv) = broadcast::channel(1);
//
//     loop {
//         tokio::select! {
//             Some(Ok(t)) = ws_recv.next() => {
//                 handle_client_msg(t, rooms, send, recv, ws_send);
//             }
//             Ok(m) = recv.recv() => {
//                 if m.client_id != client_id {
//                     let client_msg = ServerMessage::NewMessage(NewMessageRequest::new(m.content, m.client_name));
//                     let client_msg = bincode::serialize(&client_msg).expect("Could not serialize msg");
//                     ws_send.send(tungstenite::Message::Binary(client_msg)).await.expect("failed send message to client");
//                 }
//             }
//         }
//     }
// }

// async fn handle_client_msg(
//     msg: Message,
//     rooms: Arc<Mutex<ChatRooms>>,
//     send: tokio::sync::broadcast::Sender<ChatMessage>,
//     recv: tokio::sync::broadcast::Receiver<JoinChatRoomResp>,
//     ws_send: futures::stream::SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>,
// ) {
//     match msg {
//         tungstenite::Message::Binary(msg) => {
//             let message: ClientMessage = bincode::deserialize(&msg).expect("Failed to deserialize");
//
//             match message {
//                 ClientMessage::SendMessage(SendMessageRequest { content }) => {
//                     send.send(ChatMessage {
//                         client_id,
//                         client_name: client_name.clone(),
//                         content,
//                     })
//                     .expect("Could not send to chatroom");
//                 }
//                 ClientMessage::JoinChatRoom(JoinChatRoomRequest { name }) => {
//                     {
//                         let rooms = rooms.lock().unwrap();
//                         let room = rooms.join_room(name.clone());
//                         recv = room.send.subscribe();
//                         send = room.send.clone();
//                     }
//
//                     let client_msg =
//                         ServerMessage::JoinedChatRoom(JoinChatRoomResponse::new(name.clone()));
//                     let client_msg =
//                         bincode::serialize(&client_msg).expect("Could not serialize msg");
//                     ws_send
//                         .send(tungstenite::Message::Binary(client_msg))
//                         .await
//                         .expect("failed send message to client");
//                 }
//                 ClientMessage::MakeChatRoom(MakeChatRoomRequest { name }) => {
//                     let mut rooms = rooms.lock().unwrap();
//                     rooms.new_room(name);
//                 }
//                 ClientMessage::ChangeName(ChangeNameRequest { new_name }) => {
//                     client_name = new_name;
//                 }
//                 ClientMessage::ListChatRooms() => {
//                     let r = rooms.lock().unwrap().list();
//                     let client_msg = ServerMessage::ListChatRooms(ListChatRoomsResponse::new(r));
//                     let client_msg =
//                         bincode::serialize(&client_msg).expect("Could not serialize msg");
//                     ws_send
//                         .send(tungstenite::Message::Binary(client_msg))
//                         .await
//                         .expect("failed send message to client");
//                 }
//                 ClientMessage::Help() => {}
//             }
//         }
//         Ok(_) => {
//             println!("Recv different type msg");
//         }
//         Err(e) => {
//             println!("Something went wrong receiving {}", e);
//         }
//     }
// }
