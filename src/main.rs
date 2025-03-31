use clap::Parser;
use rusty_weather::Cli;

fn main() {
    let args = Cli::parse();
    match args.commands {
        rusty_weather::Command::Current { city } => {
            println!("Current weather for {}", city);
        }
    }
}
