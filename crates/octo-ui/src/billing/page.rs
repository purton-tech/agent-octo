#![allow(non_snake_case)]

use crate::{
    components::section_introduction::SectionIntroduction,
    layout::{Layout, SideBar},
    render, routes,
};
use clorinde::queries::billing::TopUpTransaction;
use daisy_rsx::*;
use dioxus::prelude::*;

const TOP_UP_AMOUNTS_CENTS: [i64; 3] = [1000, 2500, 5000];

pub fn page(
    org_id: String,
    balance_label: String,
    history: Vec<TopUpTransaction>,
    error_message: Option<String>,
) -> String {
    let checkout_action = routes::billing::StartCheckout {
        org_id: org_id.clone(),
    }
    .to_string();

    let page = rsx! {
        Layout {
            title: "Billing".to_string(),
            org_id,
            balance_label: balance_label.clone(),
            selected_item: SideBar::Billing,
            header_left: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Agent Octo".to_string(),
                            href: Some("/".to_string()),
                        },
                        BreadcrumbItem {
                            text: "Billing".to_string(),
                            href: None,
                        },
                    ]
                }
            ),
            header_right: None,
            SectionIntroduction {
                header: "Billing".to_string(),
                subtitle: "Top up your prepaid balance with Stripe and review recent payments.".to_string(),
                is_empty: false,
                empty_text: "".to_string()
            }
            if let Some(message) = error_message {
                Alert {
                    class: "mt-4".to_string(),
                    alert_color: Some(AlertColor::Error),
                    span { "{message}" }
                }
            }
            div {
                class: "mt-4",
                div {
                    class: "flex items-center gap-3",
                    h2 { class: "text-lg font-semibold", "Current Balance" }
                    span { class: "text-3xl font-semibold", "{balance_label}" }
                }
                p { class: "text-sm text-base-content/70 mt-2", "Select a fixed amount to continue to Stripe Checkout." }
            }
            div {
                class: "grid grid-cols-1 gap-4 mt-6 md:grid-cols-3",
                for amount_cents in TOP_UP_AMOUNTS_CENTS {
                    form {
                        action: "{checkout_action}",
                        method: "post",
                        "mu-disabled": true,
                        Card {
                            class: "card-border bg-base-100 h-full",
                            CardBody {
                                h3 { class: "text-xl font-semibold", "{format_dollars_from_cents(amount_cents)}" }
                                p {
                                    class: "text-sm text-base-content/70",
                                    "Prepaid balance added after Stripe confirms payment."
                                }
                                input {
                                    r#type: "hidden",
                                    name: "amount_cents",
                                    value: "{amount_cents}"
                                }
                                div {
                                    class: "card-actions justify-end mt-4",
                                    Button {
                                        button_type: ButtonType::Submit,
                                        button_scheme: ButtonScheme::Primary,
                                        "Top Up"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Card {
                class: "card-border bg-base-100 mt-6",
                CardBody {
                    h2 { class: "card-title", "Billing History" }
                    if history.is_empty() {
                        p {
                            class: "text-sm text-base-content/70 mt-2",
                            "No top-up transactions yet."
                        }
                    } else {
                        div {
                            class: "overflow-x-auto mt-4",
                            table {
                                class: "table",
                                thead {
                                    tr {
                                        th { "Amount" }
                                        th { "Status" }
                                        th { "Created" }
                                        th { "Completed" }
                                    }
                                }
                                tbody {
                                    for entry in history {
                                        tr {
                                            td { "{format_balance_microcents(entry.amount_microcents)}" }
                                            td { "{title_case_status(&entry.status)}" }
                                            td { "{entry.created_at.to_rfc3339()}" }
                                            td {
                                                if entry.status == "succeeded" {
                                                    "{entry.completed_at.to_rfc3339()}"
                                                } else {
                                                    "-"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}

fn format_dollars_from_cents(amount_cents: i64) -> String {
    format!("${}.00", amount_cents / 100)
}

fn format_balance_microcents(balance_microcents: i64) -> String {
    let is_negative = balance_microcents < 0;
    let abs_microcents = balance_microcents.unsigned_abs();
    let cents = abs_microcents / 1_000_000;
    let dollars = cents / 100;
    let cents_remainder = cents % 100;

    if is_negative {
        format!("-${dollars}.{cents_remainder:02}")
    } else {
        format!("${dollars}.{cents_remainder:02}")
    }
}

fn title_case_status(status: &str) -> String {
    let mut chars = status.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
