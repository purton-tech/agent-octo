use js_sys::{Function, Reflect};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Document, Element, Event, console};

pub fn hydrate_data_target_popovers(doc: &Document) -> Result<(), JsValue> {
    if doc.query_selector("[data-target]")?.is_none() {
        console::log_1(
            &"[octo-islands] data-target hydrator: no [data-target] elements found".into(),
        );
        return Ok(());
    }
    console::log_1(&"[octo-islands] data-target hydrator: attached click listener".into());

    let doc_for_handler = doc.clone();
    let click_handler = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        let Some(event_target) = event.target() else {
            console::log_1(&"[octo-islands] click event without target".into());
            return;
        };

        let Ok(element) = event_target.dyn_into::<Element>() else {
            console::log_1(&"[octo-islands] click target is not an element".into());
            return;
        };

        let Ok(Some(trigger)) = element.closest("[data-target]") else {
            console::log_1(&"[octo-islands] click did not hit [data-target] trigger".into());
            return;
        };
        console::log_1(&"[octo-islands] found [data-target] trigger".into());

        let Some(target_id) = trigger
            .get_attribute("data-target")
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty())
        else {
            console::warn_1(&"[octo-islands] trigger had empty data-target".into());
            return;
        };
        console::log_1(&format!("[octo-islands] opening target id='{target_id}'").into());

        let Some(target) = doc_for_handler.get_element_by_id(&target_id) else {
            console::warn_1(&format!("No element found for data-target='{target_id}'").into());
            return;
        };

        if let Err(error) = open_target_element(&target) {
            console::error_1(
                &format!("[octo-islands] failed to open target id='{target_id}'").into(),
            );
            console::error_1(&error);
        } else {
            console::log_1(&format!("[octo-islands] opened target id='{target_id}'").into());
        }
    });

    doc.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
    click_handler.forget();

    Ok(())
}

fn open_target_element(target: &Element) -> Result<(), JsValue> {
    if target.tag_name().eq_ignore_ascii_case("dialog") && call_method(target, "showModal")? {
        console::log_1(&"[octo-islands] used showModal() for <dialog>".into());
        return Ok(());
    }

    if call_method(target, "showPopover")? {
        console::log_1(&"[octo-islands] used showPopover()".into());
        return Ok(());
    }

    if call_method(target, "showModal")? {
        console::log_1(&"[octo-islands] used showModal()".into());
        return Ok(());
    }

    console::log_1(&"[octo-islands] fallback to setting open attribute".into());
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
