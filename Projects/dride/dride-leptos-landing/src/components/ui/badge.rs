use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeVariant {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Badge(
    #[prop(into, optional)] variant: MaybeSignal<BadgeVariant>,
    #[prop(into, optional)] size: MaybeSignal<BadgeSize>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = variant.get_untracked().unwrap_or(BadgeVariant::Default);
    let size = size.get_untracked().unwrap_or(BadgeSize::Md);

    let base_classes = "inline-flex items-center justify-center rounded-full font-medium transition-all duration-200";

    let variant_classes = match variant {
        BadgeVariant::Default => "bg-bg-tertiary text-text-primary",
        BadgeVariant::Primary => "bg-brand-purple/20 text-brand-purple",
        BadgeVariant::Success => "bg-accent-green/20 text-accent-green",
        BadgeVariant::Warning => "bg-accent-amber/20 text-accent-amber",
        BadgeVariant::Error => "bg-accent-red/20 text-accent-red",
    };

    let size_classes = match size {
        BadgeSize::Sm => "px-2 py-0.5 text-xs",
        BadgeSize::Md => "px-3 py-1 text-sm",
        BadgeSize::Lg => "px-4 py-1.5 text-base",
    };

    view! {
        <span class=format!("{} {} {}", base_classes, variant_classes, size_classes, class)>
            {children()}
        </span>
    }
}
