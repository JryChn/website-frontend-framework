#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;

#[derive(Props, PartialEq)]
pub struct TimerContext {
    about_me_video_url: String,
}
pub fn Timer(cx: Scope) -> Element {
    cx.render(
        rsx!(
        )
    )
}