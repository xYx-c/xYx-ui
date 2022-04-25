use dioxus::prelude::*;


#[inline_props]
pub fn Row<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx!{
        div {
            class: "cloumns",
            children
        }
    })
}
