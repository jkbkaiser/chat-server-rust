use crossterm::{cursor, terminal, ExecutableCommand};
use miette::{miette, IntoDiagnostic, Result};
use std::io::{self, stdout, Write};
use std::process;
use tokio::io::{AsyncBufReadExt, BufReader, Stdin};

use crate::server::communication::client::{
    ChangeNameRequest, ClientMakeChatRoomRequest, ClientMessage, JoinChatRoomRequest,
    SendMessageRequest,
};

/// Prints help message to the terminal
fn print_help() {
    println!("usage:\n\t/make <room-name>\tcreate a new chatroom\n\t/join <room-name>\tjoins a chatroom\n\t/list\t\t\tlists all chatrooms\n\t/cname <new-username>\tchanges used name\n\t/exit\t\t\texits the application")
}

/// Crops a given number of characters from the start of a string
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

/// Checks whether a given string is a command
fn is_command(line: String) -> bool {
    line.starts_with('/')
}

/// Clears a number of lines in the terminal interface
fn clear_lines(num: u16) -> Result<()> {
    let mut out = stdout();
    out.execute(cursor::MoveUp(num)).into_diagnostic()?;
    out.execute(cursor::MoveToColumn(0)).into_diagnostic()?;
    out.execute(terminal::Clear(terminal::ClearType::FromCursorDown))
        .into_diagnostic()?;

    Ok(())
}

/// Flush IO
fn flush_io() {
    let _ = io::stdout().flush();
}

/// Alias
pub type Command = ClientMessage;

impl Command {
    /// Instatiates a command from a parsed line
    fn from_arguments(arguments: Vec<String>) -> Result<Command> {
        let mut arguments = arguments.into_iter();
        let keyword = arguments.next().ok_or(miette!("expected args"))?;

        match &keyword[..] {
            "make" => Ok(Command::MakeChatRoom(ClientMakeChatRoomRequest {
                name: arguments
                    .next()
                    .ok_or(miette!("make chat room not enough args"))?,
            })),
            "join" => Ok(Command::JoinChatRoom(JoinChatRoomRequest {
                name: arguments
                    .next()
                    .ok_or(miette!("make join chat room not enough args"))?,
            })),
            "list" => Ok(Command::ListChatRooms()),
            "cname" => Ok(Command::ChangeName(ChangeNameRequest {
                new_name: arguments.next().ok_or(miette!("cname not enough args"))?,
            })),
            "help" => Ok(Command::Help()),
            "exit" => process::exit(0),
            _ => Err(miette!("Not a valid argument")),
        }
    }

    /// Insantiates a command from an input line
    pub fn new(line: String) -> Result<Self> {
        if line.starts_with('/') {
            let args: Vec<String> = line
                .trim()
                .get(1..)
                .ok_or(miette!("Not enough arguments"))?
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

/// Handles in and output for the terminal interface
pub struct Frontend {
    reader: BufReader<Stdin>,
    pub current_chatroom: String,
}

impl Frontend {
    /// Instatiates a new frontend
    pub fn new() -> Result<Self> {
        let reader: BufReader<Stdin> = BufReader::new(tokio::io::stdin());
        let frontend = Frontend {
            reader,
            current_chatroom: String::from("None"),
        };
        frontend.print_prompt()?;
        Ok(frontend)
    }

    /// Prints the prompt in the terminal interface
    pub fn print_prompt(&self) -> Result<()> {
        clear_lines(2)?;

        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints a chatroom message in the terminal interface
    pub fn print_message(&self, msg: String, usr: String) -> Result<()> {
        clear_lines(2)?;

        print!(
            "{usr}: {msg}\n-------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints a command in the terminal interface
    pub fn print_command(&self, msg: String) -> Result<()> {
        clear_lines(3)?;

        print!(
            "-> {}\n-------------------------\n(room: {})\n⤷ ",
            crop_letters(&msg, 1),
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints user input in the terminal interface
    pub fn print_input(&self, inp: String) -> Result<()> {
        clear_lines(3)?;

        print!(
            "You: {inp}\n-------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints help in the terminal interface
    pub fn print_help(&self) -> Result<()> {
        clear_lines(2)?;

        print_help();
        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints help in the terminal interface after invalid command is issued
    pub fn print_invalid_command_help(&self, command: String) -> Result<()> {
        clear_lines(2)?;

        print!("!invalid command: {}", command);
        print_help();
        print!(
            "------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Prints list of rooms in the terminal interface
    pub fn print_rooms(&self, rooms: Vec<String>) -> Result<()> {
        clear_lines(2)?;

        println!("Chat rooms:");
        print!("\t{}", rooms.join("\n\t"));
        print!(
            "\n------------------------\n(room: {})\n⤷ ",
            self.current_chatroom
        );

        flush_io();

        Ok(())
    }

    /// Returns a next command if there is one
    pub async fn next(&mut self) -> Result<Option<Command>> {
        let mut buffer = Vec::new();
        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .into_diagnostic()?;

        let line = String::from_utf8(buffer).into_diagnostic()?;

        if is_command(line.clone()) {
            self.print_command(line.trim().to_string())?;
        } else {
            self.print_input(line.trim().to_string())?;
        }

        let command = Command::new(line.clone());

        match command {
            Ok(c) => Ok(Some(c)),
            Err(_) => {
                self.print_invalid_command_help(line)?;
                Ok(None)
            }
        }
    }
}
