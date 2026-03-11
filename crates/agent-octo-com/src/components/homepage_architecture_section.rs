use daisy_rsx::timeline::TimelineDirection;
use daisy_rsx::{
    Card, CardBody, Timeline, TimelineEnd, TimelineItem, TimelineMiddle, TimelineStart,
};
use dioxus::prelude::*;

#[component]
pub fn HomepageArchitectureSection() -> Element {
    rsx! {
        section {
            id: "architecture",
            class: "px-6 py-16 lg:px-12",
            div {
                class: "mx-auto max-w-6xl space-y-8",
                div {
                    class: "max-w-3xl space-y-3",
                    h2 { class: "text-3xl font-bold", "How Agent Octo is wired" }
                    p {
                        class: "text-base-content/75",
                        "Like the source page’s systems section, this breaks the product down into a sequence and a few architectural guarantees."
                    }
                }
                Timeline {
                    class: Some("timeline-compact".to_string()),
                    direction: TimelineDirection::Vertical,
                    compact: true,
                    TimelineItem {
                        TimelineStart {
                            boxed: true,
                            strong { "1. Connect a channel" }
                            p { class: "text-sm text-base-content/70", "Start with Telegram or add more delivery paths over time." }
                        }
                        TimelineMiddle { div { class: "h-3 w-3 rounded-full bg-primary" } }
                        TimelineEnd {
                            boxed: true,
                            strong { "2. Route into an org" }
                            p { class: "text-sm text-base-content/70", "Messages stay scoped to the correct organization, providers, and integrations." }
                        }
                    }
                    TimelineItem {
                        TimelineStart {
                            boxed: true,
                            strong { "3. Call tools and models" }
                            p { class: "text-sm text-base-content/70", "Agents can use provider connections, runtime integrations, and sandboxed code execution." }
                        }
                        TimelineMiddle { div { class: "h-3 w-3 rounded-full bg-secondary" } }
                        TimelineEnd {
                            boxed: true,
                            strong { "4. Ship the answer back" }
                            p { class: "text-sm text-base-content/70", "Responses return through the channel with the same operational boundaries you configured." }
                        }
                    }
                }
                div {
                    class: "grid gap-6 md:grid-cols-3",
                    Card {
                        class: Some("border border-base-300 bg-base-200/40 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Typed backend" }
                            p { "Axum, Rust on Nails, and generated database queries keep the core path explicit and testable." }
                        }
                    }
                    Card {
                        class: Some("border border-base-300 bg-base-200/40 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Runtime integrations" }
                            p { "OpenAPI specs can be loaded as integrations so the tool surface can change without recompiling the app." }
                        }
                    }
                    Card {
                        class: Some("border border-base-300 bg-base-200/40 shadow-sm".to_string()),
                        CardBody {
                            h3 { class: "card-title", "Operational control" }
                            p { "Multi-tenant org boundaries, provider setup, and channel routing are managed directly in the application." }
                        }
                    }
                }
            }
        }
    }
}
