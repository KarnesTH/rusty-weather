use std::path::PathBuf;

use axum::http;
use axum::{Json, Router, extract::Path, response::Html, routing::get};
use lingua_i18n_rs::prelude::Lingua;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::weather::{ForecastWeather, Weather};

async fn index() -> Html<String> {
    let html = include_str!("../../ui-src/dist/index.html");
    Html(html.to_string())
}

async fn get_current_weather(Path(city): Path<String>) -> Json<Weather> {
    let lang = Lingua::get_language().unwrap();
    let weather = Weather::new();
    let result = weather.get_current_weather(city, lang).await.unwrap();
    Json(result)
}

async fn get_forecast_weather(
    Path((city, days)): Path<(String, usize)>,
) -> Json<ForecastWeather> {
    let lang = Lingua::get_language().unwrap();
    let weather = ForecastWeather::new();
    let result = weather
        .get_forecast_weather(days, city, lang)
        .await
        .unwrap();
    Json(result)
}

pub async fn start_web_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {    
    let dist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui-src/dist");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(http::Method::GET)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .route("/api/v1/current/{city}", get(get_current_weather))
        .route("/api/v1/forecast/{city}/{days}", get(get_forecast_weather))
        .fallback_service(ServeDir::new(dist_dir))
        .layer(cors);

    let host = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(host).await?;
    println!("Server is listening on: 'http://127.0.0.1:{}'", port);
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(http::Method::GET)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/v1/current/{city}", get(get_current_weather))
        .route("/api/v1/forecast/{city}/{days}", get(get_forecast_weather))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    println!("Server is listening on: 'http://localhost:5000'");
    axum::serve(listener, app).await?;

    Ok(())
}
