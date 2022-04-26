use dioxus::prelude::*;

use crate::models::size::Sizes;

#[derive(Props)]
pub struct DeleteProps<'a> {
    sizes: Sizes,
    children: Element<'a>
}

pub fn Delete<'a>(cx: Scope<'a, DeleteProps<'a>>) -> Element {
    let mut classes: String = "delete".into();
    classes += cx.props.sizes.into();
    cx.render(rsx! {
        button {
            class: "{classes}",
            &cx.props.children
        }
    })
}
