use daisy_rsx::{Badge, BadgeColor, BadgeStyle, Button, ButtonScheme, ButtonType, Card, CardBody};
use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

const COMMUNITY_EDITION_URL: &str = "https://github.com/purton-tech/agent-octo";
const APP_URL: &str = "https://app.agent-octo.com";
const BYOK_INPUT_PRICE: &str = "$0.03 / 1M input tokens";
const BYOK_OUTPUT_PRICE: &str = "$0.03 / 1M output tokens";

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "Pricing | agent-octo.com".to_string(),
            description: "Choose between the self-hosted Community Edition and Agent Octo BYOK token-based billing.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Pricing,
            main {
                class: "min-h-screen text-base-content",

                section {
                    class: "px-6 pt-28 pb-16 md:pt-40 lg:px-12",
                    div {
                        class: "mx-auto max-w-6xl space-y-10",
                        div {
                            class: "max-w-3xl space-y-4",
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_color: BadgeColor::Primary,
                                "Pricing"
                            }
                            h1 {
                                class: "text-4xl font-bold lg:text-5xl",
                                "Simple pricing for self-hosting or BYOK usage"
                            }
                            p {
                                class: "text-lg text-base-content/75",
                                "Run Agent Octo yourself with the open-source Community Edition, or use the hosted app with token-based BYOK billing."
                            }
                        }

                        div {
                            class: "grid gap-6 md:grid-cols-2",
                            Card {
                                class: Some("h-full border border-base-300 bg-base-100 shadow-sm".to_string()),
                                CardBody {
                                    class: Some("gap-5".to_string()),
                                    div {
                                        class: "space-y-3",
                                        Badge {
                                            badge_style: BadgeStyle::Outline,
                                            "Community Edition"
                                        }
                                        h2 { class: "text-3xl font-bold", "Open source" }
                                        p {
                                            class: "text-base-content/75",
                                            "Self-host Agent Octo on your own infrastructure and manage your own runtime, storage, and operations."
                                        }
                                    }
                                    div {
                                        class: "space-y-2 text-sm text-base-content/80",
                                        p { "Install it yourself from the public repository." }
                                        p { "Best fit if you want full control over deployment and infrastructure." }
                                        p { "Pricing: free software, self-managed hosting costs." }
                                    }
                                    div {
                                        class: "mt-auto pt-2",
                                        Button {
                                            button_type: ButtonType::Link,
                                            href: COMMUNITY_EDITION_URL.to_string(),
                                            button_scheme: ButtonScheme::Primary,
                                            "View GitHub"
                                        }
                                    }
                                }
                            }

                            Card {
                                class: Some("h-full border border-primary/20 bg-base-100 shadow-sm".to_string()),
                                CardBody {
                                    class: Some("gap-5".to_string()),
                                    div {
                                        class: "space-y-3",
                                        Badge {
                                            badge_style: BadgeStyle::Outline,
                                            badge_color: BadgeColor::Primary,
                                            "BYOK"
                                        }
                                        h2 { class: "text-3xl font-bold", "Hosted app, token-based billing" }
                                        p {
                                            class: "text-base-content/75",
                                            "Bring your own provider keys and pay for Agent Octo usage based on metered token throughput."
                                        }
                                    }
                                    div {
                                        class: "space-y-3",
                                        div {
                                            class: "flex items-baseline justify-between gap-4 border-b border-base-300 pb-3",
                                            span { class: "text-sm uppercase tracking-wide text-base-content/60", "Input" }
                                            span { class: "text-xl font-semibold", "{BYOK_INPUT_PRICE}" }
                                        }
                                        div {
                                            class: "flex items-baseline justify-between gap-4 border-b border-base-300 pb-3",
                                            span { class: "text-sm uppercase tracking-wide text-base-content/60", "Output" }
                                            span { class: "text-xl font-semibold", "{BYOK_OUTPUT_PRICE}" }
                                        }
                                        p {
                                            class: "text-sm text-base-content/70",
                                            "These are the current internal platform rates used when recording usage. Billing is applied against your prepaid balance."
                                        }
                                    }
                                    div {
                                        class: "mt-auto pt-2",
                                        Button {
                                            button_type: ButtonType::Link,
                                            href: APP_URL.to_string(),
                                            button_scheme: ButtonScheme::Primary,
                                            "Login / Signup"
                                        }
                                    }
                                }
                            }
                        }

                        Card {
                            class: Some("border border-base-300 bg-base-100 shadow-sm md:col-span-2".to_string()),
                            CardBody {
                                class: Some("gap-3".to_string()),
                                h2 { class: "text-2xl font-bold", "How usage is charged" }
                                p {
                                    class: "text-base-content/75",
                                    "Agent Octo currently records usage by pricing input and output tokens separately. The charge is computed from your configured provider pricing and deducted from your organization balance in microcents."
                                }
                                p {
                                    class: "text-sm text-base-content/65",
                                    "Current defaults are 3,000,000 microcents per million input tokens and 3,000,000 microcents per million output tokens, which is $0.03 each."
                                }
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    };

    ssg_whiz::render(page)
}
