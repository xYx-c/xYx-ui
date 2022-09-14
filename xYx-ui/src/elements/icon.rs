use dioxus::prelude::*;

use crate::models::size::Sizes;

#[derive(Props)]
pub struct IconProps<'a> {
    icon: &'a str,
    #[props(default)]
    size: Sizes,
    #[props(default)]
    color: Color,
    children: Element<'a>
}

/// https://ionic.io/ionicons
pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let mut icon: String = "icon".into();
    icon += cx.props.size.into();
    let mut icon_text: String = "icon-text".into();
    icon_text += cx.props.color.into();
    cx.render(rsx! {
        span {
            class: "{icon_text}",
            span {
                class: "{icon}", 
                i {
                    class: "{cx.props.icon}"
                }
            }
            span {
                &cx.props.children
            }
        }
    })
}




#[derive(Clone, Copy)]
pub enum Color {
    Info,
    Success,
    Warning,
    Danger,
    Null,
}

impl Default for Color {
    fn default() -> Self {
        Color::Null
    }
}

impl Into<&str> for Color {
    fn into(self) -> &'static str {
        match self {
            Color::Info => " has-text-info",
            Color::Success => " has-text-success",
            Color::Warning => " has-text-warning",
            Color::Danger => " has-text-danger",
            Color::Null => "",
        }
    }
}
