use std::{error, io, io::Write, io::stdout};
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};
use crossterm::{cursor, ExecutableCommand, terminal};
use std::process;

use crate::server::communication::client::{
    ClientMessage, JoinChatRoomRequest, MakeChatRoomRequest, SendMessageRequest,
};

pub type Command = ClientMessage;

impl Command {
    pub fn from_arguments(arguments: Vec<String>) -> Result<Command, Box<dyn error::Error>> {
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
            "exit" => {process::exit(0)},
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
            return Ok(Command::SendMessage(SendMessageRequest::new(line.trim().to_string())));
        }
    }
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}


fn is_command(line: String) -> bool {
    line.starts_with("/")
}

pub struct Frontend {
    reader: BufReader<Stdin>,
    current_chatroom: String,
}

impl Frontend {
    pub fn new() -> Self {
        let reader: BufReader<Stdin> = BufReader::new(tokio::io::stdin());
        let f =Frontend { reader, current_chatroom: String::from("None") };
        f.print_prompt();
        f
    }

    pub fn print_prompt(&self) {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveToColumn(0)).expect("failed to move cursor up");
        stdout.execute(cursor::MoveUp(2)).expect("failed to move cursor left");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Could not clear");
        print!("------------------------\n(room: {})\n⤷ ", self.current_chatroom);
        let _ = io::stdout().flush();
    }

    pub fn print_message(&self, msg: String) {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveUp(2)).expect("failed to move cursor up");
        stdout.execute(cursor::MoveToColumn(0)).expect("failed to move cursor left");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Could not clear");
        print!("UnknownUser: {msg}\n-------------------------\n(room: {})\n⤷ ", self.current_chatroom);
        let _ = io::stdout().flush();
    }

    pub fn print_command(&self, msg: String) {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveUp(3)).expect("failed to move cursor up");
        stdout.execute(cursor::MoveToColumn(0)).expect("failed to move cursor left");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Could not clear");
        print!("-> {}\n-------------------------\n(room: {})\n⤷ ", crop_letters(&msg, 1), self.current_chatroom);
        let _ = io::stdout().flush();
    }

    pub fn print_input(&self, inp: String) {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveUp(3)).expect("failed to move cursor up");
        stdout.execute(cursor::MoveToColumn(0)).expect("failed to move cursor left");
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Could not clear");
        print!("You: {inp}\n-------------------------\n(room: {})\n⤷ ", self.current_chatroom);
        let _ = io::stdout().flush();
    }

    pub async fn next_command(&mut self) -> Command {
        let mut buffer = Vec::new();
        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Could not read from input");

        let line = String::from_utf8(buffer).expect("Could not decode input");

        if is_command(line.clone()) {
            self.print_command(line.trim().to_string());
        } else {
            self.print_input(line.trim().to_string());
        }

        Command::new(line).expect("Could not build command")
    }
}
