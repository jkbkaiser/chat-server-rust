use serde::{Deserialize, Serialize};
use std::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListChatRoomsCommand {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MakeChatRoomCommand {
    pub room_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinChatRoomCommand {
    pub room_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    ListChatRooms(ListChatRoomsCommand),
    JoinChatRoom(JoinChatRoomCommand),
    MakeChatRoom(MakeChatRoomCommand),
    Message(String),
}

impl Command {
    pub fn from_arguments(arguments: Vec<String>) -> Result<Command, Box<dyn error::Error>> {
        println!("parsing command: {arguments:?}");
        let mut arguments = arguments.into_iter();

        let keyword = arguments.next().expect("expected args");

        let command = match &keyword[..] {
            "make" => Command::MakeChatRoom(MakeChatRoomCommand {
                room_name: arguments
                    .next()
                    .ok_or_else(|| "make chat room not enough args")?,
            }),
            "list" => Command::ListChatRooms(ListChatRoomsCommand {}),
            "join" => Command::JoinChatRoom(JoinChatRoomCommand {
                room_name: arguments
                    .next()
                    .ok_or_else(|| "make join chat room not enough args")?,
            }),
            _ => {
                panic!("not a valid commend")
            }
        };

        Ok(command)
    }

    pub fn new(line: String) -> Result<Self, Box<dyn error::Error>> {
        if line.starts_with("/") {
            let args: Vec<String> = line
                .get(1..)
                .ok_or("test")?
                .split(" ")
                .map(str::to_string)
                .collect();

            return Ok(Command::from_arguments(args)?);
        } else {
            return Ok(Command::Message(line));
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ListChatRoomsCommand {}
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct MakeChatRoomCommand {
//     pub room_name: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct JoinChatRoomCommand {
//     pub room_name: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub enum Command {
//     ListChatRooms(ListChatRoomsCommand),
//     JoinChatRoom(JoinChatRoomCommand),
//     MakeChatRoom(MakeChatRoomCommand),
// }
//
// #[derive(Debug, Serialize)]
// pub enum UserInput {
//     Command(Command),
//     Message(String),
// }
//
// impl UserInput {
//     // TODO:
//     // - implement help command
//     // - better error handeling
//     pub fn from_line(line: String) -> Result<Self, Box<dyn error::Error>> {
//         if line.starts_with("/") {
//             let args: Vec<String> = line
//                 .get(1..)
//                 .ok_or("test")?
//                 .split(" ")
//                 .map(str::to_string)
//                 .collect();
//
//             return Ok(UserInput::Command(Command::from_arguments(args)?));
//         } else {
//             return Ok(UserInput::Message(line));
//         }
//     }
//
//     pub fn to_bin(&self) -> Result<Vec<u8>, Box<dyn error::Error>> {
//         Ok(bincode::serialize(&self)?)
//     }
// }
//
// impl Command {
//     // TODO:
//     // - implement help command
//     // - better error handeling
//     pub fn from_arguments(arguments: Vec<String>) -> Result<Command, Box<dyn error::Error>> {
//         println!("parsing command: {arguments:?}");
//         let mut arguments = arguments.into_iter();
//
//         let keyword = arguments.next().expect("expected args");
//
//         let command = match &keyword[..] {
//             "make" => Command::MakeChatRoom(MakeChatRoomCommand {
//                 room_name: arguments
//                     .next()
//                     .ok_or_else(|| "make chat room not enough args")?,
//             }),
//             "list" => Command::ListChatRooms(ListChatRoomsCommand {}),
//             "join" => Command::JoinChatRoom(JoinChatRoomCommand {
//                 room_name: arguments
//                     .next()
//                     .ok_or_else(|| "make join chat room not enough args")?,
//             }),
//             _ => {
//                 panic!("not a valid commend")
//             }
//         };
//
//         Ok(command)
//     }
// }
