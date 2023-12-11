use serde::{Deserialize, Serialize};
use std::error;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

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
                panic!("not a valid commend");
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

pub struct Frontend {
    reader: BufReader<Stdin>,
}

impl Frontend {
    pub fn new() -> Self {
        let reader: BufReader<Stdin> = BufReader::new(tokio::io::stdin());
        Frontend { reader }
    }

    pub async fn next(&mut self) -> Command {
        let mut buffer = Vec::new();
        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Could not read from input");

        let line = String::from_utf8(buffer).expect("Could not decode input");
        Command::new(line).expect("Could not build command")
    }
}
