#![allow(non_snake_case)]

use dioxus::html::animate;
use dioxus::prelude::*;
use getrandom::getrandom;
use rand::Rng;

#[derive(Props, PartialEq)]
pub struct TimerContext {
    timer_content_url: String,
}
pub fn Timer(cx: Scope) -> Element {
    let rand_top=rand::thread_rng().gen_range(20..60);
    let rand_right=rand::thread_rng().gen_range(0..400);
    cx.render(
        rsx!(
            style { include_str!("css/timer.css") }
            div {
                id: "timer",
                class: "w-screen min-h-[400px] shadow-inner bg-gradient-to-br  from-pink-100 to-indigo-300 relative",
                h2 {
                    id: "timer_title",
                    class: "text-5xl font-mono font-bold text-center mb-10 translate-y-5",
                    "Timer"
                }
                div {
                    id: "timer_decoration",
                    class: "grid grid-cols-11 grid-rows-[11] gap-0 w-full h-[300px]",
                    (1..45).map(|_|rsx!(
                        div{
                            class:"border border-gray-300"
                        }
                    ))
                }
                div {
                    id: "timer_content",
                    class: "border border-gray-300 w-96 h-32 rounded-2xl absolute drop-shadow-2xl bg-indigo-50 translate-x-96",
                    top: "{rand_top}%",
                    right: "{rand_right}px",
                    animation_name: "parallel_move",
                    animation_duration: "45s",
                    img {
                        src: "",
                        alt: "",
                        class: "w-1/5 h-5/6 absolute top-3 left-3 rounded-xl"
                    }
                    span { class: "block w-3/5 h-5/6 absolute top-3 right-7 p-2 overflow-ellipsis overflow-hidden",
                        ""
                    }
                }
            }
        )
    )
}