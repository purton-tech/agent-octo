use daisy_rsx::{Badge, BadgeColor, BadgeStyle};
use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

use crate::components::site_card::SiteCard;
use crate::open_api_specs::{Endpoint, IntegrationSpec};

pub fn index_page(integrations: &[IntegrationSpec]) -> String {
    let page = rsx! {
        Layout {
            title: "OpenAPI Specs | agent-octo.com".to_string(),
            description: "Browse OpenAPI integration specs for Agent Octo and inspect the endpoints each spec exposes.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::McpServers,
            main {
                class: "min-h-screen text-base-content",

                section {
                    class: "px-6 pt-28 pb-16 md:pt-40 lg:px-12",
                    div {
                        class: "mx-auto max-w-6xl space-y-10",
                        div {
                            class: "max-w-3xl space-y-4",
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_color: BadgeColor::Primary,
                                "OpenAPI specs"
                            }
                            h1 {
                                class: "font-mono text-4xl font-bold tracking-tight lg:text-5xl",
                                "Browse integrations Agent Octo can understand"
                            }
                            p {
                                class: "text-lg text-base-content/75",
                                "Each card below maps to a YAML OpenAPI spec. Open a spec to inspect its methods, parameters, responses, and copy the source directly."
                            }
                        }
                        div {
                            class: "grid gap-6 md:grid-cols-2 xl:grid-cols-3",
                            if integrations.is_empty() {
                                div {
                                    class: "col-span-full rounded-box border border-dashed border-base-300 p-10 text-center text-base-content/70",
                                    "No specs are available yet."
                                }
                            } else {
                                for integration in integrations {
                                    a {
                                        class: "block h-full",
                                        href: "{integration.detail_path()}",
                                        SiteCard {
                                            class: Some("h-full".to_string()),
                                            body_class: Some("h-full gap-4".to_string()),
                                            interactive: Some(true),
                                            div {
                                                class: "flex items-start gap-4",
                                                SpecLogo {
                                                    logo_url: integration.logo_url.clone(),
                                                    title: integration.title.clone(),
                                                    class: Some("h-14 w-14 shrink-0 rounded-box border border-base-300 bg-base-200/40 p-2".to_string())
                                                }
                                                div {
                                                    class: "min-w-0 space-y-2",
                                                    h2 {
                                                        class: "card-title line-clamp-2 font-mono text-xl tracking-tight",
                                                        "{integration.title}"
                                                    }
                                                    p {
                                                        class: "line-clamp-3 text-sm text-base-content/75",
                                                        "{integration.description.as_deref().unwrap_or(\"No description provided.\")}"
                                                    }
                                                }
                                            }
                                            div {
                                                class: "mt-auto flex flex-wrap gap-2",
                                                if let Some(version) = integration.version.as_deref() {
                                                    Badge {
                                                        badge_style: BadgeStyle::Outline,
                                                        "v{version}"
                                                    }
                                                }
                                                Badge {
                                                    badge_style: BadgeStyle::Outline,
                                                    badge_color: BadgeColor::Primary,
                                                    "{integration.endpoints.len()} methods"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    };

    ssg_whiz::render(page)
}

pub fn detail_page(integration: &IntegrationSpec) -> String {
    let page = rsx! {
        Layout {
            title: format!("{} | OpenAPI Spec | agent-octo.com", integration.title),
            description: integration
                .description
                .clone()
                .unwrap_or_else(|| "OpenAPI specification details for Agent Octo.".to_string()),
            image: integration.logo_url.clone().or_else(|| Some("/logo.svg".to_string())),
            mobile_menu: None,
            section: Section::McpServers,
            main {
                class: "min-h-screen text-base-content",

                section {
                    class: "px-6 pt-28 pb-16 md:pt-40 lg:px-12",
                    div {
                        class: "mx-auto max-w-6xl space-y-10",
                        nav {
                            a {
                                class: "link link-primary",
                                href: "/open-api-specs/",
                                "Back to all specs"
                            }
                        }
                        SiteCard {
                            body_class: Some("gap-5".to_string()),
                            interactive: Some(false),
                            div {
                                class: "flex flex-col gap-5 md:flex-row md:items-start",
                                SpecLogo {
                                    logo_url: integration.logo_url.clone(),
                                    title: integration.title.clone(),
                                    class: Some("h-20 w-20 rounded-box border border-base-300 bg-base-200/40 p-3".to_string())
                                }
                                div {
                                    class: "space-y-4",
                                    h1 { class: "font-mono text-4xl font-bold tracking-tight", "{integration.title}" }
                                    p {
                                        class: "text-base-content/75",
                                        "{integration.description.as_deref().unwrap_or(\"No description provided.\")}"
                                    }
                                    div {
                                        class: "flex flex-wrap gap-2",
                                        if let Some(version) = integration.version.as_deref() {
                                            Badge {
                                                badge_style: BadgeStyle::Outline,
                                                "v{version}"
                                            }
                                        }
                                        Badge {
                                            badge_style: BadgeStyle::Outline,
                                            badge_color: BadgeColor::Primary,
                                            "{integration.endpoints.len()} methods"
                                        }
                                    }
                                }
                            }
                        }

                        section {
                            class: "space-y-6",
                            div {
                                class: "space-y-2",
                                h2 { class: "font-mono text-3xl font-bold tracking-tight", "Methods" }
                                p {
                                    class: "text-base-content/70",
                                    "Each method below is pulled directly from the OpenAPI file."
                                }
                            }
                            if integration.endpoints.is_empty() {
                                SiteCard {
                                    class: Some("border-dashed shadow-none".to_string()),
                                    interactive: Some(false),
                                    p { class: "text-base-content/70", "This spec does not define any methods." }
                                }
                            } else {
                                for endpoint in integration.endpoints.clone() {
                                    EndpointCard { endpoint, logo_url: integration.logo_url.clone(), title: integration.title.clone() }
                                }
                            }
                        }

                        section {
                            class: "space-y-4",
                            div {
                                class: "space-y-2",
                                h2 { class: "font-mono text-3xl font-bold tracking-tight", "YAML" }
                                p {
                                    class: "text-base-content/70",
                                    "Copy and paste the full spec from the textarea below."
                                }
                            }
                            SiteCard {
                                interactive: Some(false),
                                textarea {
                                    class: "textarea textarea-bordered min-h-[28rem] w-full font-mono text-sm leading-6",
                                    readonly: true,
                                    "{integration.yaml}"
                                }
                            }
                        }
                    }
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    };

    ssg_whiz::render(page)
}

#[component]
fn EndpointCard(endpoint: Endpoint, logo_url: Option<String>, title: String) -> Element {
    let description = endpoint
        .description
        .as_deref()
        .or(endpoint.summary.as_deref())
        .unwrap_or("No description provided.");

    rsx! {
        SiteCard {
            body_class: Some("gap-5".to_string()),
            interactive: Some(false),
            div {
                class: "flex flex-col gap-4 md:flex-row md:items-start md:justify-between",
                div {
                    class: "space-y-4",
                    div {
                        class: "flex flex-wrap items-center gap-3",
                        Badge {
                            badge_style: BadgeStyle::Outline,
                            badge_color: BadgeColor::Primary,
                            "{endpoint.method}"
                        }
                        code {
                            class: "rounded bg-base-200 px-3 py-2 text-sm",
                            "{endpoint.path}"
                        }
                        if let Some(operation_id) = endpoint.operation_id.as_deref() {
                            span {
                                class: "text-sm text-base-content/60",
                                "{operation_id}"
                            }
                        }
                    }
                    p {
                        class: "text-base-content/75",
                        "{description}"
                    }
                }
                SpecLogo {
                    logo_url,
                    title,
                    class: Some("h-16 w-16 rounded-box border border-base-300 bg-base-200/40 p-3".to_string())
                }
            }
            if !endpoint.parameters.is_empty() {
                div {
                    class: "space-y-3",
                    h3 { class: "font-mono text-lg font-semibold tracking-tight", "Parameters" }
                    div {
                        class: "grid gap-3 md:grid-cols-2",
                        for parameter in endpoint.parameters {
                            SiteCard {
                                class: Some("bg-base-200/30 shadow-none".to_string()),
                                body_class: Some("gap-2 p-4".to_string()),
                                interactive: Some(false),
                                div {
                                    class: "flex flex-wrap items-center gap-2",
                                    strong { "{parameter.name}" }
                                    if let Some(location) = parameter.location.as_deref() {
                                        Badge { badge_style: BadgeStyle::Outline, "{location}" }
                                    }
                                    if parameter.required {
                                        Badge { badge_style: BadgeStyle::Outline, badge_color: BadgeColor::Warning, "required" }
                                    }
                                }
                                if let Some(description) = parameter.description.as_deref() {
                                    p { class: "text-sm text-base-content/70", "{description}" }
                                }
                            }
                        }
                    }
                }
            }
            if !endpoint.request_body_content.is_empty() {
                div {
                    class: "space-y-3",
                    h3 { class: "font-mono text-lg font-semibold tracking-tight", "Request body" }
                    div {
                        class: "flex flex-wrap gap-2",
                        for content_type in endpoint.request_body_content {
                            Badge {
                                badge_style: BadgeStyle::Outline,
                                badge_color: BadgeColor::Secondary,
                                "{content_type}"
                            }
                        }
                    }
                }
            }
            if !endpoint.responses.is_empty() {
                div {
                    class: "space-y-3",
                    h3 { class: "font-mono text-lg font-semibold tracking-tight", "Responses" }
                    div {
                        class: "grid gap-3 md:grid-cols-2",
                        for response in endpoint.responses {
                            SiteCard {
                                class: Some("bg-base-200/30 shadow-none".to_string()),
                                body_class: Some("gap-2 p-4".to_string()),
                                interactive: Some(false),
                                div {
                                    class: "flex items-center gap-2",
                                    Badge {
                                        badge_style: BadgeStyle::Outline,
                                        badge_color: BadgeColor::Primary,
                                        "{response.status}"
                                    }
                                }
                                if let Some(description) = response.description.as_deref() {
                                    p { class: "text-sm text-base-content/70", "{description}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SpecLogo(logo_url: Option<String>, title: String, class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| {
        "h-12 w-12 rounded-box border border-base-300 bg-base-200/40 p-2".to_string()
    });
    rsx! {
        if let Some(logo_url) = logo_url {
            img {
                class: "{class}",
                alt: "{title} logo",
                src: "{logo_url}"
            }
        } else {
            div {
                class: "{class} flex items-center justify-center text-xs font-semibold uppercase text-base-content/60",
                "{title.chars().next().unwrap_or('A')}"
            }
        }
    }
}
