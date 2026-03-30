use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardVariant {
    Default,
    Glass,
    Glow,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GlowColor {
    Purple,
    Green,
    Red,
    Amber,
}

#[component]
pub fn Card(
    #[prop(into, optional)] variant: MaybeSignal<CardVariant>,
    #[prop(into, optional)] glow_color: MaybeSignal<GlowColor>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = variant.get_untracked().unwrap_or(CardVariant::Default);
    let glow_color = glow_color.get_untracked().unwrap_or(GlowColor::Purple);

    let base_classes = "rounded-16 backdrop-blur-20 transition-all duration-300";

    let (bg_classes, border_classes) = match variant {
        CardVariant::Default => (
            "bg-bg-secondary",
            "border border-border",
        ),
        CardVariant::Glass => (
            "bg-bg-secondary/80 backdrop-blur-xl",
            "border border-border/50",
        ),
        CardVariant::Glow => {
            let glow_class = match glow_color {
                GlowColor::Purple => "before:from-brand-purple/60 before:via-brand-purple/10 before:to-brand-purple/10",
                GlowColor::Green => "before:from-accent-green/60 before:via-accent-green/10 before:to-accent-green/10",
                GlowColor::Red => "before:from-accent-red/60 before:via-accent-red/10 before:to-accent-red/10",
                GlowColor::Amber => "before:from-accent-amber/60 before:via-accent-amber/10 before:to-accent-amber/10",
            };

            (
                "bg-bg-secondary/90 backdrop-blur-xl relative overflow-hidden",
                format!("before:content-[''] before:absolute before:inset-0 before:rounded-16 before:padding-1 before:bg-gradient-to-r {} after:mask-linear-gradient after:[mask:linear-gradient(#fff_0_0)_content-box,_linear-gradient(#fff_0_0)] after:[-webkit-mask:linear-gradient(#fff_0_0)_content-box,_linear-gradient(#fff_0_0)] after:[-webkit-mask-composite:xor] after:[mask-composite:exclude]", glow_class)
            )
        }
    };

    view! {
        <div class=format!("{} {} {}", base_classes, bg_classes, border_classes, class)>
            {children()}
        </div>
    }
}
