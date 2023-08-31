#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[inline_props]
pub fn PageNotFound(cx: Scope, route:Vec<String>) -> Element {
    cx.render(rsx!( div { "Page not Found" } ))
}
