#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::integrations::IntegrationForm;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, integration: Option<IntegrationForm>) -> String {
    let is_edit = integration.is_some();
    let back_href = routes::integrations::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let action_href = routes::integrations::Upsert {
        org_id: org_id.clone(),
    }
    .to_string();

    let id_value = integration
        .as_ref()
        .map(|it| it.id.to_string())
        .unwrap_or_default();
    let visibility_value = integration
        .as_ref()
        .map(|it| it.visibility.clone())
        .unwrap_or_else(|| "private".to_string());
    let spec_value = integration
        .as_ref()
        .map(|it| it.openapi_spec.clone())
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
                        div {
                            label { class: "label", "Visibility" }
                            select {
                                class: "select select-bordered w-full max-w-xs",
                                name: "visibility",
                                option {
                                    value: "private",
                                    selected: visibility_value == "private",
                                    "private"
                                }
                                option {
                                    value: "org",
                                    selected: visibility_value == "org",
                                    "org"
                                }
                            }
                        }
                        div {
                            label { class: "label", "OpenAPI Spec (JSON or YAML)" }
                            textarea {
                                class: "textarea textarea-bordered w-full min-h-72 font-mono text-sm",
                                name: "openapi_spec",
                                required: true,
                                "{spec_value}"
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
