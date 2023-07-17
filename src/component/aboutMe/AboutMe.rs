#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;

pub fn AboutMe(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div { id: "about_me",
                div { id: "about_me_video" }
                div { id: "about_me_words" }
                Link { id: "about_me_link", to: "/aboutMe", "Read More" }
            }
        )
    )
}
