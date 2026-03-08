#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::integration_connections::IntegrationConnectionCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, connections: Vec<IntegrationConnectionCard>) -> String {
    let new_href = routes::connections::New {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Connections".to_string(),
            org_id,
            selected_item: SideBar::Connections,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Connections".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                Button {
                    button_type: ButtonType::Link,
                    href: new_href,
                    button_scheme: ButtonScheme::Primary,
                    "Add Connection"
                }
            )),
            SectionIntroduction {
                header: "Connections".to_string(),
                subtitle: "Manage authenticated connections for your integrations.".to_string(),
                is_empty: connections.is_empty(),
                empty_text: "No connections configured yet.".to_string()
            }
            if !connections.is_empty() {
                for connection in connections {
                    CardItem {
                        class: None,
                        title: connection.name,
                        description: Some(rsx!(
                            div {
                                class: "flex flex-col gap-1",
                                p { "{connection.integration_name}" }
                                p {
                                    class: "text-sm text-base-content/70",
                                    "Auth: {connection.auth_type}"
                                }
                            }
                        )),
                        footer: Some(rsx!(
                            span {
                                "Updated "
                                {connection.updated_at.to_rfc3339()}
                            }
                        )),
                        count_labels: vec![
                            CountLabel {
                                count: 1,
                                label: format!("{} visibility", connection.visibility),
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
