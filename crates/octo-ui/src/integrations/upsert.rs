#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::integrations::IntegrationForm;
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UpsertDraft {
    pub id: Option<String>,
    pub visibility: String,
    pub openapi_spec: String,
}

pub fn page(
    org_id: String,
    balance_label: String,
    integration: Option<IntegrationForm>,
    draft: Option<UpsertDraft>,
    error_message: Option<String>,
) -> String {
    let is_edit = integration.is_some() || draft.as_ref().and_then(|d| d.id.clone()).is_some();
    let back_href = routes::integrations::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let action_href = routes::integrations::Upsert {
        org_id: org_id.clone(),
    }
    .to_string();

    let id_value = draft
        .as_ref()
        .and_then(|it| it.id.clone())
        .or_else(|| integration.as_ref().map(|it| it.id.to_string()))
        .unwrap_or_default();
    let visibility_value = draft
        .as_ref()
        .map(|it| it.visibility.clone())
        .or_else(|| integration.as_ref().map(|it| it.visibility.clone()))
        .unwrap_or_else(|| "private".to_string());
    let spec_value = draft
        .as_ref()
        .map(|it| it.openapi_spec.clone())
        .or_else(|| integration.as_ref().map(|it| it.openapi_spec.clone()))
        .unwrap_or_default();

    let page_title = if is_edit {
        "Edit OpenAPI Spec"
    } else {
        "Add OpenAPI Spec"
    };

    let page = rsx! {
        Layout {
            title: page_title.to_string(),
            org_id: org_id.clone(),
            balance_label,
            selected_item: SideBar::Integrations,
            content_class: Some("p-4 max-w-5xl w-full mx-auto".to_string()),
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Integrations".to_string(),
                            href: Some(back_href.clone()),
                        },
                        BreadcrumbItem {
                            text: page_title.to_string(),
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
                header: page_title.to_string(),
                subtitle: "Paste an OpenAPI JSON or YAML document. We validate it before saving.".to_string(),
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
                        action: action_href,
                        class: "flex flex-col gap-4",
                        if is_edit {
                            input {
                                r#type: "hidden",
                                name: "id",
                                value: id_value
                            }
                        }
                        Fieldset {
                            legend: "Visibility".to_string(),
                            help_text: Some("Choose who can access this OpenAPI spec.".to_string()),
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
                            legend: "OpenAPI Spec (JSON or YAML)".to_string(),
                            help_text: Some("info.title in the spec is used as the display name.".to_string()),
                            TextArea {
                                name: "openapi_spec".to_string(),
                                class: Some("w-full min-h-72 font-mono text-sm".to_string()),
                                value: Some(spec_value),
                                required: Some(true),
                            }
                        }
                        div {
                            class: "flex justify-end",
                            button {
                                class: "btn btn-primary",
                                r#type: "submit",
                                if is_edit { "Save Changes" } else { "Create Spec" }
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
