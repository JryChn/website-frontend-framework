#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::NAV;

#[component]
pub fn Navigate(cx: Scope) -> Element {
    let navigator = use_shared_state::<NAV>(cx).unwrap().read().0.clone();
    let nav_sidebar_switch = use_state(cx,||{false});
    let nav_list = navigator.iter().map(|url| {
        rsx!(
            Link { class: "text-center align-middle p-3 m-1.5", to: "{url.1}", "{url.0}" onclick:|_e|{
                    nav_sidebar_switch.set(false);
            } }
            img { src: "/static/straightLine.svg" }
        )
    });
    render!(
            nav {
                id: "navigator",
                class: "fixed top-0 right-0 w-[10vw] h-[10vw] rounded-full translate-x-1/2 -translate-y-1/2 bg-gray-50 cursor-pointer md:bg-black",
                onclick: |_e| {
                    nav_sidebar_switch.set(true);
                }
            }
            div {
                id: "hamburger",
                class: "fixed top-3 right-4 w-10 h-8 flex flex-col justify-evenly cursor-pointer",
                onclick: |_e| {
                    nav_sidebar_switch.set(true);
                },
                (1..4).map(|_i|{
                    rsx!(
                    div{
                        class:"w-3/4 h-1 border-black border-2 bg-black rounded-lg mx-auto md:border-gray-50 md:bg-gray-50",
                    }
                    )
                }
                )
            }
            if *nav_sidebar_switch.get(){
                rsx!(
                img {
                    src: "/static/close.svg",
                    class: "fixed right-4 top-3 cursor-pointer z-30",
                    onclick: |_e| {
                        nav_sidebar_switch.set(false);
                    }
                }
            nav {
                id: "navigator_sidebar",
                class: "fixed right-0 top-0 w-screen bg-black shadow-zinc-700 shadow-2xl h-screen z-20 md:animate-slideFromR2L",
                div { class: "fixed right-1/2 top-1/2 w-44 h-80 -translate-y-1/2 translate-x-1/2 cursor-pointer md:right-1/4 md:translate-x-0",
                    ul { class: "text-neutral-200 text-2xl flex flex-col justify-evenly",
                        nav_list
                    }
                }
                div { class: "fixed left-[15%] top-1/2 w-1/3 h-[40%] -translate-y-1/2 bg-white rounded-[37px] hidden md:block" }
                        })
            }
    )
}
