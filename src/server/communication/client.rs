use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    SendMessage(SendMessageRequest),
    MakeChatRoom(ClientMakeChatRoomRequest),
    JoinChatRoom(JoinChatRoomRequest),
    ChangeName(ChangeNameRequest),
    ListChatRooms(),
    Help(),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub content: String,
}

impl SendMessageRequest {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMakeChatRoomRequest {
    pub name: String,
}

impl ClientMakeChatRoomRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinChatRoomRequest {
    pub name: String,
}

impl JoinChatRoomRequest {
    pub fn new(name: String) -> Self {
        JoinChatRoomRequest { name }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeNameRequest {
    pub new_name: String,
}

impl ChangeNameRequest {
    pub fn new(new_name: String) -> Self {
        Self { new_name }
    }
}
