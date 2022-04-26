use dioxus::prelude::*;
use fex::{elements::icon::Icon, models::size::Sizes};

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            icon: "fas fa-home",
            size: Sizes::Small,
        }
        Icon {
            icon: "fas fa-home",
            // size: Sizes::Normal,
        }
        Icon {
            icon: "fas fa-lg fa-home",
            size: Sizes::Medium,
        }
        Icon {
            icon: "fas fa-2x fa-home",
            size: Sizes::Medium,
            color: fex::elements::icon::Color::Info,
            "Home"
        }
        Icon {
            icon: "fas fa-2x fa-home",
            size: Sizes::Large,
        }
    })
} 
