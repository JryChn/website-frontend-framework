use dioxus::prelude::*;

use crate::model::{ConfigurationTemplate, Welcome};
use crate::Route;

#[component]
pub fn WelcomePage(welcome:Welcome) -> Element {
    let animation_url = if gloo::utils::document_element().class_list().contains("dark") {
        &welcome.animation_url.dark
    } else {
        &welcome.animation_url.light
    };
    rsx!{
        div {
            id: "welcome_page",
            class: "w-screen h-screen min-h-[800px] bg-gray-50",
            Link { to: Route::HomePage {},
                h1 {
                    id: "title_logo",
                    class: "relative top-4 left-7 text-[2vw] leading-none w-[7em] h-auto uppercase font-bold break-words",
                    style: "font-family: JetBrains Mono",
                    "{welcome.title}"
                }
            }
            div {
                id: "welcome_video",
                class: "relative -translate-x-1/2 left-1/2 top-1/2 -translate-y-1/2 w-[55vw] h-[21vw] border-b-2 border-b-black pb-6 px-20",
                mix_blend_mode: "multiply",
                video {
                    class: "w-full h-full object-fill",
                    src: "{animation_url}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true"
                }
            }
            div {
                id: "light_line_box",
                class: "absolute h-[15vw] top-0 right-[30vw]",
                div {
                    id: "light_line",
                    class: "relative left-1/2 w-0 h-4/5 border-[1.5px] border-black md:cursor-pointer"
                }
                button {
                    id: "light_bold",
                    class: "relative w-11 h-11 translate-x-2.5 -translate-y-1",
                    onclick: |_e| {
                        let dom = gloo::utils::document_element();
                        dom.class_list().toggle("dark").expect("Error when toggle dark");
                    },
                    img { src: "/static/bulb.svg" }
                }
            }
            div {
                id: "down_scroll_pointer",
                class: "absolute bottom-5 left-1/2 -translate-x-1/2 w-8 h-8",
                img {
                    class: "relative mx-auto w-full h-full top-1/2 animate-bounce cursor-pointer",
                    src: "/static/down.svg"
                }
            }
        }
    }
}
