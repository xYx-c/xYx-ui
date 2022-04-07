use crate::models::color::Colors;
use dioxus::prelude::*;

#[derive(Props)]
pub struct NotificationProps<'a> {
    #[props(default)]
    color: Colors,
    #[props(default)]
    is_light: bool,
    children: Element<'a>,
}

pub fn Notification<'a>(cx: Scope<'a, NotificationProps<'a>>) -> Element {
    let mut class_name = "notification".to_string();
    class_name += cx.props.color.into();
    if cx.props.is_light {
        class_name += " is-light";
    }
    rsx!{cx,
        div{class: "{class_name}"}
    }
}
