use dioxus::prelude::*;
use fex::{
    elements::button::Button,
    models::{color::Colors, size::Sizes}, layout::container::Container,
};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Container {
            div {
                style: "margin-top: 20%;",
                Button {
                    color: Colors::Link,
                    "Link"
                }
                Button {
                    "Primary",
                }
                Button {
                    color: Colors::Info,
                    "Info"
                }
                Button {
                    color: Colors::Success,
                    "Success"
                }
                Button {
                    color: Colors::Warning,
                    "Warning"
                }
                Button {
                    color: Colors::Danger,
                    "Danger"
                }
                Button {
                    color: Colors::White,
                    "White"
                }
                Button {
                    color: Colors::Black,
                    "Black"
                }
                Button {
                    color: Colors::Light,
                    "Light"
                }
                Button {
                    color: Colors::Dark,
                    "Dark"
                }
            }
            br {}
            div {
                Button {
                    color: Colors::Danger,
                    is_inverted: true,
                    "is_inverted"
                }
                Button {
                    color: Colors::Danger,
                    is_responsive: true,
                    "is_responsive"
                }
                Button {
                    color: Colors::Danger,
                    is_outlined: true,
                    "is_outlined"
                }
                Button {
                    color: Colors::Danger,
                    is_light: true,
                    "is_light"
                }
                Button {
                    color: Colors::Danger,
                    is_rounded: true,
                    "is_rounded"
                }
            }
            br {}
            div {
                Button {
                    size: Sizes::Small,
                    "Small"
                }
                Button {
                    size: Sizes::Medium,
                    "Medium"
                }
                Button {
                    size: Sizes::Large,
                    "Large"
                }
            }
            br {}
            Button {
                color: Colors::Success,
                size: Sizes::Normal,
                is_light: true,
                disabled: false,
                onclick: move |e| {
                    tracing::info!("Clicked!{:?}", e);
                }
                "Click Me"
            }
        }
    })
}
