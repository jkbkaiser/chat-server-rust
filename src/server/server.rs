use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};
use miette::{miette, IntoDiagnostic, Result};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, Mutex},
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use uuid::Uuid;

use crate::server::{
    backend::Backend,
    communication::{
        client::{
            ChangeNameRequest, ClientMakeChatRoomRequest, ClientMessage, JoinChatRoomRequest,
            SendMessageRequest,
        },
        server::{JoinChatRoomResponse, ListChatRoomsResponse, NewMessageRequest, ServerMessage},
        ChatMessage,
    },
};

/// Deserializes a msg from the client into a [`ClientMessage`]
fn deserialize_client_msg(msg: Message) -> Result<ClientMessage> {
    match msg {
        Message::Binary(bytes) => {
            let message: ClientMessage = bincode::deserialize(&bytes).into_diagnostic()?;

            Ok(message)
        }
        _ => Err(miette!("Received an invalid websocket message encoding")),
    }
}

/// Serializes a [`ServerMessage`] into a tungestenite message
fn serialize_server_msg(msg: ServerMessage) -> Result<Message> {
    let client_msg = bincode::serialize(&msg).into_diagnostic()?;
    let serialized_msg = Message::Binary(client_msg);
    Ok(serialized_msg)
}

/// Contains the logic for running the server
pub struct Server {
    /// The socket the server listens on
    socket_addr: SocketAddr,
    /// Datastructure to keep track of all backrooms, etc.
    backend: Arc<Mutex<Backend>>,
}

impl Server {
    /// Instantiates a new server that listens on the given socket
    pub fn new(socket_addr: SocketAddr) -> Self {
        return Server {
            socket_addr,
            backend: Arc::new(Mutex::new(Backend::new())),
        };
    }

    /// Starts the server and handles connection to the socket
    pub async fn run(&self) {
        println!("Starting server");

        // Instatiate listener for incoming connections
        let listener = TcpListener::bind(self.socket_addr).await.unwrap();

        while let Ok((conn, addr)) = listener.accept().await {
            println!("Connection accepted with address: {addr:?}");

            // Setup websocket connection and handler
            match accept_async(conn).await {
                Ok(ws) => {
                    let b = self.backend.clone();
                    let uuid = Uuid::new_v4();
                    let handler = Handler::new(uuid, ws, b);

                    tokio::spawn(async move { handler.run().await });
                }
                Err(err) => {
                    println!("Failed to connect {err:?}");
                }
            }
        }
    }
}

/// Handler for a single client
pub struct Handler {
    /// ID for the client handled by this handler
    uuid: Uuid,
    /// User name
    name: String,
    /// Backend that keeps track of all chatrooms etc
    backend: Arc<Mutex<Backend>>,
    /// Websocket sender
    ws_send: SplitSink<WebSocketStream<TcpStream>, Message>,
    /// Websocket receiver
    ws_recv: SplitStream<WebSocketStream<TcpStream>>,
    /// Channel to send a message into a chat room
    room_send: broadcast::Sender<ChatMessage>,
    /// Channel to receive messages from a chat room
    room_recv: broadcast::Receiver<ChatMessage>,
}

impl Handler {
    /// Instantiates a new handler
    pub fn new(uuid: Uuid, ws: WebSocketStream<TcpStream>, backend: Arc<Mutex<Backend>>) -> Self {
        let (ws_send, ws_recv) = ws.split();
        let (room_send, room_recv) = broadcast::channel(1);
        let name = "anonymous".to_string();

        Handler {
            uuid,
            name,
            backend,
            ws_send,
            ws_recv,
            room_send,
            room_recv,
        }
    }

    /// Handles messages from the client
    async fn handle_client_msg(&mut self, msg: Message) -> Result<()> {
        let message = deserialize_client_msg(msg)?;

        match message {
            ClientMessage::MakeChatRoom(ClientMakeChatRoomRequest { name }) => {
                let mut backend = self.backend.lock().await;
                backend.new_room(name);
            }
            ClientMessage::ListChatRooms() => {
                let backend = self.backend.lock().await;
                let rooms = backend.list();
                let server_msg = ServerMessage::ListChatRooms(ListChatRoomsResponse::new(rooms));
                self.ws_send
                    .send(serialize_server_msg(server_msg)?)
                    .await
                    .into_diagnostic()?;
            }
            ClientMessage::JoinChatRoom(JoinChatRoomRequest { name }) => {
                let backend = self.backend.lock().await;
                let room = backend.get_room(name.clone())?;
                self.room_recv = room.subscribe();
                self.room_send = room.publish();

                let server_msg = ServerMessage::JoinedChatRoom(JoinChatRoomResponse::new(name));
                self.ws_send
                    .send(serialize_server_msg(server_msg)?)
                    .await
                    .expect("failed send message to client");
            }
            ClientMessage::SendMessage(SendMessageRequest { content }) => {
                self.room_send
                    .send(ChatMessage {
                        sender_uuid: self.uuid.to_string(),
                        sender_name: self.name.clone(),
                        content,
                    })
                    .expect("Could not send to chatroom");
            }
            ClientMessage::ChangeName(ChangeNameRequest { new_name }) => {
                self.name = new_name;
            }
            _ => {}
        };

        Ok(())
    }

    /// Handles messages from the connected chat room
    async fn handle_room_msg(&mut self, msg: ChatMessage) -> Result<()> {
        if msg.sender_uuid != self.uuid.to_string() {
            let server_msg =
                ServerMessage::NewMessage(NewMessageRequest::new(msg.content, msg.sender_name));
            self.ws_send
                .send(serialize_server_msg(server_msg)?)
                .await
                .into_diagnostic()?;
        }

        Ok(())
    }

    /// Starts a handler
    async fn run(mut self) -> Result<()> {
        println!("Started handler {}", self.uuid);

        loop {
            tokio::select! {
                Some(Ok(msg)) = self.ws_recv.next() => {
                    self.handle_client_msg(msg).await?;
                },
                Ok(msg) = self.room_recv.recv() => {
                    self.handle_room_msg(msg).await?;
                }
            }
        }
    }
}
