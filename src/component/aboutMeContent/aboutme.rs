use std::time::Duration;

use dioxus::prelude::*;

#[component]
pub fn AboutMe(title: String, subtitle: String, image1: String, image2: String) -> Element {
    rsx! {
        div { class: "relative w-screen min-h-[1000px] top-18 select-none cursor-default bg-gray-100 dark:bg-gray-950",
            AboutMeTitle { title, subtitle, image: &image2 }
            Circle {}
            AboutMeImages { image_url: image1, image2_url: &image2 }
        }
    }
}

#[component]
fn AboutMeTitle(title: String, subtitle: String, image: String) -> Element {
    let mut typing_words = use_signal(|| "".to_string());
    // words blink type animation
    let _ = use_resource(move || {
        let subtitle = subtitle.clone();
        async move {
            loop {
                for i in vec![subtitle.clone()].iter() {
                    let whole_str = i;
                    gloo::timers::future::sleep(Duration::from_millis(1000)).await;
                    let mut init_string = "".to_string();
                    for i in whole_str.chars() {
                        if i != '_' {
                            init_string.push(i);
                        }
                        gloo::timers::future::sleep(Duration::from_millis(300)).await;
                        typing_words.set(init_string.to_string());
                    }
                    gloo::timers::future::sleep(Duration::from_millis(1500)).await;
                    loop {
                        if init_string.len() == 0 {
                            break;
                        }
                        init_string.pop();
                        gloo::timers::future::sleep(Duration::from_millis(200)).await;
                        typing_words.set(init_string.to_string());
                    }
                }
            }
        }
    });
    rsx! {
        div { class: "absolute w-full h-64 top-44 md:w-96 md:left-[12vw] md:animate-showFromLeft dark:text-white",
            div { class: "h-1/3 flex flex-row items-center justify-center text-center font-semibold text-3xl ",
                "{title}"
            }
            img {
                class: "w-72 h-96 rounded-tl-[149px] shadow-[6px_1px_8px_3px_rgba(0,0,0,0.25)] mx-auto my-20 scale-125 object-cover md:hidden",
                src: "{image}"
            }
            div { class: "w-5/6 mx-auto border-t border-black md:left-0 md:w-12 md:absolute dark:border-white" }
            div { class: "w-5/6 mx-auto h-full my-8 text-sm font-normal text-left tracking-normal md:my-4 md:w-full md:left-0 md:absolute",
                span { "/*" }
                span { class: "border-r border-black px-2 animate-wordBlink", "{typing_words}" }
            }
        }
    }
}

#[component]
fn Circle() -> Element {
    rsx! {
        div {
            id: "circle",
            class: "hidden absolute w-96 h-40 top-[460px] md:left-[50vw] md:block",
            div {
                id: "circle_1",
                class: "border w-40 h-40 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                div {
                    id: "",
                    class: "border w-32 h-32 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                    div {
                        id: "",
                        class: "border w-24 h-24 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                        div {
                            id: "",
                            class: "border w-16 h-16 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                            div {
                                id: "",
                                class: "border w-8 h-8 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center bg-[rgb(96,24,123)]"
                            }
                        }
                    }
                }
            }
            div {
                id: "circle_2",
                class: "border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-full translate-x-2/3"
            }
            div {
                id: "circle_3",
                class: "border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-[200%] translate-x-[120%]"
            }
        }
    }
}
#[component]
fn AboutMeImages(image_url: String, image2_url: String) -> Element {
    rsx! {
        div { class: "hidden md:flex absolute top-52 right-[10vw] animate-showFromRight",
            img {
                id: "image_1",
                class: "w-56 h-96 rounded-tl-[110px] shadow-[-9px_8px_25px_3px_rgba(0,0,0,0.25)] mx-6 my-2 object-cover",
                src: "{image_url}"
            }
            img {
                id: "image_2",
                class: "w-72 h-96 rounded-tl-[149px] shadow-[6px_1px_8px_3px_rgba(0,0,0,0.25)] mx-6 my-2 object-cover",
                src: "{image2_url}"
            }
        }
    }
}
