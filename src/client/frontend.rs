use crate::server::communication::client::{ClientMessage, JoinChatRoomRequest, MakeChatRoomRequest, SendMessageRequest};
use std::error;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

pub type Command = ClientMessage;

impl Command {
    pub fn from_arguments(arguments: Vec<String>) -> Result<Command, Box<dyn error::Error>> {
        println!("parsing command: {arguments:?}");

        let mut arguments = arguments.into_iter();

        let keyword = arguments.next().expect("expected args");

        let command = match &keyword[..] {
            "make" => Command::MakeChatRoom(MakeChatRoomRequest {
                name: arguments
                    .next()
                    .ok_or_else(|| "make chat room not enough args")?,
            }),
            "join" => Command::JoinChatRoom(JoinChatRoomRequest {
                name: arguments
                    .next()
                    .ok_or_else(|| "make join chat room not enough args")?,
            }),
            "list" => Command::ListChatRooms(),
            _ => {
                panic!("not a valid commend");
            }
        };

        Ok(command)
    }

    pub fn new(line: String) -> Result<Self, Box<dyn error::Error>> {
        if line.starts_with("/") {
            let args: Vec<String> = line
                .trim()
                .get(1..)
                .ok_or("test")?
                .split(" ")
                .map(str::to_string)
                .collect();

            return Ok(Command::from_arguments(args)?);
        } else {
            return Ok(Command::SendMessage(SendMessageRequest::new(line)));
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

    pub async fn next_command(&mut self) -> Command {
        let mut buffer = Vec::new();
        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Could not read from input");

        let line = String::from_utf8(buffer).expect("Could not decode input");
        Command::new(line).expect("Could not build command")
    }
}
