#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Router};

pub fn Article(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div{
                id:"article",
                div{
                    id:"article_image"
                }
                div{
                    id:"article_content",
                    div{
                        id:"article_content_index"
                    }
                    div{
                        id:"article_content_box"
                    }
                }
            }
        )
    )
}
