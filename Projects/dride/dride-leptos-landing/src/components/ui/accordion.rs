use leptos::*;

#[component]
pub fn Accordion(
    #[prop(into, optional)] default_open: MaybeSignal<bool>,
    #[prop(into, optional)] class: String,
    title: String,
    children: Children,
) -> impl IntoView {
    let is_open = create_rw_signal(default_open.get_untracked().unwrap_or(false));

    view! {
        <div class=format!("w-full {}", class)>
            <button
                class="w-full flex items-center justify-between px-6 py-4 bg-bg-secondary rounded-lg border border-border hover:border-brand-purple/30 transition-all duration-200"
                on:click=move |_| {
                    is_open.update(|o| *o = !*o);
                }
            >
                <span class="text-lg font-semibold text-text-primary">{title}</span>
                <svg
                    class=format!("w-5 h-5 text-text-secondary transition-transform duration-200 {}", if is_open.get() { "rotate-180" } else { "" })
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
            </button>

            <div
                class=format!("overflow-hidden transition-all duration-300 ease-in-out {}", if is_open.get() { "max-h-96 opacity-100" } else { "max-h-0 opacity-0" })
            >
                <div class="px-6 py-4">
                    {children()}
                </div>
            </div>
        </div>
    }
}
