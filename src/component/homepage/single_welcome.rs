use dioxus::prelude::*;
use manganis::mg;

use crate::model::Welcome;
use crate::Route;
use crate::utils::wireLaneWithSvg::wireLaneWithSvg;

#[component]
pub fn WelcomePage(welcome: Welcome) -> Element {
    let bulb =mg!(file("src/assets/svg/bulb.svg"));
    rsx! {
        div {
            id: "welcome_page",
            class: "w-screen h-screen min-h-[800px] bg-gray-50 dark:bg-black",
            Link { to: Route::HomePage {},
                h1 {
                    id: "title_logo",
                    class: "relative top-4 left-7 text-[2vw] leading-none w-[7.5em] h-auto uppercase font-bold break-words dark:text-white md:animate-showFromUp md:delay-1000",
                    style: "font-family: JetBrains Mono",
                    "{welcome.title}"
                }
            }
            div {
                id: "welcome_video",
                class: "relative -translate-x-1/2 left-1/2 top-1/2 -translate-y-1/2 w-[55vw] h-[21vw] border-b-2 border-b-black pb-6 px-20 dark:border-b-white dark:hidden",
                mix_blend_mode: "multiply",
                video {
                    class: "w-full h-full object-fill",
                    src: "{&welcome.animation_url.light}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true"
                }
            }
            div {
                id: "welcome_video_dark",
                class: "hidden relative -translate-x-1/2 left-1/2 top-1/2 -translate-y-1/2 w-[55vw] h-[21vw] border-b-2 border-b-black pb-6 px-20 dark:border-b-gray-100 dark:block",
                video {
                    class: "w-full h-full object-fill",
                    src: "{&welcome.animation_url.dark}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true"
                }
            }
            div {
                id: "light_line_box",
                class: "absolute h-[15vw] top-0 right-[30vw] md:right-[10vw]",
                div {
                    id: "md_light",
                    class: "hidden md:block dark:invert",
                    onmounted: move |_| {
                        eval(&wireLaneWithSvg(bulb, "md_light"));
                    }
                }
                div {
                    id: "light_line",
                    class: "relative left-1/2 w-0 h-4/5 border-[1.5px] border-black md:hidden dark:border-white"
                }
                button {
                    id: "light_bold",
                    class: "relative h-11 translate-x-0.5 -translate-y-1 cursor-pointer md:hidden dark:after:block dark:after:w-24 dark:after:h-24 dark:after:absolute dark:after:bg-yellow-400 dark:after:round-full dark:after:-translate-x-1/3 dark:after:-translate-y-1/4 dark:after:blur-3xl dark:after:animate-infiniteShowing",
                    onclick: |_e| {
                        let dom = gloo::utils::document_element();
                        dom.class_list().toggle("dark").expect("Error when toggle dark");
                        let body = gloo::utils::body();
                        body.class_list()
                            .toggle("dark:bg-gray-950")
                            .expect("Error when change background");
                    },
                    img { class: "dark:invert", src: bulb }
                }
            }
            div {
                id: "down_scroll_pointer",
                class: "absolute bottom-5 left-1/2 -translate-x-1/2 w-8 h-8 cursor-pointer",
                img {
                    class: "relative mx-auto w-full h-full animate-bounce dark:invert",
                    src: mg!(file("src/assets/svg/down.svg"))
                }
            }
        }
    }
}
