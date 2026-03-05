pub mod agents;
pub mod channels;
pub mod components;
mod layout;
pub mod routes;

use dioxus::prelude::*;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{html}</html>")
}
