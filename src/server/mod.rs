pub mod chat_room;
pub mod communication;
pub mod server;

pub use server::Server;

// use std::{
//     collections::HashMap,
//     net::SocketAddr,
//     sync::{Arc, Mutex},
// };
//
// use bincode;
// use tokio::{
//     net::{TcpListener, TcpStream},
//     sync::broadcast,
//     sync::broadcast::Sender, };
//
// use tokio_tungstenite::{
//     accept_async,
//     tungstenite::{self, Message},
//     WebSocketStream,
// };
//
// use futures_util::stream::StreamExt;
// use futures_util::SinkExt;
//
// use crate::server::communication::{
//     client::{
//         ChangeNameRequest, ClientMessage, JoinChatRoomRequest, MakeChatRoomRequest,
//         SendMessageRequest,
//     },
//     server::{JoinChatRoomResponse, ListChatRoomsResponse, NewMessageRequest, ServerMessage},
// };
//
// pub mod communication;
//
// type MyWebSocketStream = WebSocketStream<TcpStream>;
//
//
