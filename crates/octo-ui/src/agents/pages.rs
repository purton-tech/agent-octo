#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render,
};
use clorinde::queries::agents::AgentCard;
use dioxus::prelude::*;

pub fn page(agents: Vec<AgentCard>) -> String {
    let page = rsx! {
        Layout {
            title: "Agents".to_string(),
            selected_item: SideBar::Agents,
            div {
                class: "mx-auto w-full max-w-4xl py-4",
                SectionIntroduction {
                    header: "Agents".to_string(),
                    subtitle: "Manage the assistants you created.".to_string(),
                    is_empty: agents.is_empty(),
                    empty_text: "You have not created any agents yet.".to_string()
                }
                if !agents.is_empty() {
                    for agent in agents {
                        CardItem {
                            class: None,
                            title: agent.name,
                            description: Some(rsx!(
                                p {
                                    class: "line-clamp-2",
                                    if agent.description.is_empty() {
                                        "No description"
                                    } else {
                                        "{agent.description}"
                                    }
                                }
                            )),
                            footer: Some(rsx!(
                                span {
                                    "Updated "
                                    {agent.updated_at.to_rfc3339()}
                                }
                            )),
                            count_labels: vec![
                                CountLabel {
                                    count: 1,
                                    label: format!("{} visibility", agent.visibility),
                                }
                            ],
                            action: None
                        }
                    }
                }
            }
        }
    };

    render(page)
}
