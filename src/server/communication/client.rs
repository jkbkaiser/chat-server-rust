use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    SendMessage(SendMessageRequest),
    MakeChatRoom(MakeChatRoomRequest),
    JoinChatRoom(JoinChatRoomRequest),
    ListChatRooms(),

}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

impl SendMessageRequest {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}


#[derive(Serialize, Deserialize)]
pub struct MakeChatRoomRequest {
    pub name: String,
}

impl MakeChatRoomRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}


#[derive(Serialize, Deserialize)]
pub struct JoinChatRoomRequest {
    pub name: String,
}

impl JoinChatRoomRequest {
    pub fn new(name: String) -> Self {
        JoinChatRoomRequest { name }
    }
}