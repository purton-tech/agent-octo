#![allow(non_snake_case)]

use crate::{
    components::card_item::{CardItem, CountLabel},
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::providers::ProviderConnectionCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, providers: Vec<ProviderConnectionCard>) -> String {
    let new_href = routes::providers::New {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Providers".to_string(),
            org_id,
            selected_item: SideBar::Providers,
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
                            li { "Providers" }
                        }
                    }
                    Button {
                        button_type: ButtonType::Link,
                        href: new_href,
                        button_scheme: ButtonScheme::Primary,
                        "Add Provider"
                    }
                }
            ),
            SectionIntroduction {
                header: "Model Providers".to_string(),
                subtitle: "Manage provider connections used by your agents.".to_string(),
                is_empty: providers.is_empty(),
                empty_text: "No providers configured yet.".to_string()
            }
            if !providers.is_empty() {
                for provider in providers {
                    CardItem {
                        class: None,
                        title: provider.display_name,
                        description: Some(rsx!(
                            div {
                                class: "flex flex-col gap-1",
                                p {
                                    class: "capitalize",
                                    "{provider.provider_kind}"
                                }
                                p {
                                    class: "text-sm text-base-content/70",
                                    if provider.default_model.is_empty() {
                                        "Default model: not set"
                                    } else {
                                        "Default model: {provider.default_model}"
                                    }
                                }
                            }
                        )),
                        footer: Some(rsx!(
                            span {
                                "Updated "
                                {provider.updated_at.to_rfc3339()}
                            }
                        )),
                        count_labels: vec![
                            CountLabel {
                                count: 1,
                                label: "connection".to_string(),
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
