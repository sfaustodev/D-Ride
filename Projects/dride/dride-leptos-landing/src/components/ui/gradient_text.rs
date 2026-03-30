use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GradientVariant {
    Purple,
    Green,
    Red,
    Amber,
    Blue,
    Rainbow,
}

#[component]
pub fn GradientText(
    #[prop(into, optional)] variant: MaybeSignal<GradientVariant>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = variant.get_untracked().unwrap_or(GradientVariant::Purple);

    let gradient_classes = match variant {
        GradientVariant::Purple => "bg-gradient-to-r from-brand-purple via-brand-purple-dark to-brand-purple-light bg-clip-text text-transparent",
        GradientVariant::Green => "bg-gradient-to-r from-accent-green via-accent-green-dark to-accent-green-light bg-clip-text text-transparent",
        GradientVariant::Red => "bg-gradient-to-r from-accent-red via-accent-red-dark to-accent-red-light bg-clip-text text-transparent",
        GradientVariant::Amber => "bg-gradient-to-r from-accent-amber via-accent-amber-dark to-accent-amber-light bg-clip-text text-transparent",
        GradientVariant::Blue => "bg-gradient-to-r from-blue-500 via-blue-600 to-blue-400 bg-clip-text text-transparent",
        GradientVariant::Rainbow => "bg-gradient-to-r from-purple-500 via-pink-500 to-red-500 bg-clip-text text-transparent animate-gradient",
    };

    view! {
        <span class=format!("{} {}", gradient_classes, class)>
            {children()}
        </span>
    }
}
