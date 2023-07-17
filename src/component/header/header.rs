#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(
        rsx!(
            nav { id: "navigate", div { id: "navigate_style", onclick: |e| {}, onscroll: |e| {} } }
        )
    )
}
