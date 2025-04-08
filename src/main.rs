use std::path::Path;

use clap::Parser;
use lingua_i18n_rs::prelude::Lingua;
use rusty_weather::prelude::*;

#[tokio::main]
async fn main() {
    Lingua::init_with_dir("languages").unwrap();

    let language = Lingua::load_lang_from_config(Path::new("config.ini"), "language").unwrap();

    Lingua::set_language(language.as_str()).unwrap();

    let args = Cli::parse();
    match args.commands {
        WeatherCommand::Current { city } => {
            let lang = Lingua::get_language().unwrap();
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
        WeatherCommand::Forecast { city, days } => {
            let lang = Lingua::get_language().unwrap();
            let weather = ForecastWeather::new();
            let result = weather.get_forecast_weather(days, city, lang).await;
            match result {
                Ok(weather) => {
                    weather.print_forecast_weather();
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        WeatherCommand::Language { commands } => match commands {
            LanguageCommand::Set { lang } => {
                if let Ok(is_set_lang) = Lingua::set_language(lang.as_str()) {
                    if is_set_lang {
                        save_language(lang.as_str()).unwrap();
                        println!(
                            "{}",
                            Lingua::t("language.set_language_ok", &[("lang", lang.as_str())])
                                .unwrap()
                        )
                    }
                } else {
                    println!(
                        "{}",
                        Lingua::t("language.set_language_error", &[("lang", lang.as_str())])
                            .unwrap()
                    )
                }
            }
            LanguageCommand::List => {
                let languages = Lingua::get_languages().unwrap();
                println!("{}", Lingua::t("language.list_languages", &[]).unwrap());
                for lang in languages {
                    println!("{}", lang);
                }
            }
            LanguageCommand::Get => {
                let lang = Lingua::get_language().unwrap();
                println!(
                    "{}",
                    Lingua::t("language.current_language", &[("lang", lang.as_str())]).unwrap()
                );
            }
        },
        WeatherCommand::Web { port } => {
            start_web_server(port).await.unwrap();
        }
        WeatherCommand::Serve => {
            start_server().await.unwrap();
        }
    }
}
