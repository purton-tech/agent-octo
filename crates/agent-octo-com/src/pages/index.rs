use daisy_rsx::marketing::{
    faq_accordian::{Faq, FaqText},
    footer::Footer,
    hero::Hero,
    navigation::Section,
};
use daisy_rsx::timeline::TimelineDirection;
use daisy_rsx::{
    Badge, BadgeColor, BadgeStyle, Card, CardBody, Timeline, TimelineEnd, TimelineItem,
    TimelineMiddle, TimelineStart,
};
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "agent-octo.com".to_string(),
            description: "Agent Octo is a multi-tenant agent platform with runtime integrations, channels, and a Rust-powered sandbox.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "min-h-screen bg-base-100 text-base-content",

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
                                class: Some("border border-base-300 bg-base-100 shadow-sm".to_string()),
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

                Faq {
                    questions: faq_items(),
                    class: Some("mx-auto max-w-4xl px-6 py-16 lg:px-12".to_string())
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }

                script {
                    dangerous_inner_html: include_str!("index.js")
                }
            }
        }
    );

    ssg_whiz::render(page)
}

struct CapabilityFeature {
    title: &'static str,
    description: &'static str,
    icon: &'static str,
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
