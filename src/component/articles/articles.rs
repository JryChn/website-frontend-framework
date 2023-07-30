#![allow(non_snake_case)]

use std::ops::{Add, Deref};
use dioxus::html::span;
use dioxus::prelude::*;
use dioxus_router::{Link, Router};
use gloo_file::ObjectUrl;
use web_sys::Blob;

#[derive(Props, PartialEq)]
pub struct ArticlesContext {
    articles_intro:Vec<(String,String,String)>
}

pub fn Articles(cx: Scope<ArticlesContext>) -> Element {
    let article_content = cx.props.articles_intro.iter().map(|a|{
        rsx!(
        div { class: "w-[333px] h-[666px]",
            div { class: "border-black border-2 w-[90%] h-[60%] rounded-xl shadow-zinc-600 shadow-lg mx-5 my-5",
                img { id:"test",src: "{a.0}", alt: "", class: "w-full h-full rounded-xl" }
            }
            div { class: "w-[90%] h-[30%] mx-auto overflow-hidden overflow-ellipsis",
                h2 { class: "font-medium text-2xl font-serif dark:text-gray-50", "{a.1}" }
                span { class: "my-4 font-sans text-gray-500 dark:text-gray-300", "{a.2}" }
            }
        }
    )
    });
    cx.render(
        rsx!(
            div {
                id: "article",
                class: "w-screen min-h-[1000px] bg-gray-200 dark:bg-black",
                div { id: "article_title_box", class: "w-3/4 h-1/6 mx-auto",
                    h2 {
                        id: "article_title",
                        class: "text-5xl font-mono font-bold text-center p-10 dark:text-gray-50",
                        "This is Article"
                    }
                }
                div {
                    id: "article_list",
                    class: " w-3/4 mx-auto flex flex-wrap shrink-0 justify-around",
                    article_content,
                    Link {
                        to: "/articles",
                        class: "block w-3/4 h-12 mx-auto text-center align-middle text-xl underline dark:text-gray-50",
                        "Read More"
                    }
                }
            }
        )
    )
}