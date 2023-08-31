#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Story(cx: Scope) -> Element {
    cx.render(rsx!(
        div { id: "story",
            // div { id: "story_content",
            //     div { id: "story_one",
            //         div { id: "story_one_image" }
            //         div { id: "story_one_index" }
            //         Router {
            //             Link { to: "/story/***", "Read" }
            //         }
            //     }
            //     div { id: "story_two",
            //         div { id: "story_two_image" }
            //         div { id: "story_two_index" }
            //         Router {
            //             Link { to: "/story/***", "Read" }
            //         }
            //     }
            //     div { id: "story_three",
            //         div { id: "story_three_image" }
            //         div { id: "story_three_index" }
            //         Router {
            //             Link { to: "/story/***", "Read" }
            //         }
            //     }
            // }
        }
    ))
}
