use dotenv::{dotenv, var};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub location: WeatherLocation,
    pub current: WeatherCurrent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherLocation {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCurrent {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i64,
    pub condition: WeatherCondition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i64,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

impl Weather {
    pub fn new() -> Self {
        Weather {
            location: WeatherLocation {
                name: "".to_string(),
                region: "".to_string(),
                country: "".to_string(),
                lat: 0.0,
                lon: 0.0,
                tz_id: "".to_string(),
                localtime_epoch: 0,
                localtime: "".to_string(),
            },
            current: WeatherCurrent {
                last_updated_epoch: 0,
                last_updated: "".to_string(),
                temp_c: 0.0,
                temp_f: 0.0,
                is_day: 0,
                condition: WeatherCondition {
                    text: "".to_string(),
                    icon: "".to_string(),
                    code: 0,
                },
                wind_mph: 0.0,
                wind_kph: 0.0,
                wind_degree: 0,
                wind_dir: "".to_string(),
                pressure_mb: 0.0,
                pressure_in: 0.0,
                precip_mm: 0.0,
                precip_in: 0.0,
                humidity: 0,
                cloud: 0,
                feelslike_c: 0.0,
                feelslike_f: 0.0,
                vis_km: 0.0,
                vis_miles: 0.0,
                uv: 0.0,
                gust_mph: 0.0,
                gust_kph: 0.0,
            },
        }
    }

    pub async fn fetch_current_weather(
        &self,
        city: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let url = &format!(
            "http://api.weatherapi.com/v1/current.json?key={}&q={}",
            var("WEATHER_API").unwrap(),
            city
        );

        let resp = reqwest::get(url).await;
        let data = serde_json::from_str(&resp?.text().await?)?;

        Ok(data)
    }

    pub fn print_current_weather(&self) {
        println!("Current weather in {}\n", self.location.name);
        println!("Temperature: {}°C", self.current.temp_c);
        println!("Condition: {}", self.current.condition.text);

        if self.current.is_day == 1 {
            println!("Daytime");
        } else {
            println!("Nighttime");
        }

        println!("Wind: {} kph", self.current.wind_kph);

        if self.current.precip_mm > 0.0 {
            println!("Precipitation: {} mm", self.current.precip_mm);
        }

        println!("Humidity: {}%", self.current.humidity);
        println!("Cloud cover: {}%", self.current.cloud);
        println!("Feels like: {}°C", self.current.feelslike_c);
        println!("Visibility: {} km", self.current.vis_km);
        println!("UV index: {}", self.current.uv);
    }
}
