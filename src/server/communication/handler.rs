use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HandlerMessage {
    MakeChatRoom(HandlerMakeChatRoomRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HandlerMakeChatRoomRequest {
    pub name: String,
}
