use dioxus::prelude::*;

#[inline_props]
pub fn Col<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "column",
            children
        }
    })
}
