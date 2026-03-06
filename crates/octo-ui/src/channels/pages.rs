#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::channels_list::ChannelCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, channels: Vec<ChannelCard>, has_telegram_channel: bool) -> String {
    let connect_action = routes::channels::ConnectTelegram {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Channels".to_string(),
            org_id,
            selected_item: SideBar::Channels,
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
                            input {
                                class: "input input-bordered w-full",
                                name: "bot_token",
                                placeholder: "Bot Token",
                                required: true
                            }
                            button {
                                class: "btn btn-primary w-fit",
                                r#type: "submit",
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
