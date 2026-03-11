#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use daisy_rsx::*;
use dioxus::prelude::*;

const PROVIDER_OPTIONS: [(&str, &str, &str); 3] = [
    (
        "openai",
        "OpenAI",
        "Configure OpenAI API access for model inference.",
    ),
    (
        "anthropic",
        "Anthropic",
        "Configure Anthropic API access for Claude models.",
    ),
    (
        "gemini",
        "Google Gemini",
        "Configure Gemini API access for Google models.",
    ),
];

#[derive(Debug, Clone, PartialEq)]
pub struct CreateProviderDraft {
    pub provider_kind: String,
    pub api_key: String,
}

pub fn page(
    org_id: String,
    balance_label: String,
    draft: Option<CreateProviderDraft>,
    error_message: Option<String>,
) -> String {
    let create_action = routes::providers::Create {
        org_id: org_id.clone(),
    }
    .to_string();
    let back_href = routes::providers::Index {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Add Provider".to_string(),
            org_id,
            balance_label,
            selected_item: SideBar::Providers,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Providers".to_string(),
                            href: Some(back_href.clone()),
                        },
                        BreadcrumbItem {
                            text: "Add Provider".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: Some(rsx!(
                    Button {
                        button_type: ButtonType::Link,
                        href: back_href.clone(),
                        button_style: ButtonStyle::Outline,
                        "Back"
                    }
            )),
            SectionIntroduction {
                header: "Add Provider".to_string(),
                subtitle: "Pick a provider and add an API key. Agents without LLM config will be attached automatically.".to_string(),
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
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4 mt-4",
                for (kind, title, desc) in PROVIDER_OPTIONS {
                    Card {
                        class: "card-border bg-base-100",
                        CardBody {
                            h2 { class: "card-title", "{title}" }
                            p { class: "text-sm text-base-content/70", "{desc}" }
                            div {
                                class: "card-actions justify-end mt-4",
                                Button {
                                    button_scheme: ButtonScheme::Primary,
                                    popover_target: format!("provider-modal-{kind}"),
                                    "Configure"
                                }
                            }
                        }
                    }
                    Modal {
                        trigger_id: format!("provider-modal-{kind}"),
                        submit_action: create_action.clone(),
                        ModalBody {
                            h3 { class: "text-lg font-semibold", "Configure {title}" }
                            p { class: "text-sm text-base-content/70 mt-1", "Add credentials for {title}." }
                            input {
                                r#type: "hidden",
                                name: "provider_kind",
                                value: kind
                            }
                            div {
                                class: "mt-4 flex flex-col gap-3",
                                label { class: "label", "API Key" }
                                input {
                                    class: "input input-bordered w-full",
                                    name: "api_key",
                                    placeholder: "sk-...",
                                    value: draft
                                        .as_ref()
                                        .and_then(|d| if d.provider_kind == kind { Some(d.api_key.clone()) } else { None })
                                        .unwrap_or_default(),
                                    required: true
                                }
                                p { class: "text-xs text-base-content/70", "Uses the provider's required default model metadata." }
                            }
                            ModalAction {
                                Button {
                                    class: "cancel-modal",
                                    button_scheme: ButtonScheme::Warning,
                                    "Cancel"
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Primary,
                                    "Save Provider"
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
