use std::collections::HashMap;

use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use manganis::mg;

use crate::utils::wordCloud::word_cloud_maker;

#[component]
pub fn Keywords(keywords: Signal<HashMap<String, i32>>) -> Element {
    let keywords = keywords();
    eval(&word_cloud_maker(&keywords));
    rsx! {
        div {
            id: "article_list_sidebar_key_words",
            class: "w-11/12 h-[20vw] mx-auto my-10 flex-1",
            img { class: "inline-block w-8 h-8 my-2 mr-[2%]", src: mg!(file("src/assets/svg/keywords.svg")) }
            div { id: "article_list_keys", class: "w-11/12 h-[90%]" }
        }
    }
}
