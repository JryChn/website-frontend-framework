#![allow(non_snake_case)]

use std::time::Duration;
use dioxus::html::video;

use dioxus::prelude::*;
use dioxus_router::{Link, Router};
use reqwest;

#[derive(Props, PartialEq)]
pub struct SingleWelcomeContext {
    title: String,
    animation_video_url: (String, String),
    content: Vec<String>,
}

pub fn SingleWelcome(cx: Scope<SingleWelcomeContext>) -> Element {
    let typing_words = use_state(cx, || {
        "".to_string()
    });
    use_future(cx, (), |_| {
        to_owned![typing_words];
        let whole_str = cx.props.content.to_owned();
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
                        typing_words.set(init_string.to_string());
                    }
                    gloo_timers::future::sleep(Duration::from_millis(1500)).await;
                    loop {
                        if init_string.len() == 0 {
                            break;
                        }
                        init_string.pop();
                        gloo_timers::future::sleep(Duration::from_millis(200)).await;
                        typing_words.set(init_string.to_string());
                    }
                }
            }
        }
    });
    cx.render(rsx!(
        style { include_str!("css/single_welcome.css") }
        div { id: "welcome_page",
            h1 { id: "title_logo" ,
            Router{
                    Link{
                        to:"/",
                    }
                }}
            h2 { id: "subtitle_wordplay", "{typing_words}" }
            div { id: "welcome_video",
                video{
                    src:"",
                    autoplay:"true",
                    muted:"true",
                    "loop":"loop",
                }
            }
            div { id: "dark_mode_switcher" }
            div { id: "book_my_time" ,
                Router{
                    Link{
                        to:"/calendar",
                    }
                }
            }
        }
    ))
}
