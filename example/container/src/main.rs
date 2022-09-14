use dioxus::prelude::*;
use xYx_ui::{layout::container::Container, elements::r#box::Box};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
            Container {
                Box {
                    "This container is "
                    strong {
                        "centered"
                    }
                    " on desktop and larger viewports."
                }
            }
        })
}
