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
    println!();
    println!("BRRTSH v0.1 (late-alpha)");
    println!("type `help` for command list");
}

pub fn handle_help() {
    println!("Command list:");
    println!("\texit -> exit program");
    println!("\techo -> print to screen");
    println!("\thelp -> list commands");
    println!("\tmkfile -> create file");
    println!("\treadfile -> read contents of file");
    println!("\tprintbanner -> print banner that shows at startup")
}

// TODO: Make echo actually behave like echo, you dummy!!
pub fn handle_echo() -> io::Result<()> {
    let mut input: String = String::new();
    print!("Enter text to echo: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    println!("{}", input.trim_end());
    Ok(())
}
