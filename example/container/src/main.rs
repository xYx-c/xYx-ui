use dioxus::prelude::*;
use fex::layout::container::Container;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
            Container {
                div {
                    class: "box is-primary",
                    "This container is "
                    strong {
                        "centered"
                    }
                    " on desktop and larger viewports."
                }
            }
        })
}
