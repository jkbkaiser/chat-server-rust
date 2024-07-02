// #[derive(Clone, Debug)]
// pub struct ChatMessage {
//     content: String,
//     client_name: String,
//     client_id: i32,
// }
//
// pub struct ChatRoom {
//     send: Sender<ChatMessage>,
// }
//
// impl ChatRoom {
//     fn new() -> Self {
//         let (send, _) = broadcast::channel(10);
//         Self { send }
//     }
// }
//
// pub struct ChatRooms {
//     rooms: HashMap<String, ChatRoom>,
// }
//
// impl ChatRooms {
//     fn new() -> Self {
//         Self {
//             rooms: HashMap::new(),
//         }
//     }
//
//     fn new_room(&mut self, name: String) {
//         let room = ChatRoom::new();
//         self.rooms.insert(name, room);
//     }
//
//     fn join_room(&self, name: String) -> &ChatRoom {
//         self.rooms.get(&name).expect("Could not find room")
//     }
//
//     fn list(&self) -> Vec<String> {
//         self.rooms.keys().cloned().collect()
//     }
// }
