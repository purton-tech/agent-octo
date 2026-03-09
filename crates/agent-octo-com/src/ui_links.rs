use daisy_rsx::marketing::footer::FooterLinks;
use daisy_rsx::marketing::navigation::{NavigationEntry, NavigationLink, NavigationModel, Section};
use ssg_whiz::SiteMeta;

pub fn navigation_links() -> NavigationModel {
    NavigationModel {
        home: "/".to_string(),
        logo_src: Some("/logo.svg".to_string()),
        logo_alt: Some("agent-octo.com".to_string()),
        desktop_left: vec![
            NavigationEntry::Link(NavigationLink::external("Home", "/", Section::Home)),
            NavigationEntry::Link(NavigationLink::external("Blog", "/blog", Section::Blog)),
        ],
        desktop_right: vec![NavigationLink::new("Get Started", "/#hero", Section::Home)
            .with_class("btn btn-primary")],
        mobile: vec![
            NavigationLink::external("Home", "/", Section::Home),
            NavigationLink::external("Blog", "/blog", Section::Blog),
            NavigationLink::new("Get Started", "/#hero", Section::Home),
        ],
    }
}

pub fn footer_links() -> FooterLinks {
    FooterLinks {
        blog: "/blog".to_string(),
        pricing: "/".to_string(),
        contact: "/".to_string(),
        terms: "/".to_string(),
        privacy: "/".to_string(),
        about: None,
        variant: None,
    }
}

pub fn site_meta() -> SiteMeta {
    SiteMeta {
        base_url: "https://agent-octo.com".to_string(),
        site_name: "agent-octo.com".to_string(),
        brand_name: "agent-octo.com".to_string(),
        goatcounter: "".to_string(),
    }
}
