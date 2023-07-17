#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(
        rsx!(
            nav { id: "navigate", div { id: "navigate_style", onclick: |e| {}, onscroll: |e| {} } }
        )
    )
}
