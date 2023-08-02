#![allow(non_snake_case)]

use crate::model::config::TimerContent;
use dioxus::html::animate;
use dioxus::prelude::*;
use getrandom::getrandom;
use rand::Rng;

#[derive(Props, PartialEq)]
pub struct TimerContext {
    timer_intro: Vec<TimerContent>,
}

pub fn Timer(cx: Scope<TimerContext>) -> Element {
    let rand_top = || rand::thread_rng().gen_range(20..60);
    let delay_time = || rand::thread_rng().gen_range(0..cx.props.timer_intro.len() * 25);
    cx.render(
        rsx!(
            style { include_str!("css/timer.css") }
            div {
                id: "timer",
                class: "w-screen min-h-[400px] shadow-inner bg-gradient-to-br  from-pink-100 to-indigo-300 relative overflow-hidden",
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
                cx.props.timer_intro.iter().map(|t|{
                    rsx!(
                div {
                    id: "timer_content",
                    class: "border border-gray-300 w-96 h-32 rounded-2xl absolute drop-shadow-2xl bg-indigo-50 translate-x-96",
                    top: "{rand_top()}%",
                    right: "-300px",
                    animation_name: "parallel_move",
                    animation_duration: "45s",
                    animation_delay:"{delay_time()}s",
                    animation_iteration_count:"infinite",
                    img {
                        src: "{t.image}",
                        alt: "",
                        class: "w-1/5 h-5/6 absolute top-3 left-3 rounded-xl"
                    }
                    span { class: "block w-3/5 h-5/6 absolute top-3 right-7 p-2 overflow-ellipsis overflow-hidden",
                        "{t.content}"
                    }
                }
                    )

                })
            }
        )
    )
}
