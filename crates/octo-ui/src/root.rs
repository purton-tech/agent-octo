use crate::{
    layout::{Layout, SideBar},
    render,
};
use clorinde::queries::auth::User;
use dioxus::prelude::*;
use octo_assets::files::favicon_svg;

pub fn index(org_id: String, users: Vec<User>) -> String {
    let page = rsx! {
        Layout {
            title: "Users".to_string(),
            org_id,
            selected_item: SideBar::Users,
            table {
                class: "users-table",
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
                                span {
                                    class: "user-id",
                                    img {
                                        src: favicon_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                    strong { "{user.id}" }
                                }
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
