#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct CalendarContext {
    url: String,
}
pub fn Calendar(cx: Scope<CalendarContext>) -> Element {
    cx.render(rsx!(
        div { id: "calendar",
            div { id: "calendar_box" }
            div { id: "calendar_button" }
        }
    ))
}
