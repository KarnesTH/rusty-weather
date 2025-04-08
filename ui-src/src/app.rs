use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::CurrentWeather;
use crate::components::ForecastWeather;

#[derive(Clone, Copy, PartialEq)]
enum WeatherPage {
    Current,
    Forecast,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal(WeatherPage::Current);

    view! {
        <Router>
            <div class="min-h-screen bg-background text-white p-4">
                <header class="max-w-4xl mx-auto mb-8 text-center">
                    <h1 class="text-7xl md:text-4xl font-bold text-primary-100 mb-2">
                        "Rusty Weather"
                    </h1>
                    <p class="text-lg text-gray-400">
                        "Dein persönlicher Wetterdienst für aktuelles und vorhergesagtes Wetter"
                    </p>
                    <nav class="flex justify-center gap-4 mt-4">
                        <A href="/">
                            <span
                                class=move ||format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium text-gray-400 hover:text-gray-300 hover:border-gray-600 {}", if current_page.get() == WeatherPage::Current {
                                    "border-primary-100"
                                } else {
                                    "border-transparent"
                                })
                                on:click=move |_| {
                                    set_current_page.set(WeatherPage::Current);
                                }
                            >
                                "Aktuelles Wetter"
                            </span>
                        </A>

                        <A href="/forecast">
                            <span
                                class=move || format!("inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium text-gray-400 hover:text-gray-300 hover:border-gray-600 {}", if current_page.get() == WeatherPage::Forecast {
                                    "border-primary-100"
                                } else {
                                    "border-transparent"
                                })
                                on:click=move |_| {
                                    set_current_page.set(WeatherPage::Forecast);
                                }
                            >
                                "Wettervorhersage"
                            </span>
                        </A>
                    </nav>
                </header>

                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=CurrentWeather />
                    <Route path=path!("/forecast") view=ForecastWeather />
                </Routes>

                <footer class="max-w-4xl mx-auto mt-8 text-center text-gray-400 text-sm">
                    <p>"Powered by Rusty Weather CLI - Leptos Edition - Developt with ❤️ by KarnesTH"</p>
                </footer>
            </div>
        </Router>
    }
}
