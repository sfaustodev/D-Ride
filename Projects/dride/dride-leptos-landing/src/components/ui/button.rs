use leptos::*;
use leptos_router::Link;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] loading: MaybeSignal<bool>,
    #[prop(into, optional)] to: Option<String>,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, optional)] target: Option<String>,
    #[prop(into, optional)] icon: Option<View>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.get_untracked().unwrap_or(ButtonVariant::Primary);
    let size = size.get_untracked().unwrap_or(ButtonSize::Md);
    let disabled = disabled.get_untracked();
    let loading = loading.get_untracked();

    let base_classes = "inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-brand-purple focus:ring-offset-2 focus:ring-offset-bg-primary disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-brand-purple hover:bg-brand-purple-dark text-white shadow-lg shadow-brand-purple/25 active:scale-95",
        ButtonVariant::Secondary => "bg-bg-secondary hover:bg-bg-tertiary text-text-primary border border-border active:scale-95",
        ButtonVariant::Outline => "bg-transparent hover:bg-bg-tertiary text-text-primary border border-border active:scale-95",
        ButtonVariant::Ghost => "bg-transparent hover:bg-bg-tertiary text-text-primary active:scale-95",
        ButtonVariant::Destructive => "bg-accent-red hover:bg-accent-red/90 text-white shadow-lg active:scale-95",
    };

    let size_classes = match size {
        ButtonSize::Sm => "px-3 py-1.5 text-sm",
        ButtonSize::Md => "px-4 py-2 text-base",
        ButtonSize::Lg => "px-6 py-3 text-lg",
        ButtonSize::Xl => "px-8 py-4 text-xl",
    };

    let is_disabled = disabled || loading;

    let content = view! {
        <span class="flex items-center gap-2">
            {if loading {
                Some(view! {
                    <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                })
            } else {
                icon
            }}
            {children()}
        </span>
    };

    if let Some(to) = to {
        view! {
            <Link to=to
                  class=format!("{} {} {}", base_classes, variant_classes, size_classes, class)
                  disabled=is_disabled
                  attr:tabindex={if is_disabled { "-1" } else { "0" }}>
                {content}
            </Link>
        }
    } else if let Some(href) = href {
        view! {
            <a href=href
               class=format!("{} {} {}", base_classes, variant_classes, size_classes, class)
               disabled=is_disabled
               target=target.unwrap_or("_self")
               rel="noopener noreferrer">
                {content}
            </a>
        }
    } else {
        view! {
            <button class=format!("{} {} {}", base_classes, variant_classes, size_classes, class)
                    disabled=is_disabled
                    type="button"
                    on:click=move |e| {
                        if let Some(on_click) = on_click {
                            if !is_disabled {
                                on_click.call(e);
                            }
                        }
                    }>
                {content}
            </button>
        }
    }
}
