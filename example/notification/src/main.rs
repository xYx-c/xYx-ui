use dioxus::prelude::*;
use uix::elements::notification::Notification;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Notification {
            is_delete: true,
            "Hello, world!",
        }
    })
}
