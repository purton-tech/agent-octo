#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::integration_connections::ConnectableIntegration;
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateConnectionDraft {
    pub integration_id: String,
    pub visibility: String,
    pub api_key: String,
}

pub fn page(
    org_id: String,
    integrations: Vec<ConnectableIntegration>,
    draft: Option<CreateConnectionDraft>,
    error_message: Option<String>,
) -> String {
    let create_action = routes::connections::Create {
        org_id: org_id.clone(),
    }
    .to_string();
    let back_href = routes::connections::Index {
        org_id: org_id.clone(),
    }
    .to_string();

    let selected_integration_id = draft
        .as_ref()
        .map(|d| d.integration_id.clone())
        .unwrap_or_default();
    let visibility_value = draft
        .as_ref()
        .map(|d| d.visibility.clone())
        .unwrap_or_else(|| "private".to_string());
    let api_key_value = draft
        .as_ref()
        .map(|d| d.api_key.clone())
        .unwrap_or_default();

    let page = rsx! {
        Layout {
            title: "Add Connection".to_string(),
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
                            href: Some(back_href.clone()),
                        },
                        BreadcrumbItem {
                            text: "Add Connection".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                Button {
                    button_type: ButtonType::Link,
                    href: back_href,
                    button_style: ButtonStyle::Outline,
                    "Back"
                }
            )),
            SectionIntroduction {
                header: "Add Connection".to_string(),
                subtitle: "Pick an integration and configure credentials. OAuth support can be added later.".to_string(),
                is_empty: false,
                empty_text: "".to_string()
            }
            if let Some(message) = error_message {
                Alert {
                    class: "mt-4".to_string(),
                    alert_color: Some(AlertColor::Error),
                    span { "{message}" }
                }
            }
            Card {
                class: "mt-4",
                CardBody {
                    form {
                        method: "post",
                        action: create_action,
                        class: "flex flex-col gap-4",
                        Fieldset {
                            legend: "Integration".to_string(),
                            help_text: Some("Select which OpenAPI integration this connection should use.".to_string()),
                            Select {
                                name: "integration_id".to_string(),
                                value: Some(selected_integration_id.clone()),
                                SelectOption {
                                    value: "".to_string(),
                                    selected_value: Some(selected_integration_id.clone()),
                                    "Select an integration"
                                }
                                for integration in integrations {
                                    SelectOption {
                                        value: integration.id.to_string(),
                                        selected_value: Some(selected_integration_id.clone()),
                                        "{integration.name}"
                                    }
                                }
                            }
                        }
                        Fieldset {
                            legend: "Visibility".to_string(),
                            help_text: Some("private is only visible to you, org is shared with your org.".to_string()),
                            Select {
                                name: "visibility".to_string(),
                                value: Some(visibility_value.clone()),
                                SelectOption {
                                    value: "private".to_string(),
                                    selected_value: Some(visibility_value.clone()),
                                    "private"
                                }
                                SelectOption {
                                    value: "org".to_string(),
                                    selected_value: Some(visibility_value.clone()),
                                    "org"
                                }
                            }
                        }
                        Fieldset {
                            legend: "API Key".to_string(),
                            help_text: Some("Required only when the selected integration needs authentication.".to_string()),
                            Input {
                                name: "api_key".to_string(),
                                value: Some(api_key_value),
                                placeholder: Some("Paste API key".to_string()),
                                input_type: Some(InputType::Password),
                            }
                        }
                        div {
                            class: "flex justify-end",
                            button {
                                class: "btn btn-primary",
                                r#type: "submit",
                                "Create Connection"
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
