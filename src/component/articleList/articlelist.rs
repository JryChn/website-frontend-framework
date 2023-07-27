#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Router};

pub fn ArticleList(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div { id: "article_list",
                div { id: "article_list_head" }
                div { id: "article_list_box" }
                div { id: "article_list_sidebar" }
            }
        )
    )
}
