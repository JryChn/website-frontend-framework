#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Navigate(cx: Scope) -> Element {
    cx.render(
        rsx!(
            nav { id: "navigate", div { id: "navigate_style", onclick: |e| {}, onscroll: |e| {} }
                div{
                    id:"nav_content",
                    ul{
                        li{
                            Link{
                                to:"#article",
                                "Article"
                            }
                        }
                        li{
                            Link{
                                to:"#aboutme",
                                "AboutMe"
                            }
                        }
                        li{
                            Link{
                                to:"#story",
                                "Story"
                            }
                        }
                    }
                }
            }
        )
    )
}
