use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    NewMessage(NewMessageRequest),
    JoinedChatRoom(JoinChatRoomResponse),
    ListChatRooms(ListChatRoomsResponse),
}

#[derive(Serialize, Deserialize)]
pub struct NewMessageRequest {
    pub content: String,
    pub user_name: String,
}

impl NewMessageRequest {
    pub fn new(content: String, user_name: String) -> Self {
        Self { content, user_name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JoinChatRoomResponse {
    pub name: String,
}

impl JoinChatRoomResponse {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ListChatRoomsResponse {
    pub names: Vec<String>,
}

impl ListChatRoomsResponse {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }
}
