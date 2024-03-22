use std::collections::{HashMap, HashSet};
use std::ops::Add;

use dioxus::html::article;
use dioxus::prelude::*;
use futures::future::join_all;
use gloo::console::{console, console_dbg};

use crate::component::loading::Loading;
use crate::model::Article::Article;
use crate::model::ConfigurationTemplate;
use crate::Route;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;
use crate::utils::wordCloud::word_cloud_maker;

#[component]
pub fn ArticleList() -> Element {
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let mut tags_filter = use_signal(||HashSet::<String>::new());
    let mut articles = use_signal(||Vec::<Article>::new());
    let mut tags = 
        use_signal(||HashMap::<String,i32>::new());
    let mut keywords = use_signal(||HashMap::<String,i32>::new());
    let content = use_resource(move || async move{
        let api = configuration().article_api;
        if api.is_empty() {
            articles.set(serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json")).expect("loading failed"));
        }else{
            articles.set(fetch_and_decrypt::<Vec<Article>>(&api).await);
        }
        // todo: why this throw error?
        // join_all(articles.write().iter_mut().map(|a| async {
        //     a.image = parse_to_data_url(a.image.clone(),IMAGE).await;
        // })).await;
        articles.read().iter().for_each(|a|{
            a.tags.iter().for_each(|t|{
                
                if tags().contains_key(t){
                    let count = tags.read().get(t).unwrap().add(1);
                    tags.write().insert(t.clone(), count);
                }else{
                    tags.write().insert(t.clone(),1);
                }
            });
            a.keywords.iter().for_each(|k|{
                if keywords().contains_key(k){
                    let count = keywords.read().get(k).unwrap().add(1);
                    keywords.write().insert(k.clone(),count);
                }else{
                    keywords.write().insert(k.clone(),1);
                }
            });
        });
        "Done"
    });
    let articles_all = articles.read();
    let articles_all =
        articles_all.iter().take(10).map(|a|{
            rsx!{
                li { class: "my-3 underline",
                    Link { to: Route::Article { id: a.id.clone() }, "{a.title}" }
                }
            }
        });
    
    let articles_after_filter  = articles.read();
    let articles_after_filter = 
        articles_after_filter.iter()
        .filter(|a|{
            let mut check_tags = true;
            tags_filter.with(|tags_filter|{
                for t in tags_filter{
                    if !a.tags.contains(&t){
                        check_tags = false;
                    }
                }
            });
            check_tags
        })
        .map(|a|{
            rsx!{
                Link {
                    to: Route::Article { id: a.id.clone() },
                    class: "relative w-full h-44 border border-black/10 rounded-2xl shadow-[0_4px_4px_0_rgba(0,0,0,0.25)]",
                    img {
                        src: "{a.image}",
                        alt: "",
                        class: "w-full h-full rounded-2xl object-cover contrast-50 brightness-50 saturate-50"
                    }
                    span { class: "w-[90%] h-20 flex flex-col absolute top-1/2 left-4 text-white text-ellipsis overflow-hidden",
                        span { class: "text-3xl block", "{a.title}" }
                        span { class: "text-md block text-gray-300 flex-1 ", "{a.introduction}" }
                    }
                }
            }
                                 });
    let tags_all = tags.read();
    let tags_all = tags_all.iter().map(|t|{
        rsx! {
            li {
                class: "m-3 inline-block hover:underline cursor-pointer",
                onclick: move |_| {
                    // if !tags_filter().contains(t.0){
                    //     tags_filter().insert(t.0.clone());
                    // }
                },
                span { class: "text-base whitespace-pre-wrap", "●  {t.0}({t.1})" }
            }
        }
    });
        match &*content.value().read() {
            None => {
                rsx!( Loading {} ) }
            Some(_) => {
                rsx! {
                    div {
                        id: "article_list",
                        class: "w-screen h-screen min-h-[400px] relative",
                        div {
                            id: "article_title",
                            class: "relative top-48 mx-auto w-1/2 text-4xl font-semibold capitalize text-center md:hidden",
                            "article"
                        }
                        div {
                            id: "article_list_box",
                            class: "w-[90%] h-auto mx-auto relative top-48",
                            div {
                                id: "article_list_sidebar",
                                class: "absolute h-auto w-[30%] bg-white right-4 top-12 shadow-[-2px_4px_4px_2px_rgba(0,0,0,0.25)] hidden md:flex md:flex-col",
                                div {
                                    id: "article_list_sidebar_tag",
                                    class: "w-11/12 mx-auto my-10 flex-1",
                                    div {
                                        img {
                                            class: "inline-block w-8 h-8 my-2.5 mr-[2%]",
                                            src: "/static/tag.svg"
                                        }
                                        for t in tags_filter() {
                                            div {
                                                id: "tags_block",
                                                class: "inline-flex h-8 mx-2 my-2 rounded-sm shadow-[0_4px_10px_0_rgba(0,0,0,0.25)] flex-row items-center cursor-pointer",
                                                span { class: "text-sm font-normal align-middle px-2 flex-1",
                                                    "{t}"
                                                }
                                                img {
                                                    class: "w-3.5 h-3.5 flex-1 pr-1 ",
                                                    src: "/static/close_black.svg",
                                                    onclick: |_| {
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ul { class: "w-11/12 h-4/5 p-8", {tags_all} }
                                }
                                div {
                                    id: "article_list_sidebar_key_words",
                                    class: "w-11/12 h-[20vw] mx-auto my-10 flex-1",
                                    img {
                                        class: "inline-block w-8 h-8 my-2 mr-[2%]",
                                        src: "/static/keywords.svg"
                                    }
                                    div {
                                        id: "article_list_keys",
                                        class: "w-11/12 h-[90%]",
                                        onmounted: move |_| {
                                            eval(&word_cloud_maker(&keywords()));
                                        }
                                    }
                                }
                                div {
                                    id: "article_list_sidebar_recommend",
                                    class: "w-11/12 mx-auto my-10 flex-1",
                                    img {
                                        class: "inline-block w-8 h-8 my-2 mr-[2%]",
                                        src: "/static/editor.svg"
                                    }
                                    ul { class: "w-11/12 h-4/5 p-4 px-20", {articles_all} }
                                }
                            }
                            ul {
                                id: "article_list_content",
                                class: "absolute h-[1600px] w-[90%]  md:w-[65%] left-4 top-12 p-5 flex flex-col justify-start gap-5",
                                {articles_after_filter},
                                // todo: implement page feature
                                div {
                                    id: "article_list_table",
                                    class: "relative w-full h-8 my-16 flex rounded-[30px] shadow-[0_-4px_4px_0_rgba(0,0,0,0.25)] justify-center text-lg",
                                    div { class: "mx-2 text-gray-500", "1" }
                                    div { class: "mx-2 text-black", "2" }
                                }
                            }
                        }
                    }
                }
            }
        }
}
