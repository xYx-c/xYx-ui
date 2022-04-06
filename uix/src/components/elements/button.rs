use dioxus::{prelude::*, events::MouseEvent};

use crate::models::{color::Colors, size::Sizes};

#[derive(PartialEq)]
pub enum Status {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
    Disabled,
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default)]
    color: Colors,
    #[props(default)]
    size: Sizes,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    disabled: bool,

    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let mut class_name = String::from("button");
    class_name += &format!(" is-{}", cx.props.color.to_string());
    class_name += &format!(" is-{}", cx.props.size.to_string());
    if cx.props.is_light {
        class_name += " is-light";
    }
    cx.render(rsx! {
        button {
            class: "{class_name}",
            disabled: "{cx.props.disabled}",
            onclick: move |evt| cx.props.onclick.call(evt),
            &cx.props.children,
        }
    })
}
