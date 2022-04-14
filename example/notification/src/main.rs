use dioxus::prelude::*;
use uix::elements::button::Button;
use uix::elements::notification;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Button {
            onclick: move |_evt| {
                notification::open(cx);
            },
            "click"
            rsx! (div {"1111111111"})
        }
    })
}
