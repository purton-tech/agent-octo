use crate::{layout::Layout, render};
use clorinde::queries::auth::User;
use dioxus::prelude::*;

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
                            td { "{user.id}" }
                            td { "{user.email}" }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
