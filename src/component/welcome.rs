use std::time::Duration;

use dioxus::html::{div, link, thead};
use dioxus::prelude::*;
use reqwest;

pub fn WelcomeContent(cx: Scope) -> Element {
    let states = use_state(cx, || {
        "".to_string()
    });
    use_future(cx, (), |_| {
        to_owned![states];
        let whole_str = vec!["正在__建设中...", "Under Cons_tructi__on"];
        async move {
            loop {
                for i in whole_str.iter()
                {
                    let whole_str = i;
                    gloo_timers::future::sleep(Duration::from_millis(1000)).await;
                    let mut init_string = "".to_string();
                    for i in whole_str.chars() {
                        if i != '_' {
                            init_string.push(i);
                        }
                        gloo_timers::future::sleep(Duration::from_millis(300)).await;
                        states.set(init_string.to_string());
                    }
                    gloo_timers::future::sleep(Duration::from_millis(1500)).await;
                    loop {
                        if init_string.len() == 0 {
                            break;
                        }
                        init_string.pop();
                        gloo_timers::future::sleep(Duration::from_millis(200)).await;
                        states.set(init_string.to_string());
                    }
                }
            }
        }
    });

    cx.render(rsx!(
        div{
            display:"block",
            height: "100vh", position: "relative",
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
            font_size: "5vw",
            font_weight: "bold",
            font_family: "AliMama, monospace",
            style { include_str!("../css/welcome_animation.css"),include_str!("../css/font_ali.css") }
            "{states}"
        }
        }
    ))
}
