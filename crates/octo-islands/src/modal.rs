use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Document, Element, Event, console};

pub fn hydrate_modal_triggers(doc: &Document) -> Result<(), JsValue> {
    let Some(root) = doc.document_element() else {
        return Ok(());
    };

    if root.has_attribute("data-octo-modal-hydrated") {
        return Ok(());
    }

    root.set_attribute("data-octo-modal-hydrated", "true")?;

    let doc_for_handler = doc.clone();
    let click_handler = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        let Some(event_target) = event.target() else {
            return;
        };

        let Ok(element) = event_target.dyn_into::<Element>() else {
            return;
        };

        // Close modals from explicit cancel controls.
        if let Ok(Some(cancel_trigger)) = element.closest(".cancel-modal, [data-cancel-modal]") {
            if let Ok(Some(dialog)) = cancel_trigger.closest("dialog") {
                event.prevent_default();
                if let Err(error) = call_method(&dialog, "close") {
                    console::error_1(&error);
                }
            }
            return;
        }

        let Ok(Some(trigger)) = element.closest("[data-target]") else {
            return;
        };

        event.prevent_default();

        let Some(target_id) = trigger
            .get_attribute("data-target")
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty())
        else {
            return;
        };

        let Some(target) = doc_for_handler.get_element_by_id(&target_id) else {
            console::warn_1(&format!("No element found for data-target='{target_id}'").into());
            return;
        };

        if let Err(error) = open_target_element(&target) {
            console::error_1(&error);
        }
    });

    doc.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
    click_handler.forget();

    Ok(())
}

fn open_target_element(target: &Element) -> Result<(), JsValue> {
    if target.tag_name().eq_ignore_ascii_case("dialog") && call_method(target, "showModal")? {
        return Ok(());
    }

    if call_method(target, "showPopover")? {
        return Ok(());
    }

    if call_method(target, "showModal")? {
        return Ok(());
    }

    target.set_attribute("open", "")?;
    Ok(())
}

fn call_method(target: &Element, method_name: &str) -> Result<bool, JsValue> {
    let method = Reflect::get(target.as_ref(), &JsValue::from_str(method_name))?;
    if !method.is_function() {
        return Ok(false);
    }

    let method = method.dyn_into::<Function>()?;
    method.call0(target.as_ref())?;
    Ok(true)
}
