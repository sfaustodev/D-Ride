use leptos::*;

#[component]
pub fn SectionWrapper(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            id=id.unwrap_or_default()
            class=format!("min-h-screen py-20 px-4 sm:px-6 lg:px-8 {}", class)
        >
            <div class="max-w-7xl mx-auto">
                {children()}
            </div>
        </section>
    }
}
