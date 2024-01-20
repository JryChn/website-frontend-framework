#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::NAV;

pub fn Navigate(cx: Scope) -> Element {
    let navigator = use_shared_state::<NAV>(cx).unwrap().read().0.clone();
    let nav_list = navigator.iter().map(|url| {
        rsx!(
            Link { class: "text-center align-middle p-3", to: "{url.1}", "{url.0}" }
        )
    });
    let nav_sidebar_switch = use_state(cx,||{false});
    cx.render(
        rsx!(
            nav {
                id: "navigator",
                class: "fixed top-0 right-0 w-[10vw] h-[10vw] rounded-full translate-x-1/2 -translate-y-1/2 bg-black cursor-pointer",
                onclick: |_e|  {
                    nav_sidebar_switch.set(true);
                },
                div{
                    id: "hamburger",
                    class: "relative bottom-0 left-0 w-14 h-14 border border-gray-50 flex flex-col justify-evenly z-10",
                (1..4).map(|_i|{
                    rsx!(
                    div{
                        class:"w-3/4 h-1 border-2 border-gray-100  bg-gray-100 rounded-lg mx-auto dark:bg-gray-50 dark:border-gray-50",
                    }
                    )
                }
                ),
                }
            }
                if *nav_sidebar_switch.get(){
                    rsx!(
                nav {
                    id: "navigator_sidebar",
                    class: "fixed right-0 top-0 w-72 bg-black rounded-l-xl shadow-zinc-700 shadow-2xl h-screen z-20",
                    div {
                        dangerous_inner_html: "sign",
                        onclick: |_e| {
                            nav_sidebar_switch.set(false);
                        }
                    }
                    div { class: "w-3/4 h-1/2 mx-auto translate-y-1/2 cursor-pointer",
                        ul { class: "text-neutral-200 text-2xl flex flex-col justify-evenly",
                            nav_list
                        }
                    }
                }

                    )
            }
        )
    )
}
