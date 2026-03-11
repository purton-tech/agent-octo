use daisy_rsx::{Card, CardBody};
use dioxus::prelude::*;

#[component]
pub fn HomepagePrinciplesSection() -> Element {
    rsx! {
        section {
            id: "principles",
            class: "px-6 py-16 lg:px-12",
            div {
                class: "mx-auto max-w-6xl space-y-8",
                div {
                    class: "max-w-3xl space-y-3",
                    h2 { class: "text-3xl font-bold", "Built around practical agent operations" }
                    p {
                        class: "text-base-content/75",
                        "The source page has a product-principles block. This version keeps that layout but shifts the copy to how Agent Octo is meant to be used."
                    }
                }
                div {
                    class: "grid gap-6 lg:grid-cols-3",
                    Card {
                        class: Some("bg-base-200/50 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Tools stay close to the work" }
                            p { "Integrations are discoverable and lightweight in context so agents are not flooded with irrelevant tool definitions." }
                        }
                    }
                    Card {
                        class: Some("bg-base-200/50 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Sandboxing matters" }
                            p { "The Rust-powered Python sandbox and Code Mode support are there to make code execution more practical and more controlled." }
                        }
                    }
                    Card {
                        class: Some("bg-base-200/50 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Configuration is productized" }
                            p { "Providers, channels, and integrations are first-class application concepts instead of one-off scripts hidden in deployment glue." }
                        }
                    }
                }
            }
        }
    }
}
