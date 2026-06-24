use std::io::{self, Write};

pub enum Commands {
    Exit,
    Echo, // TODO: extend command parsing to support arguments. e.g. "echo hello world" -> Commands::Echo(String)
    Help,
    MakeFile,
    ReadFile,
    PrintBanner,
    Unknown,
}

pub fn print_title() {
    println!(" ____  ____  ____ _____ ____  _   _");
    println!("| __ )|  _ \\|  _ \\_   _/ ___|| | | |");
    println!("|  _ \\| |_) | |_) || | \\___ \\| |_| |");
    println!("| |_) |  _ <|  _ < | |  ___) |  _  |");
    println!("|____/|_| \\_\\_| \\_\\|_| |____/|_| |_|");
    println!("BRRTSHv0.1 type `help` for command list");
}

pub fn handle_help() {
    println!("Command list:");
    println!("\texit -> exit program");
    println!("\techo -> print to screen");
    println!("\thelp -> list commands");
    println!("\tmkfile -> create file");
    println!("\treadfile -> read contents of file");
}

// TODO: Make echo actually behave like echo, you dummy!!
pub fn handle_echo() {
    let mut input: String = String::new();
    print!("Enter text to echo: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Falied to read line");

    println!("{}", input)
}
