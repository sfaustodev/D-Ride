use leptos::*;

#[component]
pub fn AppRoot() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-bg-primary">
            // Content will be added in Phase 3
            <div class="flex items-center justify-center min-h-screen">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-white mb-4">
                        "dRide Landing Page"
                    </h1>
                    <p class="text-gray-400">
                        "Rust + Leptos implementation in progress..."
                    </p>
                </div>
            </div>
        </div>
    }
}
