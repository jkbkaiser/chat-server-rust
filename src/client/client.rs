use bincode::{deserialize, serialize};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use miette::{miette, IntoDiagnostic, Result};
use std::net::SocketAddr;
use tokio::{net::TcpStream, select};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::client::frontend::{Command, Frontend};
use crate::server::communication::server::ServerMessage;

/// Websocket shorthand
type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Write part of the websocket shorthand
type WebSocketWrite = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Receiving part of the websocket shorthand
type WebSocketRecv = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// Connects client to server
async fn connect_to_server(socket_addr: SocketAddr) -> Result<WebSocket> {
    println!("Connecting to server ...");

    match connect_async(&format!("ws://{}", socket_addr)).await {
        Ok((ws, _)) => {
            println!("Connected to server");
            Ok(ws)
        }
        Err(err) => Err(miette!("Failed to connect to server {err:?}")),
    }
}

/// Client that connects to the server
pub struct Client {
    /// Frontend that reads and prints to terminal
    frontend: Frontend,
    /// For writing messages to the server
    write: WebSocketWrite,
    /// For receiving messages from the server
    recv: WebSocketRecv,
}

impl Client {
    /// Instantiates a new client
    pub async fn setup(socket_addr: SocketAddr) -> Result<Self> {
        println!("Setting up client...");

        let conn = connect_to_server(socket_addr).await?;
        let (write, recv) = conn.split();
        let frontend = Frontend::new()?;

        let client = Client {
            frontend,
            write,
            recv,
        };

        Ok(client)
    }

    /// Handles messages from the server
    fn handle_server_msg(&mut self, msg: Vec<u8>) -> Result<()> {
        let message: ServerMessage = deserialize(&msg).into_diagnostic()?;

        match message {
            ServerMessage::NewMessage(m) => {
                self.frontend.print_message(m.content, m.user_name)?;
            }
            ServerMessage::JoinedChatRoom(m) => {
                self.frontend.current_chatroom = m.name;
                self.frontend.print_prompt()?;
            }
            ServerMessage::ListChatRooms(m) => {
                self.frontend.print_rooms(m.names)?;
                self.frontend.print_prompt()?;
            }
            ServerMessage::Err(error) => {
                self.frontend.print_err(&error)?;
            }
        }

        Ok(())
    }

    /// Sends a command to the server
    async fn send_cmd(&mut self, cmd: Command) -> Result<()> {
        let binary = serialize(&cmd).into_diagnostic()?;
        self.write
            .send(Message::Binary(binary))
            .await
            .into_diagnostic()?;

        Ok(())
    }

    /// Handles user commands
    async fn handle_user_cmd(&mut self, cmd: Command) -> Result<()> {
        match cmd {
            Command::Help() => self.frontend.print_help()?,
            Command::JoinChatRoom(_) => {
                self.send_cmd(cmd).await?;
                self.frontend.current_chatroom = String::from("Connecting...");
            }
            _ => {
                self.send_cmd(cmd).await?;
            }
        }

        Ok(())
    }

    /// Starts the client, handles commands and server messages
    pub async fn run(mut self) -> Result<()> {
        loop {
            select! {
                Some(Ok(Message::Binary(msg))) = self.recv.next() => {
                    self.handle_server_msg(msg)?;
                }
                Ok(Some(cmd)) = self.frontend.next() => {
                    self.handle_user_cmd(cmd).await?;
                },
            }
        }
    }
}
