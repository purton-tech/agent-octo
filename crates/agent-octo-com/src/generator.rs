use ssg_whiz::SitePage;

use crate::{open_api_specs, pages};

fn output_page(path: &str, html: String) -> SitePage {
    SitePage {
        path: path.to_string(),
        html,
    }
}

pub async fn generate_static_pages() -> Vec<SitePage> {
    let mut pages_out = vec![output_page("", pages::index::page())];
    let specs = open_api_specs::load_integration_specs();

    pages_out.push(output_page(
        "open-api-specs",
        pages::open_api_specs::index_page(&specs),
    ));

    for spec in specs {
        pages_out.push(output_page(
            &format!("open-api-specs/{}", spec.slug),
            pages::open_api_specs::detail_page(&spec),
        ));
    }

    pages_out
}
