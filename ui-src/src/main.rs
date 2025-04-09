use leptos::logging::{error, log};
use lingua_i18n_rs::prelude::Lingua;

mod app;
mod components;

use app::App;

fn main() {
    match Lingua::init_with_dir("ui-src/languages") {
        Ok(_) => log!("Sprachen erfolgreich geladen"),
        Err(e) => error!("Konnte Sprachen nicht laden: {}", e),
    }
    leptos::mount::mount_to_body(App);
}
