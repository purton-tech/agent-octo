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

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectTelegramDraft {
    pub bot_token: String,
    pub default_agent_id: String,
}

pub fn page(
    org_id: String,
    balance_label: String,
    channels: Vec<ChannelCard>,
    has_telegram_channel: bool,
    agents: Vec<AgentCard>,
    connect_draft: Option<ConnectTelegramDraft>,
    connect_error: Option<String>,
) -> String {
    let no_agents = agents.is_empty();
    let bot_token_value = connect_draft
        .as_ref()
        .map(|d| d.bot_token.clone())
        .unwrap_or_default();
    let default_agent_value = connect_draft
        .as_ref()
        .map(|d| d.default_agent_id.clone())
        .unwrap_or_default();
    let connect_action = routes::channels::ConnectTelegram {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Channels".to_string(),
            org_id,
            balance_label,
            selected_item: SideBar::Channels,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Channels".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: None,
            SectionIntroduction {
                header: "Channels".to_string(),
                subtitle: "Manage communication channels in your organization.".to_string(),
                is_empty: false,
                empty_text: "".to_string()
            }
            if let Some(message) = connect_error {
                Alert {
                    class: "mt-4".to_string(),
                    alert_color: Some(AlertColor::Error),
                    span { "{message}" }
                }
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
                                        selected: default_agent_value.is_empty(),
                                        value: "",
                                        "Select an agent"
                                    }
                                    for agent in agents.clone() {
                                        option {
                                            value: "{agent.id}",
                                            selected: default_agent_value == agent.id.to_string(),
                                            "{agent.name}"
                                        }
                                    }
                                }
                            }
                            input {
                                class: "input input-bordered w-full",
                                name: "bot_token",
                                placeholder: "Bot Token",
                                value: bot_token_value,
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
