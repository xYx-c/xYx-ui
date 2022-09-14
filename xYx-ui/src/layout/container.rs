use dioxus::prelude::*;
use std::fmt::Arguments;

#[derive(Props)]
pub struct ContainerProps<'a> {
    #[props(optional)]
    style: Option<Arguments<'a>>,
    children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "container is-max-desktop",
            style: cx.props.style.unwrap(),
            &cx.props.children,
        }
    })
}
