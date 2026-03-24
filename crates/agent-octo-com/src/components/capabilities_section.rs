use dioxus::prelude::*;

struct CapabilityCard {
    title: &'static str,
    body: &'static str,
    footer: &'static str,
    accent_class: &'static str,
    icon: fn() -> Element,
}

const CAPABILITIES: [CapabilityCard; 6] = [
    CapabilityCard {
        title: "Persistent Memory",
        body: "Carry forward instructions, prior decisions, and operational state so agents do not restart each task from zero context.",
        footer: "Context retained",
        accent_class: "text-info",
        icon: memory_icon,
    },
    CapabilityCard {
        title: "Cron Automation",
        body: "Schedule recurring agent runs for maintenance, reporting, and background workflows without bolting on a separate scheduler.",
        footer: "Time-driven",
        accent_class: "text-warning",
        icon: cron_icon,
    },
    CapabilityCard {
        title: "Heartbeat Monitoring",
        body: "Track liveness and runtime health so long-running automation stays observable before failures become invisible drift.",
        footer: "Runtime aware",
        accent_class: "text-success",
        icon: heartbeat_icon,
    },
    CapabilityCard {
        title: "Typed Integrations",
        body: "Connect external systems through inspectable OpenAPI specs that keep tool boundaries explicit and reusable across workflows.",
        footer: "Spec-backed",
        accent_class: "text-primary",
        icon: integrations_icon,
    },
    CapabilityCard {
        title: "Channel Control",
        body: "Route work across channels with clear boundaries for user touchpoints, notifications, and operational handoffs.",
        footer: "Delivery managed",
        accent_class: "text-secondary",
        icon: channels_icon,
    },
    CapabilityCard {
        title: "Sandboxed Execution",
        body: "Run tool calls and agent actions in a controlled runtime designed for isolation, auditability, and recovery.",
        footer: "Execution contained",
        accent_class: "text-error",
        icon: sandbox_icon,
    },
];

#[component]
pub fn CapabilitiesSection() -> Element {
    rsx! {
        section {
            id: "capabilities",
            class: "relative overflow-hidden px-6 pb-24 lg:px-12",

            div {
                class: "relative mx-auto max-w-6xl",
                div {
                    class: "mx-auto max-w-3xl text-center",
                    p {
                        class: "font-mono text-[0.65rem] uppercase tracking-[0.38em] text-primary/75 md:text-xs",
                        "Capabilities"
                    }
                    h2 {
                        class: "mt-4 font-mono text-3xl font-bold tracking-tight text-base-content md:text-5xl",
                        "Built for Agentic Work"
                    }
                    p {
                        class: "mx-auto mt-4 max-w-2xl text-base leading-7 text-base-content/70 md:text-lg",
                        "Agent Octo brings together the runtime primitives needed to schedule work, hold context, monitor execution, and connect agents to real systems."
                    }
                }

                div {
                    class: "mt-12 grid gap-5 md:grid-cols-2 xl:grid-cols-3",
                    for capability in CAPABILITIES.iter() {
                        CapabilityGridCard {
                            title: capability.title,
                            body: capability.body,
                            footer: capability.footer,
                            accent_class: capability.accent_class,
                            icon: (capability.icon)(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CapabilityGridCard(
    title: &'static str,
    body: &'static str,
    footer: &'static str,
    accent_class: &'static str,
    icon: Element,
) -> Element {
    rsx! {
        article {
            class: "group relative overflow-hidden rounded-2xl border border-primary/14 bg-base-100/40 p-5 shadow-[0_18px_56px_rgba(5,10,28,0.18)] backdrop-blur-sm transition duration-300 hover:-translate-y-1 hover:border-primary/26 hover:bg-base-100/52",

            div {
                class: "absolute inset-0 opacity-0 transition duration-300 group-hover:opacity-100",
                div {
                    class: "absolute left-0 top-0 h-28 w-28 -translate-x-6 -translate-y-6 rounded-full bg-primary/8 blur-3xl"
                }
            }

            div {
                class: "relative flex h-full flex-col",
                div {
                    class: format_args!("flex h-11 w-11 items-center justify-center rounded-xl border border-primary/12 bg-base-200/50 shadow-inner shadow-primary/5 {}", accent_class),
                    {icon}
                }

                h3 {
                    class: "mt-5 font-mono text-xl font-semibold leading-tight text-base-content",
                    "{title}"
                }

                p {
                    class: "mt-3 text-sm leading-7 text-base-content/72",
                    "{body}"
                }

                div {
                    class: "mt-auto border-t border-primary/12 pt-4",
                    p {
                        class: "font-mono text-[0.65rem] uppercase tracking-[0.32em] text-primary/40",
                        "{footer}"
                    }
                }
            }
        }
    }
}

fn memory_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "7", y: "7", width: "10", height: "10", rx: "2" }
            path { d: "M10 3v4" }
            path { d: "M14 3v4" }
            path { d: "M10 17v4" }
            path { d: "M14 17v4" }
            path { d: "M17 10h4" }
            path { d: "M17 14h4" }
            path { d: "M3 10h4" }
            path { d: "M3 14h4" }
        }
    )
}

fn cron_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "8" }
            path { d: "M12 8v4l3 2" }
            path { d: "M8 3 6 5" }
            path { d: "M16 3l2 2" }
        }
    )
}

fn heartbeat_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 12h4l2-4 4 8 2-4h6" }
        }
    )
}

fn integrations_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "5", width: "7", height: "6", rx: "1.5" }
            rect { x: "14", y: "13", width: "7", height: "6", rx: "1.5" }
            path { d: "M10 8h4" }
            path { d: "M14 8v8" }
        }
    )
}

fn channels_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M4 7h16" }
            path { d: "M4 12h10" }
            path { d: "M4 17h13" }
            circle { cx: "18", cy: "12", r: "2" }
        }
    )
}

fn sandbox_icon() -> Element {
    rsx!(
        svg {
            class: "h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M6 10V7a6 6 0 0 1 12 0v3" }
            rect { x: "4", y: "10", width: "16", height: "10", rx: "2" }
            path { d: "M12 14v2" }
        }
    )
}
