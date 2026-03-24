use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

use crate::components::capabilities_section::CapabilitiesSection;
use crate::components::hero_section::HeroSection;
use crate::components::platform_section::PlatformSection;

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "Agent Octo - Agentic AI Assistant".to_string(),
            description: "Agent Octo is a multi-tenant agent platform with runtime integrations, channels, and a Rust-powered sandbox.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "min-h-screen text-base-content",

                div {
                    class: "grid gap-16 md:gap-24",

                    HeroSection {}
                    PlatformSection {}
                    CapabilitiesSection {}
                }

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    );

    ssg_whiz::render(page)
}
