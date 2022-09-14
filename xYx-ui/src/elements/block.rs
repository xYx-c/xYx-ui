use dioxus::prelude::*;

#[inline_props]
pub fn Bolck<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "block",
            children
        }
    })
}
