use crossterm::{cursor, terminal, ExecutableCommand};
use std::process;
use std::{error, io, io::stdout, io::Write};
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

use crate::server::communication::client::{
    ChangeNameRequest, ClientMessage, JoinChatRoomRequest, MakeChatRoomRequest, SendMessageRequest,
};

fn print_help() {
    println!("usage:\n\t/make <room-name>\tcreate a new chatroom\n\t/join <room-name>\tjoins a chatroom\n\t/list\t\t\tlists all chatrooms\n\t/cname <new-username>\tchanges used name\n\t/exit\t\t\texits the application")
}

pub type Command = ClientMessage;

impl Command {
    pub fn from_arguments(arguments: Vec<String>) -> Result<Command, Box<dyn error::Error>> {
        let mut arguments = arguments.into_iter();

        let keyword = arguments.next().expect("expected args");

        match &keyword[..] {
            "make" => Ok(Command::MakeChatRoom(MakeChatRoomRequest {
                name: arguments.next().ok_or("make chat room not enough args")?,
            })),
            "join" => Ok(Command::JoinChatRoom(JoinChatRoomRequest {
                name: arguments
                    .next()
                    .ok_or("make join chat room not enough args")?,
            })),
            "list" => Ok(Command::ListChatRooms()),
            "cname" => Ok(Command::ChangeName(ChangeNameRequest {
                new_name: arguments.next().ok_or("cname not enough args")?,
            })),
            "help" => Ok(Command::Help()),
            "exit" => process::exit(0),
            _ => Err("invalid command".into()),
        }
    }

    pub fn new(line: String) -> Result<Self, Box<dyn error::Error>> {
        if line.starts_with('/') {
            let args: Vec<String> = line
                .trim()
                .get(1..)
                .ok_or("test")?
                .split(' ')
                .map(str::to_string)
                .collect();

            Command::from_arguments(args)
        } else {
            Ok(Command::SendMessage(SendMessageRequest::new(
                line.trim().to_string(),
            )))
        }
    }
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn is_command(line: String) -> bool {
    line.starts_with('/')
}

pub struct Frontend {
    reader: BufReader<Stdin>,
    pub current_chatroom: String,
}

impl Default for Frontend {
    fn default() -> Self {
        Self::new()
    }
}

impl Frontend {
    pub fn new() -> Self {
        let reader: BufReader<Stdin> = BufReader::new(tokio::io::stdin());
        let f = Frontend {
            reader,
            current_chatroom: String::from("None"),
        };
        f.print_prompt();
        f
    }

    pub fn print_prompt(&self) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveUp(2))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_message(&self, msg: String, usr: String) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveUp(2))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print!(
            "{usr}: {msg}\n-------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_command(&self, msg: String) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveUp(3))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print!(
            "-> {}\n-------------------------\n(room: {})\n⤷ ",
            crop_letters(&msg, 1),
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_input(&self, inp: String) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveUp(3))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print!(
            "You: {inp}\n-------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_help(&self) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveUp(2))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print_help();
        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_invalid_command_help(&self, command: String) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveUp(2))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");
        print!("!invalid command: {}", command);
        print_help();
        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub fn print_rooms(&self, rooms: Vec<String>) {
        let mut stdout = stdout();
        stdout
            .execute(cursor::MoveToColumn(0))
            .expect("failed to move cursor up");
        stdout
            .execute(cursor::MoveUp(2))
            .expect("failed to move cursor left");
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .expect("Could not clear");

        print!("Chat rooms:\n");
        print!("\t{}", rooms.join("\n\t"));
        print!(
            "\n------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );
        let _ = io::stdout().flush();
    }

    pub async fn next_command(&mut self) -> Option<Command> {
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

        let command = Command::new(line.clone());

        match command {
            Ok(c) => Some(c),
            Err(_) => {
                self.print_invalid_command_help(line);
                None
            }
        }
    }
}
