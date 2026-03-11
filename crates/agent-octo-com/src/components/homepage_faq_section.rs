use daisy_rsx::marketing::faq_accordian::{Faq, FaqText};
use dioxus::prelude::*;

#[component]
pub fn HomepageFaqSection() -> Element {
    rsx! {
        Faq {
            questions: faq_items(),
            class: Some("mx-auto max-w-4xl px-6 py-16 lg:px-12".to_string())
        }
    }
}

fn faq_items() -> Vec<FaqText> {
    vec![
        FaqText {
            question: "Who is Agent Octo for?".to_string(),
            answer: "Teams that want to run an agent product themselves, especially when they need integrations, channels, and tenant isolation.".to_string(),
        },
        FaqText {
            question: "Do I have to hardcode every tool?".to_string(),
            answer: "No. A core part of the model is loading integrations from OpenAPI specs so the available tool surface can evolve at runtime.".to_string(),
        },
        FaqText {
            question: "Can I connect my own model provider?".to_string(),
            answer: "Yes. Provider connections are part of the product, including OAuth2-based setups for APIs that require it.".to_string(),
        },
        FaqText {
            question: "Is this only for Telegram bots?".to_string(),
            answer: "No. Telegram is the current channel path in this repo, but the architecture separates channels from the rest of the agent runtime.".to_string(),
        },
        FaqText {
            question: "Why a sandbox at all?".to_string(),
            answer: "Because useful agents eventually need to execute code or handle richer workloads, and that needs explicit operational boundaries.".to_string(),
        },
    ]
}
