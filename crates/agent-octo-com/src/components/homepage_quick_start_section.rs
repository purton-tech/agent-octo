use daisy_rsx::{Card, CardBody};
use dioxus::prelude::*;

#[component]
pub fn HomepageQuickStartSection() -> Element {
    rsx! {
        section {
            id: "quick-start",
            class: "px-6 py-16 lg:px-12",
            div {
                class: "mx-auto max-w-6xl space-y-8",
                div {
                    class: "max-w-3xl space-y-3",
                    h2 { class: "text-3xl font-bold", "Quick start" }
                    p {
                        class: "text-base-content/75",
                        "This follows the same broad placement as the setup section on the inspiration page, but grounded in the actual README flow for Agent Octo."
                    }
                }
                div {
                    class: "grid gap-6 lg:grid-cols-[1.2fr_0.8fr]",
                    Card {
                        class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                        CardBody {
                            class: Some("gap-4".to_string()),
                            h3 { class: "card-title", "Start locally in a few steps" }
                            ol {
                                class: "list-decimal space-y-3 pl-5",
                                li { "Download the deployment file and create your environment variables." }
                                li { "Add a Telegram bot token and your preferred model provider credentials." }
                                li { "Run `docker compose up`, then open the app and configure providers, integrations, and channels." }
                            }
                        }
                    }
                    Card {
                        class: Some("border border-base-300 bg-base-200/50 shadow-sm".to_string()),
                        CardBody {
                            class: Some("gap-3".to_string()),
                            h3 { class: "card-title", "Good first configuration" }
                            ul {
                                class: "list-disc space-y-2 pl-5",
                                li { "One provider connection for your preferred LLM" }
                                li { "One Telegram channel for ingress and egress" }
                                li { "One OpenAPI integration the agent can call safely" }
                            }
                        }
                    }
                }
            }
        }
    }
}
