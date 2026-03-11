use dioxus::prelude::*;

struct StoryScene {
    title: &'static str,
    eyebrow: &'static str,
    messages: &'static [StoryMessage],
}

struct StoryMessage {
    side: &'static str,
    text: &'static str,
}

#[component]
pub fn HomepageStorySection() -> Element {
    rsx! {
        section {
            id: "agent-story",
            class: "px-6 py-16 lg:px-12",

            div {
                class: "mx-auto max-w-6xl space-y-8",

                div {
                    class: "max-w-3xl space-y-3",
                    p {
                        class: "text-sm font-semibold uppercase tracking-[0.2em] text-primary/80",
                        "See the product in motion"
                    }
                    h2 {
                        class: "text-3xl font-bold tracking-tight md:text-4xl",
                        "Watch an agent workflow unfold as you scroll"
                    }
                    p {
                        class: "text-base-content/75",
                        "Instead of a comparison grid, this section shows the operating model: inbound chat, agent reasoning, and a handoff into tools and people when the work needs it."
                    }
                }

                div {
                    class: "space-y-0",
                    for scene in story_scenes() {
                        section {
                            class: "octo-story",
                            "data-octo-story": "scene",
                            div {
                                class: "octo-story__sticky",
                                div {
                                    class: "octo-story__frame",
                                    div {
                                        class: "octo-story__header",
                                        div {
                                            class: "space-y-1",
                                            p {
                                                class: "text-xs font-semibold uppercase tracking-[0.18em] text-primary/80",
                                                "{scene.eyebrow}"
                                            }
                                            h3 {
                                                class: "text-lg font-semibold md:text-xl",
                                                "{scene.title}"
                                            }
                                        }
                                        span {
                                            class: "badge badge-outline",
                                            "{scene.messages.len()} steps"
                                        }
                                    }

                                    div {
                                        class: "octo-story__viewport",
                                        div {
                                            class: "octo-story__messages",
                                            for message in scene.messages {
                                                div {
                                                    class: "octo-story__message {message.side}",
                                                    "data-octo-story": "message",
                                                    "{message.text}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn story_scenes() -> Vec<StoryScene> {
    vec![
        StoryScene {
            eyebrow: "Telegram intake",
            title: "A customer message becomes a structured task",
            messages: &[
                StoryMessage {
                    side: "left",
                    text: "A customer says their order failed and they need help today.",
                },
                StoryMessage {
                    side: "right",
                    text: "Agent Octo classifies the issue, extracts the order reference, and opens the support workflow.",
                },
                StoryMessage {
                    side: "left",
                    text: "Can it keep the thread in Telegram instead of switching tools?",
                },
                StoryMessage {
                    side: "right",
                    text: "Yes. The response stays in-channel while the runtime prepares the next action in the background.",
                },
                StoryMessage {
                    side: "right",
                    text: "Urgent cases can be escalated to a human without losing the conversation context.",
                },
            ],
        },
        StoryScene {
            eyebrow: "Runtime tools",
            title: "The agent reaches into connected systems when it needs context",
            messages: &[
                StoryMessage {
                    side: "left",
                    text: "Check the CRM record and see whether this account is priority support.",
                },
                StoryMessage {
                    side: "right",
                    text: "The OpenAPI tool spec is loaded at runtime and the account profile is fetched through an authenticated connection.",
                },
                StoryMessage {
                    side: "right",
                    text: "The record shows enterprise support, prior incidents, and the account owner.",
                },
                StoryMessage {
                    side: "left",
                    text: "Good. Route billing details to the right team and keep the reply concise.",
                },
                StoryMessage {
                    side: "right",
                    text: "The draft response is prepared and the ownership handoff is attached for review.",
                },
            ],
        },
        StoryScene {
            eyebrow: "Human + agent operations",
            title: "Automation handles the repeatable work while operators stay in control",
            messages: &[
                StoryMessage {
                    side: "left",
                    text: "A new inbound lead asked for pricing, compliance details, and a live demo.",
                },
                StoryMessage {
                    side: "right",
                    text: "Agent Octo sends the right materials, records the lead, and flags the request for follow-up.",
                },
                StoryMessage {
                    side: "left",
                    text: "Can we stop it from sending the final offer automatically?",
                },
                StoryMessage {
                    side: "right",
                    text: "Yes. Sensitive steps can remain human-approved while the repetitive work stays automated.",
                },
                StoryMessage {
                    side: "right",
                    text: "That gives teams faster execution without giving up operational control.",
                },
            ],
        },
    ]
}
