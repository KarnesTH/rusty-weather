use clap::Parser;
use lingua_i18n_rs::prelude::Lingua;
use rusty_weather::prelude::*;

#[tokio::main]
async fn main() {
    Lingua::init_with_dir("languages").unwrap();
    let args = Cli::parse();
    match args.commands {
        WeatherCommand::Current { city, lang } => {
            if let Some(lang) = lang.clone() {
                Lingua::set_language(lang.as_str()).unwrap();
            }
            let weather = Weather::new();
            let result = weather.get_current_weather(city, lang).await;
            match result {
                Ok(weather) => {
                    weather.print_current_weather();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
