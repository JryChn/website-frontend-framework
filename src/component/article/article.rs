#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn Article(cx: Scope, id:String) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64,0f64);
    cx.render(rsx!(
        div { id: "article",
            class:"bg-gray-200 w-screen min-h-[2000px] relative",
            img{ id: "article_image",
                class:"w-full h-[400px] object-cover shadow-inner  shadow-zinc-50 border-b-2 border-black contrast-75 ",
                src:"https://images.unsplash.com/photo-1689613188426-130b2d01c9ca?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=987&q=80",
                alt:""
            }
            div { id: "article_content",
                class:"w-full min-h-[1000px] shadow-t",
                div { id: "article_content_index" ,
                class:"w-full h-[50px] text-center text-2xl font-bold font-mono align-middle p-4 my-12",
                "Title"
                }
                div { id: "article_content_box",
                    class:"w-[70%] h-full p-4 my-10 mx-auto",
                "intro"
                }
                }
        }
    ))
}
