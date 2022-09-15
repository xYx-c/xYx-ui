use dioxus::prelude::*;
/**
* @Author xYx
* @Date 2022-09-14 16:28:16
*/
#[derive(Props)]
pub struct ColProps<'a> {
    #[props(default = 0)]
    pub span: u8,
    #[props(default = 0)]
    pub offset: u8,
    #[props(default)]
    pub narrow: bool,
    children: Element<'a>,
}

pub fn Col<'a>(cx: Scope<'a, ColProps<'a>>) -> Element {
    let mut class_name = String::from("column");
    if cx.props.span > 0 {
        if cx.props.span > 12 {
            class_name += " is-12";
        } else {
            class_name += &format!(" is-{}", cx.props.span);
        }
    }
    if cx.props.offset > 0 {
        if cx.props.offset > 11 {
            class_name += " is-offset-11";
        } else {
            class_name += &format!(" is-offset-{}", cx.props.offset);
        }
    }
    if cx.props.narrow {
        class_name.push_str(" is-narrow");
    }
    cx.render(rsx! {
        div {
            class: "{class_name}",
            &cx.props.children
        }
    })
}
