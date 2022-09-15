use crate::models::{color::Colors, size::Sizes};
use dioxus::{events::MouseEvent, prelude::*};

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
    icon: &'a str,
    #[props(default)]
    is_selected: bool,
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
    if cx.props.is_selected {
        class_name += " is-selected";
    }
    if cx.props.icon != "" {
        cx.render(rsx! {
            button {
                class: "{class_name}",
                disabled: "{cx.props.disabled}",
                onclick: move |evt| cx.props.onclick.call(evt),
                span {
                    class: "icon",
                    i {
                        class: "{cx.props.icon}",
                    }
                }
                &cx.props.children,
            }
        })
    } else {
        cx.render(rsx! {
            button {
                class: "{class_name}",
                disabled: "{cx.props.disabled}",
                onclick: move |evt| cx.props.onclick.call(evt),
                &cx.props.children,
            }
        })
    }
}

#[derive(Props)]
pub struct ButtonsProps<'a>{
    #[props(default)]
    has_addons: bool,
    #[props(default)]
    is_centered: bool,
    #[props(default)]
    is_right: bool,
    children: Element<'a>,
}

pub fn Buttons<'a>(cx: Scope<'a, ButtonsProps<'a>>) -> Element {
    let mut class = String::from("buttons");
    if cx.props.has_addons {
        class += " has-addons";
    }
    if cx.props.is_centered {
        class += " is-centered";
    }
    if cx.props.is_right {
        class += " is-right";
    }
    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children,
        }
    })
}
