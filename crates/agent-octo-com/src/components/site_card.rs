use daisy_rsx::{Card, CardBody};
use dioxus::prelude::*;

#[component]
pub fn SiteCard(
    children: Element,
    class: Option<String>,
    body_class: Option<String>,
    interactive: Option<bool>,
) -> Element {
    let mut card_class =
        "rounded-[1.75rem] border border-primary/18 bg-base-200/35 shadow-[0_18px_56px_rgba(5,10,28,0.16)] backdrop-blur-sm".to_string();

    if interactive.unwrap_or(false) {
        card_class.push_str(
            " transition-colors duration-200 hover:border-primary/38 hover:bg-base-200/48",
        );
    }

    if let Some(class) = class.filter(|class| !class.is_empty()) {
        card_class.push(' ');
        card_class.push_str(&class);
    }

    rsx! {
        Card {
            class: Some(card_class),
            CardBody {
                class: body_class,
                {children}
            }
        }
    }
}

#[component]
pub fn CardIconShell(children: Element, class: Option<String>) -> Element {
    let mut shell_class =
        "flex h-12 w-12 items-center justify-center rounded-[1rem] border border-primary/16 bg-primary/8 text-primary".to_string();

    if let Some(class) = class.filter(|class| !class.is_empty()) {
        shell_class.push(' ');
        shell_class.push_str(&class);
    }

    rsx! {
        div {
            class: "{shell_class}",
            {children}
        }
    }
}
