#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    layout::{Layout, SideBar},
    render,
};
use clorinde::queries::agents::AgentCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(agents: Vec<AgentCard>) -> String {
    let page = rsx! {
        Layout {
            title: "Agents".to_string(),
            selected_item: SideBar::Agents,
            div {
                class: "mx-auto w-full max-w-4xl py-4",
                if agents.is_empty() {
                    Card {
                        class: "p-6 mt-4",
                        CardBody {
                            h2 { class: "card-title", "No Agents Yet" }
                            p { class: "text-base-content/70", "You have not created any agents yet." }
                        }
                    }
                } else {
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
