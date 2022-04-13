use crate::models::color::Colors;
use dioxus::prelude::*;
// use gloo::timers::callback::Timeout;
// use gloo::timers::future::TimeoutFuture;
use gloo::utils::document;

#[derive(Props)]
pub struct NotificationProps<'a> {
    #[props(default)]
    color: Colors,
    #[props(default)]
    is_light: bool,
    #[props(default)]
    is_delete: bool,
    children: Element<'a>,
}
pub fn Notification<'a>(cx: Scope<'a, NotificationProps<'a>>) -> Element {
    let mut class_name = "notification".to_string();
    let closed = use_state(&cx, || false);
    // use_future(&cx, closed, |closed| async {
    //     let timer = Timeout::new(3000, move || closed.set(true));
    //     timer.forget();
    // });
    // use_future(&cx, closed, |closed| async move {
    //     TimeoutFuture::new(4000).await;
    //     if !*closed.get() {closed.set(true)}
    // });
    use_future(&cx, (), |_| async {
        let test = document().get_element_by_id("test");
        tracing::debug!("test: {:#?}", test);
    });
    class_name += cx.props.color.into();
    if cx.props.is_light {
        class_name += " is-light";
    }
    if !*closed.get() {
        if cx.props.is_delete {
            cx.render(rsx! {
                div {
                    id: "test",
                    class: "{class_name}",
                    button {
                        class: "delete",
                        onclick: |_| click(closed)
                    }
                    &cx.props.children
                }
            })
        } else {
            cx.render(rsx! {
                div {
                    id: "test",
                    class: "{class_name}",
                    &cx.props.children
                }
            })
        }
    } else {
        None
    }
}

fn click(closed: &UseState<bool>) {
    let test = document().get_element_by_id("test");
    // if let Some(test) = &test {
    //     test.set_attribute("aaa", "123")
    //         .map_err(|e| tracing::debug!("{:#?}", e))
    //         .ok();
    // }
    test.as_ref().map(|el| {
        el.set_attribute("aa", "222")
    });
    tracing::debug!("{:#?}", test);
    closed.set(true);
}
