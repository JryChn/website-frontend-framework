#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn PageNotFound(cx: Scope, route:String) -> Element {
    render!( div { "Page not Found" } )
}
