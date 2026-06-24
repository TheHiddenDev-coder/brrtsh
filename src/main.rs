#![allow(unused)]

use std::io::{self, Write};

fn main() {
    print_title();

    loop {
        let mut cmd: String = String::new();

        print!(">>> ");
        io::stdout().flush().expect("Failed to flush"); // TODO: add actual error handling
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line"); // TODO: add actual error handling

        let cmd = match cmd.trim() {
            "exit" => Commands::Exit,
            "help" => Commands::Help,
            "echo" => Commands::Echo,
            "mkfile" => Commands::MakeFile,
            "readfile" => Commands::ReadFile,
            _ => Commands::Unknown,
        };

        match cmd {
            Commands::Exit => break,
            Commands::Help => handle_help(),
            Commands::Unknown => println!("Unknown Command"),
            _ => println!("Not implemented yet"),
        }
    }
}

fn print_title() {
    println!(" ____  ____  ____ _____ ____  _   _");
    println!("| __ )|  _ \\|  _ \\_   _/ ___|| | | |");
    println!("|  _ \\| |_) | |_) || | \\___ \\| |_| |");
    println!("| |_) |  _ <|  _ < | |  ___) |  _  |");
    println!("|____/|_| \\_\\_| \\_\\|_| |____/|_| |_|");
    println!("BRRTSHv0.1 type `help` for command list");
}

fn handle_help() {
    println!("Command list:");
    println!("\texit -> exit program");
    println!("\techo -> print to screen");
    println!("\thelp -> list commands");
    println!("\tmkfile -> create file");
    println!("\treadfile -> read contents of file");
}

enum Commands {
    Exit,
    Echo, // TODO: extend command parsing to support arguments. e.g. "echo hello world" -> Commands::Echo(String)
    Help,
    MakeFile,
    ReadFile,
    Unknown,
}
