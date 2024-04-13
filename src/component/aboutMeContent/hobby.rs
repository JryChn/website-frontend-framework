use dioxus::prelude::*;

use crate::component::aboutMeContent::HobbyContent;

#[component]
pub fn Hobby(hobbys: Vec<HobbyContent>) -> Element {
    let mut render_block = Vec::new();
    let mut flag = 0;
    hobbys.iter().for_each(|h| {
        match flag {
            0 => {
                render_block.push(rsx! { RightImageBlock { bg_color: "bg-[rgb(91,128,86)] dark:bg-[rgb(41,49,50)]", hobby: h.clone() } });
            }
            1 => {
                render_block.push(rsx! { LeftImageBlock { bg_color: "bg-[rgb(108,131,175)] dark:bg-[rgb(18,20,32)]", hobby: h.clone() } });
            }
            _ => {
                render_block.push(rsx! { RightImageBlock { bg_color: "bg-[rgb(173,178,131)] dark:bg-[rgb(40,54,24)]", hobby: h.clone() } });
            }
        }
        flag = (flag + 1) % 3;
    });
    rsx! {
        div { class: "relative -top-44 cursor-default select-none md:top-0",
            div { class: "hidden absolute w-5/6 border border-b-black right-0 md:block dark:border-b-gray-100" }
            div { class: "text-4xl top-6 flex justify-center md:relative md:right-[20%] dark:text-white",
                "Sport"
            }
            div { class: "relative -top-14 md:top-20",
                div { class: "absolute text-4xl top-6 left-[50vw] -translate-x-1/2 md:hidden",
                    "Sport"
                }
                for h in render_block {
                    {h}
                }
            }
        }
    }
}

#[component]
fn RightImageBlock(bg_color: String, hobby: HobbyContent) -> Element {
    rsx! {
        div { class: "w-full {bg_color} py-2 flex flex-col shadow-[inset_0px_4px_4px_0px_rgba(0,0,0,0.25)] md:relative md:h-[490px]",
            img {
                class: "w-full h-72 mb-8 mt-20 shadow-[0px_12px_12px_0px_rgba(0,0,0,0.25)] object-cover md:absolute md:w-[609px] md:h-[313px] md:-top-1/2 md:translate-y-1/3 md:right-12 md:rounded-[156.5px_0_0_156.5px]",
                src: "{hobby.image_url}"
            }
            div { class: "w-[90%] min-h-72 my-8 flex flex-col mx-auto md:w-1/3 md:h-1/2 md:absolute md:left-10 dark:text-white",
                div { class: "flex items-center justify-center",
                    div { class: "font-normal text-4xl mx-auto md:mx-0 md:-translate-x-full",
                        "{hobby.title}"
                    }
                }
                div { class: "flex items-center justify-center my-8 font-light text-4xl",
                    "{hobby.description}"
                }
            }
        }
    }
}
#[component]
fn LeftImageBlock(bg_color: String, hobby: HobbyContent) -> Element {
    rsx! {
        div { class: "w-full {bg_color} py-2 flex flex-col shadow-[inset_0px_4px_4px_0px_rgba(0,0,0,0.25)] md:relative md:min-h-[490px]",
            img {
                class: "w-full h-72 mb-8 mt-20 shadow-[0px_12px_12px_0px_rgba(0,0,0,0.25)] object-cover md:shadow-[0px_-8px_6px_0px_rgba(0,0,0,0.25)] md:absolute md:w-[609px] md:h-[313px] md:-top-1/2 md:translate-y-1/3 md:left-12 md:rounded-[0_156.5px_156.5px_0]",
                src: "{hobby.image_url}"
            }
            div { class: "w-[90%] min-h-72 my-8 flex flex-col mx-auto md:w-1/3 md:h-1/2 md:absolute md:right-10 dark:text-white",
                div { class: "flex items-center justify-center",
                    div { class: "font-normal text-4xl mx-auto md:mx-0 md:translate-x-full",
                        "{hobby.title}"
                    }
                }
                div { class: "flex items-center justify-center my-8 font-light text-4xl",
                    "{hobby.description}"
                }
            }
        }
    }
}
