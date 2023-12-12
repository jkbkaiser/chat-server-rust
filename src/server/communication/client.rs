use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    SendMessage(Message),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        Message { content }
    }
}
