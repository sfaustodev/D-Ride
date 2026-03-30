use leptos::*;
use leptos_dom::*;
use dride_leptos_landing::components::ui::{Button, ButtonVariant, ButtonSize};

#[test]
fn test_button_renders_primary() {
    mount_to(div(), || {
        view! {
            <Button variant=ButtonVariant::Primary>
                "Click me"
            </Button>
        }
    });

    let button = document().get_elements_by_tag_name("button").first().unwrap();
    assert!(button.class_list().contains("bg-brand-purple"));
    assert!(button.class_list().contains("text-white"));
}

#[test]
fn test_button_renders_secondary() {
    mount_to(div(), || {
        view! {
            <Button variant=ButtonVariant::Secondary>
                "Click me"
            </Button>
        }
    });

    let button = document().get_elements_by_tag_name("button").first().unwrap();
    assert!(button.class_list().contains("bg-bg-secondary"));
    assert!(button.class_list().contains("border-border"));
}

#[test]
fn test_button_disabled_state() {
    mount_to(div(), || {
        view! {
            <Button disabled=true>
                "Click me"
            </Button>
        }
    });

    let button = document().get_elements_by_tag_name("button").first().unwrap();
    assert!(button.get_attribute("disabled").is_some());
    assert!(button.class_list().contains("disabled:opacity-50"));
}

#[test]
fn test_button_loading_state() {
    mount_to(div(), || {
        view! {
            <Button loading=true>
                "Click me"
            </Button>
        }
    });

    let button = document().get_elements_by_tag_name("button").first().unwrap();
    assert!(button.inner_html().contains("animate-spin"));
}
