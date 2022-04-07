use dioxus::{prelude::*, events::MouseEvent};
use crate::models::{color::Colors, size::Sizes};

#[derive(PartialEq)]
pub enum State {
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
    is_responsive: bool,
    #[props(default)]
    is_fullwidth: bool,
    #[props(default)]
    is_outlined: bool,
    #[props(default)]
    is_inverted: bool,
    #[props(default)]
    is_rounded: bool,
    #[props(default)]
    is_loading: bool,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let mut class_name = String::from("button");
    class_name += cx.props.color.into();
    class_name += cx.props.size.into();
    if cx.props.is_light {
        class_name += " is-light";
    }
    if cx.props.is_responsive {
        class_name += " is-responsive";
    }
    if cx.props.is_fullwidth {
        class_name += " is-fullwidth";
    }
    if cx.props.is_outlined {
        class_name += " is-outlined";
    }
    if cx.props.is_inverted {
        class_name += " is-inverted";
    }
    if cx.props.is_rounded {
        class_name += " is-rounded";
    }
    if cx.props.is_loading {
        class_name += " is-loading";
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

#[inline_props]
pub fn Buttons<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "buttons",
            children,
        }
    })
}
