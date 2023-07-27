#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;

#[derive(Props, PartialEq)]
pub struct AboutMeContext {
    about_me_video_url: String,
    about_me_title:String,
    about_me_description:String,
}
pub fn AboutMe(cx: Scope) -> Element {
    cx.render(
        rsx!(
            div {
                id: "about_me",
                class: "w-screen min-h-[1000px] bg-gray-200 dark:bg-black",
                div { id: "about_me_box", class: "w-3/4 h-1/6 mx-auto",
                    h2 {
                        id: "about_me_intro",
                        class: "text-5xl font-mono font-bold text-center mb-10 dark:text-gray-50",
                        "About Me"
                    }
                    div {
                        id: "about_me_content",
                        class: "w-[90%] h-[700px] border border-black mx-auto relative",
                        video {
                            src: "{about_me_video_url}",
                            class: "bg-red-200 w-full h-full brightness-75 contrast-75",
                            autoplay: "true",
                            muted: "true",
                            "loop": "loop",
                            playsinline: "true"
                        }
                        h3 {
                            id: "about_me_title",
                            class: "absolute left-12 top-[60%] font-bold font-mono text-3xl text-zinc-200",
                            "{about_me_title}"
                        }
                        span {
                            id: "about_me_description",
                            class: "absolute left-12 top-[70%] font-sans text-xl break-words w-1/3 text-gray-200",
                            "{about_me_description}"
                        }
                    }
                }
                div { id: "about_me_video" }
                Link { id: "about_me_link", to: "/aboutMe", "Find Me" }
            }
        )
    )
}