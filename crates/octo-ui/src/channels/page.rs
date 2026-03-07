#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::agents::AgentCard;
use clorinde::queries::channels_list::ChannelCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(
    org_id: String,
    channels: Vec<ChannelCard>,
    has_telegram_channel: bool,
    agents: Vec<AgentCard>,
) -> String {
    let no_agents = agents.is_empty();
    let connect_action = routes::channels::ConnectTelegram {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Channels".to_string(),
            org_id,
            selected_item: SideBar::Channels,
            header: rsx!(
                div {
                    class: "flex items-center justify-between gap-4",
                    nav {
                        aria_label: "breadcrumb",
                        ol {
                            class: "flex flex-wrap items-center gap-1.5 break-words text-sm sm:gap-2.5",
                            li {
                                class: "items-center gap-1.5 hidden md:block",
                                "Agent Octo"
                            }
                            li { ">" }
                            li { "Channels" }
                        }
                    }
                }
            ),
            SectionIntroduction {
                header: "Channels".to_string(),
                subtitle: "Manage communication channels in your organization.".to_string(),
                is_empty: false,
                empty_text: "".to_string()
            }
            if !has_telegram_channel {
                Card {
                    class: "mt-4",
                    CardHeader {
                        title: "Connect Telegram"
                    }
                    CardBody {
                        p { class: "text-sm text-base-content/70", "To use agents you need a Telegram bot token." }
                        ol {
                            class: "list-decimal pl-5 mt-3 text-sm text-base-content/80",
                            li { "Open BotFather" }
                            li { "Run /newbot" }
                            li { "Paste the token below" }
                        }
                        form {
                            method: "post",
                            action: connect_action,
                            class: "mt-4 flex flex-col gap-3",
                            if no_agents {
                                p {
                                    class: "text-sm text-warning",
                                    "No agents available. Create an agent first."
                                }
                            } else {
                                label { class: "label", "Default Agent" }
                                select {
                                    class: "select select-bordered w-full",
                                    name: "default_agent_id",
                                    required: true,
                                    option {
                                        disabled: true,
                                        selected: true,
                                        value: "",
                                        "Select an agent"
                                    }
                                    for agent in agents.clone() {
                                        option {
                                            value: "{agent.id}",
                                            "{agent.name}"
                                        }
                                    }
                                }
                            }
                            input {
                                class: "input input-bordered w-full",
                                name: "bot_token",
                                placeholder: "Bot Token",
                                required: true
                            }
                            button {
                                class: "btn btn-primary w-fit",
                                r#type: "submit",
                                disabled: no_agents,
                                "Connect Bot"
                            }
                        }
                    }
                }
            }
            if !channels.is_empty() {
                for channel in channels {
                    CardItem {
                        class: None,
                        title: channel.name,
                        description: Some(rsx!(
                            p {
                                "{channel.kind} channel"
                            }
                        )),
                        footer: Some(rsx!(
                            span {
                                "Updated "
                                {channel.updated_at.to_rfc3339()}
                            }
                        )),
                        count_labels: vec![
                            CountLabel {
                                count: 1,
                                label: format!("{} visibility", channel.visibility),
                            }
                        ],
                        action: None
                    }
                }
            }
        }
    };

    render(page)
}
