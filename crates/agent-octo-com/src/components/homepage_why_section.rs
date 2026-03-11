use daisy_rsx::{Card, CardBody};
use dioxus::prelude::*;

#[component]
pub fn HomepageWhySection() -> Element {
    rsx! {
        section {
            id: "why-agent-octo",
            class: "px-6 py-16 lg:px-12",
            div {
                class: "mx-auto max-w-6xl space-y-8",
                div {
                    class: "max-w-3xl space-y-3",
                    h2 { class: "text-3xl font-bold", "Why teams choose Agent Octo" }
                    p {
                        class: "text-base-content/75",
                        "The source page uses a direct comparison section. Here, the same layout is reframed around the practical reasons to run your own agent stack."
                    }
                }
                div {
                    class: "grid gap-6 lg:grid-cols-3",
                    Card {
                        class: Some("card-md border border-base-300 bg-base-100 shadow-sm".to_string()),
                        CardBody {
                            class: Some("gap-3".to_string()),
                            h3 { class: "card-title", "Own the runtime" }
                            p {
                                "Keep your agent system inside your infrastructure instead of pushing tenant data and tool execution into a hosted black box."
                            }
                        }
                    }
                    Card {
                        class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                        CardBody {
                            class: Some("gap-3".to_string()),
                            h3 { class: "card-title", "Plug in tools at runtime" }
                            p {
                                "Load OpenAPI specs as integrations, connect providers with OAuth2, and expose lightweight tools without hardcoding each workflow."
                            }
                        }
                    }
                    Card {
                        class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
                        CardBody {
                            class: Some("gap-3".to_string()),
                            h3 { class: "card-title", "Serve more than one team" }
                            p {
                                "Agent Octo is built for multi-user, multi-tenant operation so one deployment can support internal teams, clients, or a SaaS footprint."
                            }
                        }
                    }
                }
            }
        }
    }
}
