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

pub fn index_page(org_id: String, providers: Vec<ProviderConnectionCard>) -> String {
    let new_href = routes::providers::New {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Providers".to_string(),
            org_id,
            selected_item: SideBar::Providers,
            div {
                class: "flex items-start justify-between gap-4",
                SectionIntroduction {
                    header: "Model Providers".to_string(),
                    subtitle: "Manage provider connections used by your agents.".to_string(),
                    is_empty: providers.is_empty(),
                    empty_text: "No providers configured yet.".to_string()
                }
                Button {
                    button_type: ButtonType::Link,
                    href: new_href,
                    button_scheme: ButtonScheme::Primary,
                    "Add Provider"
                }
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

pub fn new_page(org_id: String) -> String {
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
            selected_item: SideBar::Providers,
            content_class: Some("p-4 max-w-5xl w-full mx-auto".to_string()),
            div {
                class: "flex items-start justify-between gap-4",
                SectionIntroduction {
                    header: "Add Provider".to_string(),
                    subtitle: "Pick a provider and add credentials in the popup form.".to_string(),
                    is_empty: false,
                    empty_text: "".to_string()
                }
                Button {
                    button_type: ButtonType::Link,
                    href: back_href,
                    button_style: ButtonStyle::Outline,
                    "Back"
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
                                label { class: "label", "Display Name" }
                                input {
                                    class: "input input-bordered w-full",
                                    name: "display_name",
                                    value: title,
                                    required: true
                                }
                                label { class: "label", "API Key" }
                                input {
                                    class: "input input-bordered w-full",
                                    name: "api_key",
                                    placeholder: "sk-...",
                                    required: true
                                }
                                label { class: "label", "Base URL (optional)" }
                                input {
                                    class: "input input-bordered w-full",
                                    name: "base_url",
                                    placeholder: "https://..."
                                }
                                label { class: "label", "Default Model (optional)" }
                                input {
                                    class: "input input-bordered w-full",
                                    name: "default_model",
                                    placeholder: "gpt-4o-mini"
                                }
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
