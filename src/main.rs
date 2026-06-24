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

        let cmd = match cmd.as_str() {
            "exit" => Commands::Exit,
            _ => Commands::Unknown,
        };

        match cmd {
            Commands::Exit => break,
            _ => println!("Unknown Command"),
        }
    }
}

fn print_title() {
    println!(" ____  ____  ____ _____ ____  _   _");
    println!("| __ )|  _ \\|  _ \\_   _/ ___|| | | |");
    println!("|  _ \\| |_) | |_) || | \\___ \\| |_| |");
    println!("| |_) |  _ <|  _ < | |  ___) |  _  |");
    println!("|____/|_| \\_\\_| \\_\\|_| |____/|_| |_|")
}

enum Commands {
    Exit,
    Echo,
    MakeFile,
    ReadFile,
    Unknown,
}
