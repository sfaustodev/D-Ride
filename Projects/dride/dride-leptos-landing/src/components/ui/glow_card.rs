use leptos::*;
use crate::components::ui::card::{Card, CardVariant, GlowColor};

#[component]
pub fn GlowCard(
    #[prop(into, optional)] glow_color: MaybeSignal<GlowColor>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Card variant=CardVariant::Glow glow_color=glow_color class=class>
            {children()}
        </Card>
    }
}
