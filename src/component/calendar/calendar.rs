#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Calendar(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div { id: "calendar",
                div { id: "calendar_box" }
                div { id: "calendar_button" }
            }
        )
    )
}