use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, window};

// A simple helper function to get the document from the global window.
fn document() -> Document {
    window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document")
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn hydrate() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let doc = document();
    // Query for the button element with `data-alert` attribute.
    let button: Option<Element> = doc.query_selector("[data-alert]").ok().flatten();

    if let Some(btn_el) = button {
        // Convert to an HtmlElement to attach event listener easily.
        let btn = btn_el.dyn_into::<HtmlElement>()?;

        // Closure for the event listener.
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
            // Show the alert when button is clicked.
            window()
                .unwrap()
                .alert_with_message("Alert triggered!")
                .unwrap();
        });

        // Add the click event listener to the button.
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Keep the closure alive for the lifetime of the program.
    } else {
        web_sys::console::log_1(&"No element with data-alert found!".into());
    }

    Ok(())
}
