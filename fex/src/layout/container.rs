use dioxus::prelude::*;

#[derive(Props)]
pub struct ContainerProps<'a> {
    pub children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "container is-max-desktop",
            &cx.props.children,
        }
    })
}
