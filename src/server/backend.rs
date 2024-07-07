use miette::{miette, Result};
use std::collections::HashMap;
use tokio::sync::broadcast::{self, Sender};

use crate::server::communication::ChatMessage;

/// Contains a chatroom broadcast channel
pub struct ChatRoom {
    /// Chatroom broadcast channel
    send: Sender<ChatMessage>,
}

impl ChatRoom {
    /// Creates a new chatroom
    pub fn new() -> Self {
        let (send, _) = broadcast::channel(10);
        Self { send }
    }

    /// Subscribe to a chatroom
    pub fn subscribe(&self, name: &str) -> broadcast::Receiver<ChatMessage> {
        let _ = self.send.send(ChatMessage {
            sender_uuid: "".to_string(),
            sender_name: "ChatRoom".to_string(),
            content: format!("User {name} joined the room"),
        });
        self.send.subscribe()
    }

    /// Publish to a chatroom
    pub fn publish(&self) -> broadcast::Sender<ChatMessage> {
        self.send.clone()
    }
}

/// Datastructure that keep tracks of all chatrooms
pub struct Backend {
    rooms: HashMap<String, ChatRoom>,
}

impl Backend {
    /// Crates a new backend
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    /// Creates a new chatroom
    pub fn new_room(&mut self, name: String) -> Result<()> {
        match self.rooms.get(&name) {
            Some(_) => Err(miette!("Room already exists")),
            None => {
                let room = ChatRoom::new();
                self.rooms.insert(name, room);
                Ok(())
            }
        }
    }

    /// Returns a requested chatroom
    pub fn get_room(&self, name: String) -> Result<&ChatRoom> {
        let room = self
            .rooms
            .get(&name)
            .ok_or(miette!("Could not find room"))?;

        return Ok(room);
    }

    /// Lists the names of all chatrooms
    pub fn list(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }
}
