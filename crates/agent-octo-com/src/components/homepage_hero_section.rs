use dioxus::prelude::*;

#[component]
pub fn HomepageHeroSection() -> Element {
    rsx! {
        section {
            id: "hero",
            class: "relative overflow-hidden pt-28 pb-28 md:pt-40 md:pb-36",

            div {
                class: "pointer-events-none absolute inset-0",
                div {
                    class: "absolute top-0 left-1/2 h-[600px] w-[800px] -translate-x-1/2 rounded-full bg-primary/8 blur-3xl"
                }
            }

            div {
                class: "relative mx-auto max-w-5xl px-6",
                div {
                    class: "flex flex-col items-center text-center",

                    h1 {
                        class: "text-5xl font-extrabold tracking-tight md:text-7xl",
                        span {
                            class: "underline decoration-primary underline-offset-8 decoration-8",
                            "Agent Octo"
                        }
                    }

                    h2 {
                        class: "mt-28 max-w-3xl text-4xl font-extrabold leading-tight tracking-tight md:text-6xl",
                        "Manage your life with Agentic AI. "
                        span {
                            class: "bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent",
                            "Connected. Controlled. Yours."
                        }
                    }

                    p {
                        class: "mt-6 max-w-2xl text-lg text-base-content/70 md:text-xl",
                        "Agent Octo gives you a multi-tenant agent platform with runtime OpenAPI integrations, provider connections, channels, and a Rust-powered sandbox built for real operational control."
                    }

                    div {
                        class: "mt-10 flex flex-wrap items-center justify-center gap-4",
                        a {
                            class: "btn btn-primary",
                            href: "/open-api-specs/",
                            svg {
                                class: "h-5 w-5",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                path {
                                    d: "M3 5.25A2.25 2.25 0 0 1 5.25 3h13.5A2.25 2.25 0 0 1 21 5.25v13.5A2.25 2.25 0 0 1 18.75 21H5.25A2.25 2.25 0 0 1 3 18.75V5.25Zm4.5.75a.75.75 0 0 0 0 1.5h9a.75.75 0 0 0 0-1.5h-9Zm0 5.25a.75.75 0 0 0 0 1.5h9a.75.75 0 0 0 0-1.5h-9Zm0 5.25a.75.75 0 0 0 0 1.5h5.25a.75.75 0 0 0 0-1.5H7.5Z"
                                }
                            }
                            "Explore Specs"
                        }
                        a {
                            class: "btn btn-outline",
                            href: "https://github.com/purton-tech/agent-octo",
                            svg {
                                class: "h-5 w-5",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                path {
                                    d: "M12 0C5.373 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386C24 5.373 18.627 0 12 0z"
                                }
                            }
                            "View GitHub"
                        }
                    }

                    div {
                        class: "mt-8 flex items-center gap-2 text-sm text-base-content/55",
                        svg {
                            class: "h-4 w-4",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M12 .587l3.668 7.431L24 9.168l-6 5.85 1.416 8.256L12 19.187l-7.416 4.087L6 15.018 0 9.168l8.332-1.15z"
                            }
                        }
                        span {
                            "Runtime integrations, channels, and sandboxed execution in one stack"
                        }
                    }
                }
            }
        }
    }
}
