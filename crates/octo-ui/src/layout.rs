#![allow(non_snake_case)]
use crate::base_layout::BaseLayout;
use crate::routes;
use daisy_rsx::*;
use dioxus::prelude::*;
use octo_assets::files::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    Agents,
    Channels,
    Providers,
    Integrations,
    Connections,
}

impl std::fmt::Display for SideBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[component]
pub fn Layout(
    title: String,
    org_id: String,
    header_left: Element,
    header_right: Option<Element>,
    children: Element,
    selected_item: SideBar,
    content_class: Option<String>,
) -> Element {
    let agents_icon = if selected_item == SideBar::Agents {
        agents_active_svg.name
    } else {
        agents_svg.name
    };
    let channels_icon = if selected_item == SideBar::Channels {
        channels_active_svg.name
    } else {
        channels_svg.name
    };
    let providers_icon = if selected_item == SideBar::Providers {
        providers_active_svg.name
    } else {
        providers_svg.name
    };
    let integrations_icon = if selected_item == SideBar::Integrations {
        integrations_active_svg.name
    } else {
        integrations_svg.name
    };
    let connections_icon = if selected_item == SideBar::Connections {
        integrations_active_svg.name
    } else {
        integrations_svg.name
    };

    let agents_href = routes::agents::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let channels_href = routes::channels::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let providers_href = routes::providers::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let integrations_href = routes::integrations::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let connections_href = routes::connections::Index {
        org_id: org_id.clone(),
    }
    .to_string();
    let content_class = content_class.unwrap_or_else(|| "p-4 max-w-3xl w-full mx-auto".to_string());

    rsx! {
        BaseLayout {
            title,
            fav_icon_src: Some(favicon_svg.name.into()),
            web_assembly: (
                octo_islands_js.name.into(),
                octo_islands_bg_wasm.name.into()
            ),
            stylesheets: vec![tailwind_css.name.to_string(), "https://cdn.jsdelivr.net/npm/daisyui@5".into()],
            header_left,
            header_right,
            sidebar: rsx!(
                NavGroup {
                    heading: "Your Menu",
                    content:  rsx!(
                        NavItem {
                            id: SideBar::Agents.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: agents_href,
                            icon: agents_icon,
                            title: "Agents"
                        }
                        NavItem {
                            id: SideBar::Channels.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: channels_href,
                            icon: channels_icon,
                            title: "Channels"
                        }
                        NavItem {
                            id: SideBar::Providers.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: providers_href,
                            icon: providers_icon,
                            title: "Providers"
                        }
                        NavItem {
                            id: SideBar::Integrations.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: integrations_href,
                            icon: integrations_icon,
                            title: "Integrations"
                        }
                        NavItem {
                            id: SideBar::Connections.to_string(),
                            selected_item_id: selected_item.to_string(),
                            href: connections_href,
                            icon: connections_icon,
                            title: "Connections"
                        }
                    )
                }
            ),
            sidebar_header: rsx!(
                div {
                    class: "flex aspect-square size-8 items-center justify-center rounded-lg overflow-hidden",
                    img {
                        src: logo_svg.name,
                        alt: "App logo",
                        class: "size-8 object-contain"
                    }
                }
                div {
                    class: "ml-3 flex flex-col gap-0.5 leading-none",
                    span {
                        class: "font-semibold uppercase",
                        "Agent Octo"
                    }
                    span {
                        class: "",
                        "v1.0.1"
                    }
                }
            ),
            sidebar_footer: rsx!(
                div {
                    class: "text-center text-sm",
                    "You can place items at the bottom"
                }
            ),
            div {
                class: "{content_class}",
                {children}
            }
        }
    }
}
