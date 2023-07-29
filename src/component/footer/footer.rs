#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(
        rsx!(
            footer { class: "w-screen h-14 bg-zinc-950", span { class: "inline-block text-gray-50 relative top-4 left-1/2 -translate-x-1/2" } }
        )
    )
}
