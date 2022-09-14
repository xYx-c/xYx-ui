use gloo::{utils::{document, body}, timers::future::TimeoutFuture};
use js_sys::Function;
use wasm_bindgen_futures::spawn_local;
/**
* @Author xYx
* @Date 2022-09-14 15:44:01
*/
pub fn notify(messgae: &str) {
    let body = body();
    let count = document().get_elements_by_class_name("notification").length();

    let div = document().create_element("div").ok().expect("create div error");
    div.set_class_name("notification");
    div.set_text_content(Some(messgae));
    body.append_child(&div).ok();

    let button = document().create_element("button").ok().expect("create button error");
    div.append_child(&button).ok();
    button.set_class_name("delete");
    let fun = Function::new_with_args("event", "this.parentNode.remove();");
    button.add_event_listener_with_callback("click", &fun).map_err(|e| tracing::debug!("{:#?}", e)).ok();

    let mut style = "".to_string();
    let height = div.client_height() as u32 * count + 16 * (count + 1);
    style.push_str(&format!("top: {}px;", height));
    style += "width: 330px;";
    style += "position: fixed;";
    style += "right: 16px;";
    div.set_attribute("style", &style).ok();

    spawn_local(async move {
        TimeoutFuture::new(4500).await;
        body.remove_child(&div).ok();
    });
}

