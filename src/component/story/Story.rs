#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Story(cx: Scope) -> Element {
    cx.render(rsx!( div { id: "story" } ))
}
