#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::integrations::IntegrationCard;
use daisy_rsx::*;
use dioxus::prelude::*;

pub fn page(org_id: String, integrations: Vec<IntegrationCard>) -> String {
    let new_href = routes::integrations::New {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Integrations".to_string(),
            org_id: org_id.clone(),
            selected_item: SideBar::Integrations,
            header: rsx!(
                div {
                    class: "flex items-center justify-between gap-4",
                    nav {
                        aria_label: "breadcrumb",
                        ol {
                            class: "flex flex-wrap items-center gap-1.5 break-words text-sm sm:gap-2.5",
                            li { class: "items-center gap-1.5 hidden md:block", "Agent Octo" }
                            li { ">" }
                            li { "Integrations" }
                        }
                    }
                    Button {
                        button_type: ButtonType::Link,
                        href: new_href,
                        button_scheme: ButtonScheme::Primary,
                        "Add OpenAPI Spec"
                    }
                }
            ),
            div {
                class: "p-4 max-w-5xl w-full mx-auto flex flex-col gap-6",
                SectionIntroduction {
                    header: "OpenAPI Specs".to_string(),
                    subtitle: "Manage the OpenAPI specifications available to your organization.".to_string(),
                    is_empty: integrations.is_empty(),
                    empty_text: "No OpenAPI specs available yet. Add one to get started.".to_string(),
                }

                if !integrations.is_empty() {
                    Card {
                        class: "has-data-table",
                        CardHeader { title: "Available Specs" }
                        CardBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Visibility" }
                                        th { "Updated" }
                                        th { class: "text-right", "Actions" }
                                    }
                                }
                                tbody {
                                    for integration in &integrations {
                                        tr {
                                            td {
                                                div { class: "font-medium", "{integration.name}" }
                                                if !integration.description.is_empty() {
                                                    div { class: "text-sm text-base-content/70", "{integration.description}" }
                                                }
                                            }
                                            td {
                                                span {
                                                    class: if integration.visibility == "org" {
                                                        "badge badge-success badge-outline"
                                                    } else {
                                                        "badge badge-ghost"
                                                    },
                                                    "{integration.visibility}"
                                                }
                                            }
                                            td { class: "text-sm text-base-content/70", "{integration.updated_at.to_rfc3339()}" }
                                            td {
                                                class: "text-right",
                                                div {
                                                    class: "flex justify-end gap-2",
                                                    Button {
                                                        button_type: ButtonType::Link,
                                                        button_style: ButtonStyle::Outline,
                                                        href: routes::integrations::Edit {
                                                            org_id: org_id.clone(),
                                                            id: integration.id.to_string(),
                                                        }.to_string(),
                                                        "Edit"
                                                    }
                                                    form {
                                                        method: "post",
                                                        action: routes::integrations::Delete {
                                                            org_id: org_id.clone(),
                                                            id: integration.id.to_string(),
                                                        }.to_string(),
                                                        button {
                                                            class: "btn btn-warning",
                                                            r#type: "submit",
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
