use dioxus::prelude::*;

use crate::components::site_card::{CardIconShell, SiteCard};

const SPECS_ITEMS: [&str; 4] = [
    "OpenAPI specs define typed runtime integrations",
    "External services stay inspectable before execution",
    "Runtime connections map cleanly to agent workflows",
    "Interfaces stay portable across self-hosted deployments",
];

const SANDBOX_ITEMS: [&str; 4] = [
    "Sandboxed execution isolates operational side effects",
    "Channels and providers stay under explicit control",
    "Task ordering and guardrails reduce blind automation",
    "One platform handles execution, review, and recovery",
];

#[component]
pub fn PlatformSection() -> Element {
    rsx! {
        section {
            id: "platform-overview",
            class: "relative overflow-hidden px-6 pb-20 lg:px-12",

            div {
                class: "relative mx-auto max-w-6xl",

                div {
                    class: "mx-auto max-w-3xl text-center",
                    p {
                        class: "font-mono text-[0.65rem] uppercase tracking-[0.38em] text-primary/80 md:text-xs",
                        "Before you execute"
                    }
                    h2 {
                        class: "mt-4 font-mono text-3xl font-bold tracking-tight text-base-content md:text-5xl",
                        "The Orchestration Layer"
                    }
                    p {
                        class: "mx-auto mt-4 max-w-2xl text-base leading-7 text-base-content/72 md:text-lg",
                        "Agent Octo gives teams a structured way to connect external systems and run work inside a controlled runtime, so automation stays visible before it becomes operational."
                    }
                }

                div {
                    class: "mt-12 grid gap-6 lg:grid-cols-2",
                    PlatformCard {
                        step_label: "Step 01",
                        title: "Specs Become Runtime Integrations",
                        body: "Agent Octo starts with typed OpenAPI definitions so external systems can be inspected, wired, and reused without hiding the interface layer behind prompts.",
                        footer_label: "Integration-ready",
                        icon: rsx!(
                            svg {
                                class: "h-8 w-8",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M8 3H5a2 2 0 0 0-2 2v3" }
                                path { d: "M16 3h3a2 2 0 0 1 2 2v3" }
                                path { d: "M8 21H5a2 2 0 0 1-2-2v-3" }
                                path { d: "M16 21h3a2 2 0 0 0 2-2v-3" }
                                rect { x: "8", y: "8", width: "8", height: "8", rx: "1.5" }
                            }
                        ),
                        items: &SPECS_ITEMS,
                    }
                    PlatformCard {
                        step_label: "Step 02",
                        title: "Execution Stays Sandboxed",
                        body: "The runtime layer keeps tool use, channels, and provider access inside explicit boundaries, which makes automation auditable and recoverable when real work starts moving.",
                        footer_label: "Control built in",
                        icon: rsx!(
                            svg {
                                class: "h-8 w-8",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.75",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 10V7a6 6 0 0 1 12 0v3" }
                                rect { x: "4", y: "10", width: "16", height: "10", rx: "2" }
                                path { d: "M12 14v2" }
                            }
                        ),
                        items: &SANDBOX_ITEMS,
                    }
                }
            }
        }
    }
}

#[component]
fn PlatformCard(
    step_label: &'static str,
    title: &'static str,
    body: &'static str,
    footer_label: &'static str,
    icon: Element,
    items: &'static [&'static str],
) -> Element {
    rsx! {
        SiteCard {
            class: Some("h-full".to_string()),
            body_class: Some("h-full gap-0 p-6 md:p-7".to_string()),
            interactive: Some(false),

            div {
                class: "flex items-start justify-between gap-4",
                p {
                    class: "font-mono text-[0.65rem] uppercase tracking-[0.32em] text-primary/65 md:text-xs",
                    "{step_label}"
                }
                CardIconShell {
                    class: Some("text-primary".to_string()),
                    {icon}
                }
            }

            h3 {
                class: "mt-6 max-w-sm font-mono text-2xl font-semibold leading-tight text-base-content md:text-[1.7rem]",
                "{title}"
            }

            p {
                class: "mt-4 max-w-xl text-base leading-7 text-base-content/72",
                "{body}"
            }

            ul {
                class: "mt-6 space-y-3 text-sm text-base-content/78 md:text-[0.95rem]",
                for item in items.iter() {
                    li {
                        class: "flex items-start gap-3",
                        span {
                            class: "mt-0.5 inline-flex h-4.5 w-4.5 shrink-0 items-center justify-center rounded-sm border border-primary/30 text-primary",
                            svg {
                                class: "h-3 w-3",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M20 6 9 17l-5-5" }
                            }
                        }
                        span {
                            class: "leading-6",
                            "{item}"
                        }
                    }
                }
            }

            div {
                class: "mt-7 border-t border-base-300 pt-4",
                p {
                    class: "font-mono text-xs uppercase tracking-[0.35em] text-base-content/45",
                    "{footer_label}"
                }
            }
        }
    }
}
