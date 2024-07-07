pub mod client;
pub mod server;

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub content: String,
    pub sender_name: String,
    pub sender_uuid: String,
}
