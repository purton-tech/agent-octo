use ssg_whiz::{FooterLinks, NavigationEntry, NavigationLink, NavigationModel, Section, SiteMeta};

pub fn navigation_links() -> NavigationModel {
    NavigationModel {
        home: "/".to_string(),
        logo_src: Some("/logo.svg".to_string()),
        logo_alt: Some("Agent Octo".to_string()),
        desktop_left: vec![
            NavigationEntry::Link(NavigationLink::external("Home", "/", Section::Home)),
            NavigationEntry::Link(NavigationLink::external(
                "Pricing",
                "/pricing",
                Section::Pricing,
            )),
            NavigationEntry::Link(NavigationLink::external(
                "Specs",
                "/open-api-specs/",
                Section::McpServers,
            )),
            NavigationEntry::Link(NavigationLink::external("Blog", "/blog", Section::Blog)),
        ],
        desktop_right: vec![NavigationLink::external(
            "Login / Signup",
            "https://app.agent-octo.com",
            Section::Home,
        )
        .with_class("btn btn-primary")],
        mobile: vec![
            NavigationLink::external("Home", "/", Section::Home),
            NavigationLink::external("Pricing", "/pricing", Section::Pricing),
            NavigationLink::external("Specs", "/open-api-specs/", Section::McpServers),
            NavigationLink::external("Blog", "/blog", Section::Blog),
            NavigationLink::external(
                "Login / Signup",
                "https://app.agent-octo.com",
                Section::Home,
            ),
        ],
    }
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: "/blog".to_string(),
        pricing: "/pricing".to_string(),
        contact: "/".to_string(),
        terms: "/".to_string(),
        privacy: "/".to_string(),
        about: None,
    }
}

pub fn site_meta() -> SiteMeta {
    SiteMeta {
        base_url: "https://agent-octo.com".to_string(),
        site_name: "Agent Octo".to_string(),
        brand_name: "Agent Octo".to_string(),
        goatcounter: "".to_string(),
    }
}
