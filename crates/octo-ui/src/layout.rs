#![allow(non_snake_case)]

use daisy_rsx::{Card, CardBody, CardHeader};
use dioxus::prelude::*;
use octo_assets::files::{favicon_svg, tailwind_css};

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum SideBar {
    Users,
}

#[component]
pub fn Layout(title: String, children: Element, selected_item: SideBar) -> Element {
    let section_label = match selected_item {
        SideBar::Users => "Users",
    };

    rsx! {
        head {
            title { "{title}" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link {
                rel: "stylesheet",
                href: tailwind_css.name,
                r#type: "text/css"
            }
            link {
                rel: "icon",
                r#type: "image/svg+xml",
                href: favicon_svg.name
            }
        }
        body {
            main {
                class: "page-shell",
                p {
                    class: "page-kicker",
                    "{section_label}"
                }
                Card {
                    class: "card",
                    CardHeader {
                        title: "{title}"
                    }
                    CardBody {
                        class: "card-body",
                        {children}
                    }
                }
            }
        }
    }
}
