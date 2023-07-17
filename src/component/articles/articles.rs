#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Router};

pub fn Articles(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div { id: "article",
                div { id: "article_content",
                    div { id: "article_one",
                        div { id: "article_one_image" }
                        div { id: "article_one_index" }
                        Router { 
                            Link { to: "/article/***", "Read" }
                        }
                    }
                    div { id: "article_two",
                        div { id: "article_two_image" }
                        div { id: "article_two_index" }
                        Router { 
                            Link { to: "/article/***", "Read" }
                        }
                    }
                    div { id: "article_three",
                        div { id: "article_three_image" }
                        div { id: "article_three_index" }
                        Router { 
                            Link { to: "/article/***", "Read" }
                        }
                    }
                }
            }
        )
    )
}
