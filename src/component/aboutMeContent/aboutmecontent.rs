#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn AboutMeContent(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div { id: "aboutme_content",
                div { id: "aboutme_content_contact" }
                div { id: "aboutme_content_main",
                    div { id: "aboutme_content_image" }
                    div { id: "aboutme_content_description" }
                }
                div { id: "aboutme_content_stage" }
                div { id: "aboutme_content_github" }
            }
        )
    )
}
