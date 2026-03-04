use crate::{layout::Layout, render};
use clorinde::queries::auth::User;
use dioxus::prelude::*;
use octo_assets::files::favicon_svg;

pub fn index(users: Vec<User>) -> String {
    let page = rsx! {
        Layout {
            title: "Users".to_string(),
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in users {
                        tr {
                            td {
                                img {
                                    src: favicon_svg.name,
                                    width: "16",
                                    height: "16"
                                }
                                strong { "{user.id}" }
                            }
                            td { "{user.email}" }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
