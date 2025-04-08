use leptos::prelude::*;

#[component]
pub fn CurrentWeather() -> impl IntoView {
    view! {
        <main class="max-w-4xl mx-auto">
            <div class="bg-background-card rounded-lg shadow-lg p-4 mb-6">
                <form class="flex flex-col md:flex-row gap-2">
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
                </form>
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
    }
}
