use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    NewMessage(NewMessageRequest),
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
