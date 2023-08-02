#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::use_route;

#[derive(Props, PartialEq)]
pub struct ArticleContext {
    url: String,
}
pub fn Article(cx: Scope<ArticleContext>) -> Element {
    let id = use_route(&cx).last_segment()?;
    cx.render(rsx!(
        div { id: "article",
            div { id: "article_image" }
            div { id: "article_content",
                div { id: "article_content_index" }
                div { id: "article_content_box" }
            }
        }
    ))
}
