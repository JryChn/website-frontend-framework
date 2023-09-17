#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::Route;

#[derive(Props, PartialEq)]
pub struct AboutMeContext {
     name: String,
     description: String,
     video: String,
}
pub fn AboutMe(cx: Scope<AboutMeContext>) -> Element {
    cx.render(
        rsx!(
            div {
                id: "about_me",
                class: "w-screen min-h-[1000px] bg-gray-200 dark:bg-black",
                div { id: "about_me_box", class: "w-3/4 h-1/6 mx-auto",
                    h2 {
                        id: "about_me_intro",
                        class: "text-5xl font-mono font-bold text-center p-10 dark:text-gray-50",
                        "About Me"
                    }
                    Link {
                        to: Route::AboutMeContent {},
                        id: "about_me_content",
                        class: "w-[90%] h-[700px] border border-black mx-auto relative block",
                        video {
                            src: "{cx.props.video}",
                            class: "w-full h-full brightness-75 contrast-75 object-fill",
                            autoplay: "true",
                            muted: "true",
                            "loop": "true",
                            playsinline: "true"
                        }
                        h3 {
                            id: "about_me_title",
                            class: "absolute left-12 top-[60%] font-bold font-mono text-3xl text-zinc-200",
                            "{cx.props.name}"
                        }
                        span {
                            id: "about_me_description",
                            class: "block absolute left-12 top-[70%] font-sans text-xl  w-1/3 text-gray-200",
                            "{cx.props.description}"
                        }
                    }
                }
            }
        )
    )
}
