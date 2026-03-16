pub mod blog_summary;
pub mod components;
pub mod generator;
pub mod open_api_specs;
pub mod pages;
pub mod pages_summary;
pub mod ui_links;

use std::net::SocketAddr;

use ssg_whiz::{InlineScript, ScriptAsset, SiteAssets, SiteBuilder, SiteConfig};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = SiteConfig {
        dist_dir: "dist".into(),
        run_server: std::env::var("DO_NOT_RUN_SERVER").is_err(),
        addr: SocketAddr::from(([0, 0, 0, 0], 8083)),
        live_reload: true,
        navigation_links: ui_links::navigation_links(),
        footer_links: ui_links::footer_links(),
        site_meta: ui_links::site_meta(),
        site_header: None,
        site_assets: SiteAssets {
            stylesheets: vec![
                "/tailwind.css".to_string(),
                "/content-lightbox.css".to_string(),
            ],
            head_scripts: vec![ScriptAsset {
                src: "/copy-paste.js".to_string(),
                script_type: None,
                async_load: true,
                integrity: None,
                data_goatcounter: None,
            }],
            body_scripts: vec![
                ScriptAsset {
                    src: "https://unpkg.com/@digicreon/mujs/dist/mu.min.js".to_string(),
                    script_type: None,
                    async_load: false,
                    integrity: None,
                    data_goatcounter: None,
                },
                ScriptAsset {
                    src: "/scroll-story.js".to_string(),
                    script_type: None,
                    async_load: false,
                    integrity: None,
                    data_goatcounter: None,
                },
                ScriptAsset {
                    src: "/content-lightbox.js".to_string(),
                    script_type: None,
                    async_load: false,
                    integrity: None,
                    data_goatcounter: None,
                },
            ],
            head_inline_scripts: vec![],
            body_inline_scripts: vec![InlineScript::new("mu.init({ processForms: false });")],
        },
        extra_footer: None,
    };

    SiteBuilder::new(config)
        .blog(blog_summary::summary())
        .pages(pages_summary::summary())
        .static_pages(generator::generate_static_pages)
        .build()
        .await
        .expect("Failed to generate website");
}
