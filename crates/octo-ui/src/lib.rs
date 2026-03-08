pub mod agents;
mod base_layout;
pub mod channels;
pub mod components;
pub mod connections;
pub mod integrations;
mod layout;
pub mod providers;
pub mod routes;

use dioxus::prelude::*;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{html}</html>")
}
