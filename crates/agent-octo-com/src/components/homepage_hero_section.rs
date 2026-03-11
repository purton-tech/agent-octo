use daisy_rsx::marketing::hero::Hero;
use daisy_rsx::{Badge, BadgeColor, BadgeStyle};
use dioxus::prelude::*;

#[component]
pub fn HomepageHeroSection() -> Element {
    rsx! {
        section {
            id: "hero",
            class: "px-6 py-20 lg:px-12",
            div {
                class: "mx-auto flex max-w-6xl flex-col gap-8",
                div {
                    class: "flex justify-center",
                    Badge {
                        badge_style: BadgeStyle::Outline,
                        badge_color: BadgeColor::Primary,
                        "Open source agent runtime for teams"
                    }
                }
                Hero {
                    title: "Run agents with your own tools, channels, and guardrails.".to_string(),
                    subtitle: "Agent Octo helps teams stand up a multi-tenant agent platform with runtime OpenAPI integrations, provider connections, Telegram delivery, and a Rust-based sandbox for real work.".to_string(),
                    cta_label: Some("Read the starter post".to_string()),
                    cta_href: Some("/blog".to_string()),
                    class: Some("py-0".to_string())
                }
            }
        }
    }
}
