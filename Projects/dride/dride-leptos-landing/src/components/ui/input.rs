use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InputVariant {
    Default,
    Primary,
    Error,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Input(
    #[prop(into, optional)] variant: MaybeSignal<InputVariant>,
    #[prop(into, optional)] size: MaybeSignal<InputSize>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, optional)] error: Option<String>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] type_: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let variant = variant.get_untracked().unwrap_or(InputVariant::Default);
    let size = size.get_untracked().unwrap_or(InputSize::Md);
    let disabled = disabled.get_untracked();
    let input_type = type_.unwrap_or("text".to_string());

    let base_classes = "w-full rounded-lg border transition-all duration-200 focus:outline-none focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_classes = match variant {
        InputVariant::Default => "bg-bg-tertiary border-border text-text-primary focus:ring-brand-purple focus:border-brand-purple placeholder:text-text-tertiary",
        InputVariant::Primary => "bg-bg-tertiary border-brand-purple/30 text-text-primary focus:ring-brand-purple focus:border-brand-purple placeholder:text-text-tertiary",
        InputVariant::Error => "bg-bg-tertiary border-accent-red text-text-primary focus:ring-accent-red focus:border-accent-red placeholder:text-text-tertiary",
    };

    let size_classes = match size {
        InputSize::Sm => "px-3 py-2 text-sm",
        InputSize::Md => "px-4 py-3 text-base",
        InputSize::Lg => "px-5 py-4 text-lg",
    };

    let input_class = format!("{} {} {}", base_classes, variant_classes, size_classes, class);

    view! {
        <div class="flex flex-col gap-2">
            {if let Some(label_text) = label {
                Some(view! {
                    <label class="text-sm font-medium text-text-secondary">{label_text}</label>
                })
            }}
            <div class="relative">
                <input
                    type=input_type
                    name=name.unwrap_or_default()
                    class=input_class
                    disabled=disabled
                    placeholder=placeholder
                    on:input=move |e| {
                        value.set(event_target_value(&e));
                    }
                    prop:value=value
                />
                {if let Some(error_msg) = error {
                    Some(view! {
                        <p class="mt-1 text-sm text-accent-red">{error_msg}</p>
                    })
                }}
            </div>
        </div>
    }
}
