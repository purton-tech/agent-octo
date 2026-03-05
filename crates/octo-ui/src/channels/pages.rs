#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render,
};
use clorinde::queries::channels_list::ChannelCard;
use dioxus::prelude::*;

pub fn page(org_id: String, channels: Vec<ChannelCard>) -> String {
    let page = rsx! {
        Layout {
            title: "Channels".to_string(),
            org_id,
            selected_item: SideBar::Channels,
            SectionIntroduction {
                header: "Channels".to_string(),
                subtitle: "Manage communication channels in your organization.".to_string(),
                is_empty: channels.is_empty(),
                empty_text: "You have not created any channels yet.".to_string()
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
