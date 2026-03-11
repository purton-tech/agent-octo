use dioxus::prelude::*;
use ssg_whiz::{layouts::layout::Layout, Footer, Section};

use crate::components::{
    homepage_architecture_section::HomepageArchitectureSection,
    homepage_capabilities_section::HomepageCapabilitiesSection,
    homepage_faq_section::HomepageFaqSection, homepage_hero_section::HomepageHeroSection,
    homepage_principles_section::HomepagePrinciplesSection,
    homepage_quick_start_section::HomepageQuickStartSection,
    homepage_story_section::HomepageStorySection,
};

pub fn page() -> String {
    let page = rsx!(
        Layout {
            title: "agent-octo.com".to_string(),
            description: "Agent Octo is a multi-tenant agent platform with runtime integrations, channels, and a Rust-powered sandbox.".to_string(),
            image: Some("/logo.svg".to_string()),
            mobile_menu: None,
            section: Section::Home,
            main {
                class: "min-h-screen text-base-content",

                HomepageHeroSection {}
                HomepageStorySection {}
                HomepageCapabilitiesSection {}
                HomepageArchitectureSection {}
                HomepagePrinciplesSection {}
                HomepageQuickStartSection {}
                HomepageFaqSection {}

                Footer {
                    margin_top: Some("mt-0".to_string()),
                    links: crate::ui_links::footer_links(),
                }
            }
        }
    );

    ssg_whiz::render(page)
}
