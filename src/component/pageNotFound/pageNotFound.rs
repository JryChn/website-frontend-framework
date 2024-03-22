#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route:Vec<String>) -> Element {
    rsx!{ div { "Page not Found" } }
}
