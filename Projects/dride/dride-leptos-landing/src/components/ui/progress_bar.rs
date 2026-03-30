use leptos::*;

#[component]
pub fn ProgressBar(
    #[prop(into)] progress: Signal<f64>,
    #[prop(into, optional)] max: MaybeSignal<f64>,
    #[prop(into, optional)] show_percentage: MaybeSignal<bool>,
    #[prop(into, optional)] animated: MaybeSignal<bool>,
    #[prop(into, optional)] variant: MaybeSignal<ProgressBarVariant>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let max_value = max.get_untracked().unwrap_or(100.0);
    let should_animate = animated.get_untracked().unwrap_or(true);
    let variant = variant.get_untracked().unwrap_or(ProgressBarVariant::Default);
    let show_pct = show_percentage.get_untracked().unwrap_or(true);

    let current_progress = move || {
        let p = progress.get();
        if p < 0.0 { 0.0 }
        else if p > max_value { max_value }
        else { p }
    };

    let percentage = move || {
        (current_progress() / max_value * 100.0).min(100.0).max(0.0)
    };

    let bar_classes = match variant {
        ProgressBarVariant::Default => "bg-gradient-to-r from-brand-purple to-brand-purple-dark shadow-lg shadow-brand-purple/25",
        ProgressBarVariant::Green => "bg-gradient-to-r from-accent-green to-accent-green-dark shadow-lg shadow-accent-green/25",
        ProgressBarVariant::Blue => "bg-gradient-to-r from-blue-500 to-blue-600 shadow-lg shadow-blue-500/25",
    };

    let animation_classes = if should_animate {
        "transition-all duration-500 ease-out"
    } else {
        ""
    };

    view! {
        <div class=format!("w-full {}", class)>
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-text-secondary">Progress</span>
                {if show_pct {
                    Some(view! {
                        <span class="text-sm font-bold text-text-primary">
                            {move || format!("{:.1}%", percentage())}
                        </span>
                    })
                }}
            </div>
            <div class="w-full bg-bg-tertiary rounded-full overflow-hidden">
                <div
                    class=format!("h-2 rounded-full {} {}", bar_classes, animation_classes)
                    style=move || format!("width: {}%", current_progress())
                >
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgressBarVariant {
    Default,
    Green,
    Blue,
}
