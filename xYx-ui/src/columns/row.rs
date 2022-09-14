use dioxus::prelude::*;
/**
* @Author xYx
* @Date 2022-09-14 16:27:01
*/
#[derive(Props)]
pub struct RowProps<'a> {
    #[props(default)]
    pub is_mobile: bool,
    #[props(default)]
    pub is_desktop: bool,
    #[props(default)]
    pub is_gapless: bool,
    #[props(default)]
    pub gap: u8,
    #[props(default)]
    pub is_multiline: bool,
    #[props(default)]
    pub is_vcentered: bool,
    #[props(default)]
    pub is_centered: bool,
    pub children: Element<'a>,
}

pub fn Row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    let mut class_name = String::from("columns");
    if cx.props.is_mobile {
        class_name.push_str(" is-mobile");
    }
    if cx.props.is_desktop {
        class_name.push_str(" is-desktop");
    }
    if cx.props.is_gapless {
        class_name.push_str(" is-gapless");
    }
    if cx.props.gap > 0 {
        class_name.push_str(&format!(" is-{}", cx.props.gap));
    }
    if cx.props.is_multiline {
        class_name.push_str(" is-multiline");
    }
    if cx.props.is_vcentered {
        class_name.push_str(" is-vcentered");
    };
    if cx.props.is_centered {
        class_name.push_str(" is-centered");
    }
    cx.render(rsx! {
        div {
            class: "{class_name}",
            &cx.props.children
        }
    })
}
