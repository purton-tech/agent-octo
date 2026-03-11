use daisy_rsx::{Card, CardBody};
use dioxus::prelude::*;

struct CapabilityFeature {
    title: &'static str,
    description: &'static str,
    icon: &'static str,
}

#[component]
pub fn HomepageCapabilitiesSection() -> Element {
    rsx! {
        section {
            id: "capabilities",
            class: "px-6 py-16 lg:px-12",
            div {
                class: "mx-auto max-w-6xl space-y-8",
                div {
                    class: "max-w-3xl space-y-3",
                    h2 { class: "text-3xl font-bold", "What the platform already covers" }
                    p {
                        class: "text-base-content/75",
                        "This section mirrors the source page’s capabilities grid, but with Agent Octo’s actual product surface."
                    }
                }
                div {
                    class: "grid gap-6 md:grid-cols-2 lg:grid-cols-3",
                    for feature in capability_features() {
                        Card {
                            class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                            CardBody {
                                class: Some("gap-3".to_string()),
                                div {
                                    class: "flex items-center gap-3",
                                    img {
                                        class: "h-10 w-10 rounded-box border border-base-300 p-2",
                                        alt: "",
                                        src: "{feature.icon}"
                                    }
                                    h3 { class: "card-title text-xl", "{feature.title}" }
                                }
                                p { "{feature.description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn capability_features() -> Vec<CapabilityFeature> {
    vec![
        CapabilityFeature {
            title: "Multi-tenant by design",
            description: "Run one deployment for multiple organizations, teams, or customers with scoped configuration.",
            icon: "/logo.svg",
        },
        CapabilityFeature {
            title: "Provider connections",
            description: "Configure model providers and authenticated connections without rewriting the application.",
            icon: "/logo.svg",
        },
        CapabilityFeature {
            title: "OpenAPI integrations",
            description: "Load tool definitions from Swagger/OpenAPI specs so agents can use external systems at runtime.",
            icon: "/logo.svg",
        },
        CapabilityFeature {
            title: "Channel delivery",
            description: "Receive and send messages through Telegram today, with room to grow the channel surface.",
            icon: "/logo.svg",
        },
        CapabilityFeature {
            title: "Rust-powered sandbox",
            description: "Use the Monty-based Python sandbox for code execution workloads that need stronger operational boundaries.",
            icon: "/logo.svg",
        },
        CapabilityFeature {
            title: "Code Mode support",
            description: "Reduce token overhead by leaning on Code Mode patterns when tool execution is the better path.",
            icon: "/logo.svg",
        },
    ]
}
