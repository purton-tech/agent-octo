#![allow(non_snake_case)]

use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CountLabel {
    pub count: usize,
    pub label: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct CardItemProps {
    pub class: Option<String>,
    pub title: String,
    pub description: Option<Element>,
    pub footer: Option<Element>,
    pub count_labels: Vec<CountLabel>,
    pub action: Option<Element>,
}

#[component]
pub fn CardItem(props: CardItemProps) -> Element {
    rsx! {
        Card {
            class: {
                let base = "card-border p-4 mt-4 flex flex-row justify-between items-stretch";
                match props.class.clone() {
                    Some(extra) => format!("{base} {extra}"),
                    None => base.to_string(),
                }
            },
            div {
                class: "flex flex-col flex-1 min-w-0",
                h2 { class: "font-semibold text-base truncate", "{props.title}" }
                if let Some(desc) = props.description {
                    div { class: "text-sm text-base-content/70 mt-1", {desc} }
                }
                if let Some(foot) = props.footer {
                    div { class: "text-xs text-base-content/60 mt-3", {foot} }
                }
            }
            div {
                class: "flex flex-row items-center gap-5",
                for entry in props.count_labels.iter() {
                    div {
                        class: "flex flex-col justify-center text-center",
                        div { "{entry.count}" }
                        div { class: "text-base-content/70", "{entry.label}" }
                    }
                }
                if let Some(action) = props.action {
                    div { class: "ml-4 flex flex-col justify-center gap-2", {action} }
                }
            }
        }
    }
}
