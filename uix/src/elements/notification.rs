use crate::models::color::Colors;
use dioxus::prelude::*;
// use gloo::timers::callback::Timeout;
// use gloo::timers::future::TimeoutFuture;
use gloo::{utils::{document, body}, timers::future::TimeoutFuture};
use wasm_bindgen_futures::spawn_local;

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

pub fn open(cx: Scope) {
    let body = body();
    let div = document().create_element("div").ok().expect("create div error");
    div.set_class_name("notification");
    div.set_text_content(Some("hello world"));
    let aa = rsx!(div{"asd"});
    cx.render(aa);
    body.append_child(&div).map_err(|e| tracing::debug!("{:#?}", e)).ok();
    let button = document().create_element("button").ok().expect("create button error");
    button.set_class_name("delete");
    div.append_child(&button).ok();
    spawn_local(async move {
        TimeoutFuture::new(4500).await;
        body.remove_child(&div).map_err(|_| tracing::debug!("remove_child error")).ok();
    });
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
                    class: "{class_name}",
                    top: "16px",
                    right: "16px",
                    position: "fixed",
                    width: "330px",
                    display: "flex",
                    z_index: "2005",
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
