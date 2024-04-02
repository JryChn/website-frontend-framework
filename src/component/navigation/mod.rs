#![allow(non_snake_case)]


use dioxus::prelude::*;
use manganis::mg;

use crate::NAVIGATOR;

#[component]
pub fn Navigate() -> Element {
    let mut nav_sidebar_switch = use_signal(|| false);
    rsx! {
        nav {
            id: "navigator",
            class: "fixed top-0 right-0 w-48 h-48 rounded-full translate-x-1/2 -translate-y-1/2 bg-gray-50 cursor-pointer md:bg-black",
            onclick: move |_| {
                nav_sidebar_switch.set(true);
            }
        }
    Hamburger{nav_sidebar_switch}
    SideBar{nav_sidebar_switch}
    }
}

#[component]
fn Hamburger(mut nav_sidebar_switch: Signal<bool>) -> Element {
    rsx! {
            div {
                id: "hamburger",
                class: "fixed top-3 right-4 w-10 h-8 flex flex-col justify-evenly cursor-pointer",
                onclick: move |_| {
                    nav_sidebar_switch.set(true);
                },
            for _ in (1..4){
                    div{
                        class:"w-3/4 h-1 border-black border-2 bg-black rounded-lg mx-auto md:border-gray-50 md:bg-gray-50",
                    }
            }
            }
    }
}

#[component]
fn SideBar(nav_sidebar_switch: Signal<bool>) -> Element {
    let mut current_hover = use_signal(|| String::new());
    let current_template = match current_hover.read().as_str() {
        "AboutMe" => AboutMeTemplate(),
        "Calendar" => CalendarTemplate(),
        _ => ArticleTemplate(),
    };
    if nav_sidebar_switch() {
        rsx! {
                    img {
                        src: mg!(file("src/assets/svg/close.svg")),
                        class: "fixed right-4 top-3 cursor-pointer z-30",
                        onclick: move |_| {
                            nav_sidebar_switch.set(false);
                        }
                    }
                nav {
                    id: "navigator_sidebar",
                    class: "fixed right-0 top-0 w-screen bg-black shadow-zinc-700 shadow-2xl h-screen z-20 md:nimate-slideFromR2L",
                    div { class: "fixed right-1/2 top-1/2 w-44 h-80 -translate-y-1/2 translate-x-1/2 md:right-1/4 md:translate-x-0",
                        ul { class: "text-neutral-200 text-2xl flex flex-col justify-evenly",
                for url in NAVIGATOR(){
                div { class: "text-center align-middle p-3 m-1.5 cursor-pointer ",
                                        onclick: move |_|{
                        nav_sidebar_switch.set(false);
                },

                                onmouseover: move |_| {
                                    let url = url.clone().0 ;
                                    current_hover.set(url);
                                },
                                Link{to: "{url.1}", "{url.0}"}
                            }
                img { src: mg!(file("src/assets/svg/straightLine.svg")) }

        }
                        }
                    }
                    div { class: "fixed left-[15%] top-1/2 w-[35vw] h-[19vw] -translate-y-1/2 bg-white rounded-[37px] overflow-hidden hidden shadow-zinc-800 shadow-inner md:flex",
                        div{
                                class:"w-full h-full flex flex-col my-3",
                                div{
                                    class:"h-[10%] flex flex-row border-b px-10",
                                    div{
                                        class:"w-10 flex items-center justify-center",
                                        div{
                                            class:"w-3/4 h-3/4 bg-zinc-400 animate-pulse rounded-2xl"
                                        }
                                    }
                                    div{
                                        class:"flex-1 flex items-center justify-evenly",
                                        for _ in (0..4){
                                        div{
                                            class:"w-[10%] h-3/4 bg-zinc-200 animate-pulse rounded-2xl"
                                        }
                                        }
                                    }
                                }
                                div{
                                    class:"flex-1 w-4/5 mx-auto",
                 }                   // each implementation here
                        {current_template}
                                }
                            }
                        }
                            }
    } else {
        rsx! {}
    }
}

#[component]
fn ArticleTemplate() -> Element {
    rsx! {

    div{
                class:"w-full h-full flex animate-pulse",
                div{
                    class:"w-2/3 flex flex-col items-center translate-y-[15%]",
                    for _ in (0..3){
                    div{
                        class:"w-3/4 h-[10%] rounded-2xl bg-gray-200 my-2"
                    }
                    }
                }
                div{
                    class:"flex-1",
                    div{
                        class:"w-3/4 h-3/5 bg-zinc-200 mx-auto translate-y-1/4 "
                    }
                }
            }
    }
}
#[component]
fn CalendarTemplate() -> Element {
    rsx! {
    div{
                class:"w-full h-full flex animate-pulse",
                div{class:"w-3/4 h-3/4 mx-auto bg-zinc-100 my-3 rounded-2xl flex flex-col overflow-hidden" ,
                                        div{
                                            class:"h-[10%] bg-gray-100 my-2",
                                            div{
                                                class:" w-[10%] h-3/4 bg-zinc-400 translate-x-full",
                                            }
                                            div{
                                                class:"w-4/5 border-b border-zinc-700 mx-auto my-1",
                                            }
                                        }
                                        div{
                                            class:"w-4/5 flex-1 flex flex-row mx-auto",
                                            div{
                                                class:" w-[10%] bg-blue-100 translate-x-1/4"
                                            }
                                            div{
                                                class:" flex-1 flex flex-col items-center justify-center",
                                                div{
                                                    class: "relative w-[90%] h-[90%] bg-gray-200 rounded-2xl",
                                                    div{
                                                       class:"w-[10%] h-1/4 bg-red-900 rounded-md absolute top-1/2 left-1/3"
                                                    }
                                                    div{
                                                       class:"w-[10%] h-1/4 bg-red-900 rounded-md absolute top-1/4 left-[10%]"
                                                    }
                                                    div{
                                                       class:"w-[10%] h-1/2 bg-red-900 rounded-md absolute top-1/4 right-1/3"
                                                    }

                                                }
                                            }
                                        }

                }
            }
    }
}
fn AboutMeTemplate() -> Element {
    rsx! {

        div{
                    class:"relative w-full h-full animate-pulse",

                div{
                    class:"absolute top-[10%] left-[10%] w-1/5 h-1/5 bg-gray-100"
                }
                div{
                    class:"absolute top-[5%] right-[10%] -translate-x-1/2 w-[10%] h-1/4 bg-gray-100 rounded-tl-[80px]"
                }
                div{
                    class:"absolute top-[5%] right-1/3 -translate-x-1/2 w-[10%] h-1/4 bg-gray-100 rounded-tl-[40px]"
                }
                div{
                    class:"absolute top-1/2 left-1/2 -translate-x-1/2 w-1/2 h-1/5 bg-gray-100 rounded-md"
                }
        }

    }
}
