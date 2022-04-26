use dioxus::prelude::*;

use crate::models::size::Sizes;

#[derive(Props)]
pub struct ContentProps<'a> {
    #[props(default)] 
    pub sizes: Sizes,
    children: Element<'a>
}

pub fn Content<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element {
    let mut classes: String = "content".into();
    classes += cx.props.sizes.into();
    cx.render(rsx! {
        div {
            class: "{classes}",
            &cx.props.children
        }
    })
}
