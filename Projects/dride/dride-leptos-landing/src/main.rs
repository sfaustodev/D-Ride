use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod app;
mod components;
mod hooks;
mod stores;
mod services;
mod security;
mod utils;
mod i18n;
mod types;

#[component]
pub fn App() -> impl IntoView {
    // Provide metadata
    provide_meta_context();

    view! {
        <Router>
            <main>
                <app::AppRoot />
            </main>
        </Router>
    }
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "dride_leptos_landing=info,warn".to_string())
        )
        .init();

    // Mount to DOM
    leptos::mount_to_body(App)
}
