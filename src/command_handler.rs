use std::io::{self, Write};

pub enum Commands {
    Exit,
    Echo(String),
    Help,
    MakeFile,
    ReadFile,
    PrintBanner,
    Unknown,
}

pub fn parse_commands(input: &str) -> Commands {
    let mut parts = input.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match cmd {
        "exit" => Commands::Exit,
        "help" => Commands::Help,
        "echo" => Commands::Echo(args.to_string()),
        "mkfile" => Commands::MakeFile,
        "readfile" => Commands::ReadFile,
        "printbanner" => Commands::PrintBanner,
        _ => Commands::Unknown,
    }
}

pub fn print_title() {
    println!(" ____  ____  ____ _____ ____  _   _");
    println!("| __ )|  _ \\|  _ \\_   _/ ___|| | | |");
    println!("|  _ \\| |_) | |_) || | \\___ \\| |_| |");
    println!("| |_) |  _ <|  _ < | |  ___) |  _  |");
    println!("|____/|_| \\_\\_| \\_\\|_| |____/|_| |_|");
    println!();
    println!("BRRTSH v0.2 (late-alpha)");
    println!("type `help` for command list");
}

pub fn handle_help() {
    println!("Command list:");
    println!("\texit -> exit program");
    println!("\techo <text> -> print text to screen");
    println!("\thelp -> list commands");
    println!("\tmkfile -> create file");
    println!("\treadfile -> read contents of file");
    println!("\tprintbanner -> print banner that shows at startup")
}

pub fn handle_echo(text: &str) {
    println!("{}", text);
}
