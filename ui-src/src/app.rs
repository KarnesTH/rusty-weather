use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::CurrentWeather;
use crate::components::ForecastWeather;

#[component]
pub fn App() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);

    let nav_style = move || {
        if is_active.get() {
            "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium border-primary-100 text-gray-400 hover:text-gray-300 hover:border-gray-600".to_string()
        } else {
            "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-600".to_string()
        }
    };

    view! {
        <Router>
            <div class="min-h-screen bg-background text-white p-4">
                <header class="max-w-4xl mx-auto mb-8 text-center">
                    <h1 class="text-3xl md:text-4xl font-bold text-primary-100 mb-2">
                        "Rusty Weather"
                    </h1>
                    <p class="text-lg text-gray-400">
                        "Ihr zuverlässiger Wetterdienst"
                    </p>
                    <nav class="flex justify-center gap-4 mt-4">
                        <A href="/">
                            <span
                                class=move || nav_style
                            >
                                "Aktuelles Wetter"
                            </span>
                        </A>

                        <A href="/forecast">
                            <span
                                class=move || nav_style
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
