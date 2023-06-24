#![allow(non_snake_case)]

use std::io::Read;
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use dioxus::html::{div, link, thead};
use dioxus::prelude::*;
use gloo_timers::callback::Interval;
use reqwest;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}


#[derive(Props, PartialEq)]
struct Test {
    a: Vec<String>,
}


fn Content(cx: Scope<Test>) -> Element {
    let states = use_state(cx, || {
        "".to_string()
    });
    use_future(cx, (), |_| {
        let title = states.clone();
        let whole_str = cx.props.a.clone();
        async move {
            loop {
                for i in whole_str.clone()
                {
                    let whole_str = i;
                    gloo_timers::future::sleep(Duration::from_millis(1000)).await;
                    let mut init_string = "".to_string();
                    for i in whole_str.chars() {
                        if i != '_' {
                            init_string.push(i);
                        }
                        gloo_timers::future::sleep(Duration::from_millis(300)).await;
                        title.set(init_string.to_string());
                    }
                    gloo_timers::future::sleep(Duration::from_millis(1500)).await;
                    loop {
                        if init_string.len() == 0 {
                            break;
                        }
                        init_string.pop();
                        gloo_timers::future::sleep(Duration::from_millis(200)).await;
                        title.set(init_string.to_string());
                    }
                }
            }
        }
    });

    cx.render(rsx!(
        h2 {
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
            border_right: "3px solid black",
            animation: "blink 0.75s infinite step-end",
            padding_right: "1px",
            white_space: "nowrap",
            overflow: "hidden",
            word_break: "break-all",
            font_size: "300%",
            font_family: "monospace, 'WenQuanYi Micro Hei'",
            style { include_str!("css/test.css") }
            "{states}"
        }
    ))
}

fn App(cx: Scope) -> Element {
    let a = vec!["正在__建设中...".to_string(), "Under Construction...".to_string()];
    cx.render(rsx! {
        div { width: "100%", height: "100vh", position: "relative", background_color: "#f1f1f1", Content { a: a } }
    }
    )
}