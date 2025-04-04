use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background text-white p-4">
            <header class="max-w-4xl mx-auto mb-8 text-center">
                <h1 class="text-3xl md:text-4xl font-bold text-primary-100 mb-2">
                    "Rusty Weather"
                </h1>
                <p class="text-lg text-gray-400">
                    "Ihr zuverlässiger Wetterdienst"
                </p>
            </header>

            <main class="max-w-4xl mx-auto">
                <div class="bg-background-card rounded-lg shadow-lg p-4 mb-6">
                    <div class="flex flex-col md:flex-row gap-2">
                        <input
                            type="text"
                            placeholder="Stadt eingeben..."
                            class="flex-1 p-3 border border-primary-300 rounded-lg bg-background text-white placeholder-gray-400 focus:ring-2 focus:ring-primary-100 focus:border-primary-100"
                        />
                        <button
                            class="bg-gradient-primary hover:opacity-90 text-white font-medium rounded-lg p-3 transition-all"
                        >
                            "Wetter abrufen"
                        </button>
                    </div>
                </div>

                <div class="bg-background-card rounded-lg shadow-lg overflow-hidden">
                    <div class="bg-gradient-primary text-white p-6">
                        <h2 class="text-2xl font-bold">"Wetterbericht"</h2>
                        <p class="text-white opacity-80">"Wählen Sie eine Stadt, um das Wetter anzuzeigen"</p>
                    </div>

                    <div class="p-6">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="bg-background p-4 rounded-lg border border-primary-400">
                                <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                    "Temperatur"
                                </h3>
                                <p class="text-2xl font-bold text-white">
                                    "--°C"
                                </p>
                            </div>
                            <div class="bg-background p-4 rounded-lg border border-primary-400">
                                <h3 class="text-sm text-primary-200 uppercase font-semibold mb-2">
                                    "Bedingung"
                                </h3>
                                <p class="text-2xl font-bold text-white">
                                    "---"
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

            <footer class="max-w-4xl mx-auto mt-8 text-center text-gray-400 text-sm">
                <p>"Powered by Rusty Weather CLI - Leptos Edition - Developt with ❤️ by KarnesTH"</p>
            </footer>
        </div>
    }
}
