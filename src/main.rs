#![allow(unused)]
mod command_handler;
use std::io::{self, Write};

use crate::command_handler::handle_echo;

fn main() {
    command_handler::print_title();

    loop {
        let mut cmd: String = String::new();

        print!(">>> ");
        if let Err(e) = io::stdout().flush() {
            println!("Failed flush: {e}");
            continue;
        }
        if io::stdin().read_line(&mut cmd).is_err() {
            println!("Input error");
            continue;
        }

        let cmd = command_handler::parse_commands(cmd.trim());

        match cmd {
            command_handler::Commands::Exit => break,
            command_handler::Commands::Help => command_handler::handle_help(),
            command_handler::Commands::Echo(text) => handle_echo(&text),
            command_handler::Commands::PrintBanner => command_handler::print_title(),
            command_handler::Commands::Unknown => println!("Unknown Command"),
            _ => println!("Not implemented yet"),
        }
    }
}
