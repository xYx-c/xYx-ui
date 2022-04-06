use dioxus::prelude::*;
use uix::{
    components::elements::button::Button,
    models::{color::Colors, size::Sizes},
};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Button {
            color: Colors::Link,
            size: Sizes::Small,
            onclick: move |e| {
                tracing::info!("Clicked!{:?}", e);
            }
            "Click me"
        }
    })
}
