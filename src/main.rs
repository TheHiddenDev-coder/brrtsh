#![allow(unused)]
mod command_handler;
use std::io::{self, Write};

fn main() {
    command_handler::print_title();

    loop {
        let mut cmd: String = String::new();

        print!(">>> ");
        io::stdout().flush().expect("Failed to flush"); // TODO: add actual error handling
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line"); // TODO: add actual error handling

        let cmd = match cmd.trim() {
            "exit" => command_handler::Commands::Exit,
            "help" => command_handler::Commands::Help,
            "echo" => command_handler::Commands::Echo,
            "mkfile" => command_handler::Commands::MakeFile,
            "readfile" => command_handler::Commands::ReadFile,
            _ => command_handler::Commands::Unknown,
        };

        match cmd {
            command_handler::Commands::Exit => break,
            command_handler::Commands::Help => command_handler::handle_help(),
            command_handler::Commands::Echo => command_handler::handle_echo(),
            command_handler::Commands::Unknown => println!("Unknown Command"),
            _ => println!("Not implemented yet"),
        }
    }
}
