use leptos::{logging::error, prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
struct WeatherResponse {
    location: WeatherLocation,
    current: WeatherCurrent,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct WeatherLocation {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct WeatherCurrent {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i64,
    condition: WeatherCondition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i64,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: i64,
    cloud: i64,
    feelslike_c: f64,
    feelslike_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct WeatherCondition {
    text: String,
    icon: String,
    code: i64,
}

#[component]
pub fn CurrentWeather() -> impl IntoView {
    let (data, set_data) = signal(WeatherResponse::default());
    let (city, set_city) = signal(String::new());

    let fetch_weather = move || {
        spawn_local(async move {
            let city_value = move || city.get();
            let url = format!("http://localhost:5000/api/v1/current/{}", city_value());
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.text().await {
                            Ok(text) => match serde_json::from_str(&text) {
                                Ok(data) => set_data.set(data),
                                Err(e) => error!("JSON parsing error: {}", e),
                            },
                            Err(e) => error!("Failed to get response text: {}", e),
                        }
                    } else {
                        error!("API error: {}", resp.status());
                    }
                }
                Err(e) => error!("Request failed: {}", e),
            }
        });
    };

    view! {
        <main class="max-w-4xl mx-auto">
            <div class="bg-background-card rounded-lg shadow-lg p-4 mb-6">
                <form
                    on:submit=move |ev| {
                        ev.prevent_default();
                        let current_city = city.get();
                        if !current_city.is_empty() {
                            fetch_weather();
                        }
                    }
                    class="flex flex-col md:flex-row gap-2"
                >
                    <input
                        type="text"
                        on:input=move |ev| set_city.set(event_target_value(&ev).parse().unwrap())
                        prop:value=move || city.get()
                        placeholder="Stadt eingeben..."
                        class="flex-1 p-3 border border-primary-300 rounded-lg bg-background text-white placeholder-gray-400 focus:ring-2 focus:ring-primary-100 focus:border-primary-100"

                    />
                    <button
                        type="submit"
                        class="bg-gradient-primary hover:opacity-90 text-white font-medium rounded-lg p-3 transition-all"
                    >
                        "Wetter abrufen"
                    </button>
                </form>
            </div>

            <div class="bg-background-card rounded-lg shadow-lg">
                <div class="bg-gradient-primary text-white p-6">
                    <div class="flex justify-between gap-4">
                        <div class="flex-start">
                            <h2 class="text-2xl font-bold">"Wetterbericht"</h2>
                            <p class="text-white opacity-80">{move || format!("Das aktuelle Wetter für: {}", data.get().location.name)}</p>
                        </div>
                        <div class="flex-end">
                            <img src={move || data.get().current.condition.icon} alt={move || data.get().current.condition.text} />
                        </div>
                    </div>
                </div>

                <div class="p-6 h-full overflow-y-auto">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div class="bg-background p-4 rounded-lg border border-primary-400">
                            <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                "Temperatur"
                            </h3>
                            <p class="text-2xl font-bold text-white">
                                {move || format!("{}°C ({}°F)", data.get().current.temp_c, data.get().current.temp_f)}
                            </p>
                        </div>
                        <div class="bg-background p-4 rounded-lg border border-primary-400">
                            <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                "Temperatur Gefühlt"
                            </h3>
                            <p class="text-2xl font-bold text-white">
                                {move || format!("{}°C ({}°F)", data.get().current.feelslike_c, data.get().current.feelslike_f)}
                            </p>
                        </div>
                        <div class="bg-background p-4 rounded-lg border border-primary-400">
                            <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                "Bedingung"
                            </h3>
                            <p class="text-2xl font-bold text-white">
                                {move || data.get().current.condition.text}
                            </p>
                        </div>
                        <div class="bg-background p-4 rounded-lg border border-primary-400">
                            <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                "Luftfeuchtigkeit"
                            </h3>
                            <p class="text-2xl font-bold text-white">
                                "--%"
                            </p>
                        </div>
                        <div class="bg-background p-4 rounded-lg border border-primary-400">
                            <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                "Windgeschwindigkeit"
                            </h3>
                            <p class="text-2xl font-bold text-white">
                                "-- km/h"
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
