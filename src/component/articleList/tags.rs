use std::collections::{HashMap, HashSet};

use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use manganis::mg;

#[component]
pub fn Tags(tags: Signal<HashMap<String, i32>>, tags_filter: Signal<HashSet<String>>) -> Element {
    rsx! {
        div {
            id: "article_list_sidebar_tag",
            class: "w-11/12 mx-auto my-10 flex-1",
            div {
                img { class: "inline-block w-8 h-8 my-2.5 mr-[2%] dark:invert", src: mg!(file("src/assets/svg/tag.svg")) }
                for t in tags_filter() {
                    div {
                        id: "tags_block",
                        class: "inline-flex h-8 mx-2 my-2 rounded-sm shadow-[0_4px_10px_0_rgba(0,0,0,0.25)] flex-row items-center cursor-pointer",
                        span { class: "text-sm font-normal align-middle px-2 flex-1",
                            "{t}"
                        }
                        img {
                            class: "w-3.5 h-3.5 flex-1 pr-1 dark:invert",
                            src: mg!(file("src/assets/svg/close_black.svg")),
                            onclick: move |_| {
                                tags_filter.write().remove(&t);
                            }
                        }
                    }
                }
            }
            ul { class: "w-11/12 h-4/5 p-8",
                if tags().is_empty() {
                    for _ in (0..5) {
                        li { class: "w-16 h-5 m-3 inline-block animate-wait" }
                    }
                }
                for t in tags() {
                    li {
                        class: "m-3 inline-block hover:underline cursor-pointer",
                        onclick: move |_| {
                            if !tags_filter().contains(&t.0) {
                                tags_filter.write().insert(t.0.clone());
                            } else {
                                tags_filter.write().remove(&t.0);
                            }
                        },
                        span { class: "text-base whitespace-pre-wrap dark:text-gray-100", "●  {t.0}({t.1})" }
                    }
                }
            }
        }
    }
}
