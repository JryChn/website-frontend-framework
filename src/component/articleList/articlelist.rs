#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::components::Link;
use futures::future::{join, join_all};
use futures::join;
use crate::model::Article::Article;
use crate::utils::encryptedUtils::{fetch_and_decrypt, fetch_configuration};
use crate::utils::netUtils::parse_to_data_url;

#[inline_props]
pub fn ArticleList(cx: Scope) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64, 0f64);
    let content = use_future(cx, (), |_| async {
        let mut articles;
        let api = fetch_configuration().await.articles.api ;
        if api.is_empty() {
            articles = serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json")).unwrap();
        }else{
            articles = fetch_and_decrypt::<Vec<Article>>(&(api+"/fetchArticles")).await;
        }
        join_all(articles.iter_mut().map(|a| async {
            a.image = parse_to_data_url(a.image.clone()).await;
        })).await;
        articles
    });
    cx.render(
        match content.value() {
            None => { rsx!(div{}) }
            Some(article) => {
                rsx!(
            div { id: "article_list",
                class:"bg-gray-200 w-screen min-h-[2000px] relative",
                div { id: "article_list_box",
                    class:"w-[90%] h-[1800px] mx-auto relative top-48",
                 div{ id: "article_list_sidebar",
                     class:"h-[1600px] w-[30%] black bg-gray-100 absolute right-4 top-12 shadow-lg hidden md:block",
                        div{
                            id:"article_list_sidebar_key_words",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "key words:"
                            }
                            div{
                                class:"w-11/12 h-[90%]"
                            }
                        }
                        div{
                            id:"article_list_sidebar_tag",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "tag:"
                            }
                            ul{
                                class:"w-11/12 h-4/5 p-8",
                                li{
                                    class:"m-3 inline-block hover:underline",
                                    "java(10)"
                                }
                            }
                        }
                        div{
                            id:"article_list_sidebar_recommend",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "recommend:"
                            }
                            ul{
                                class:"w-11/12 h-4/5 p-8",
                                li{
                                    class:"my-1 hover:underline",
                                    "Java 的前世今生"
                                }

                            }
                        }
                }
                    ul{
                        id:"article_list_content",
                        class:"h-[1600px] w-[90%]  md:w-[65%] absolute left-4 top-12 p-5 flex flex-col justify-start gap-5",
                                article.iter().map(|a|{
                                    rsx!{
                        Link{
                            to:"/article/{a.id}",
                           class:"h-[200px] w-full border-stone-900 rounded-2xl shadow-xl relative" ,
                            img{
                                src:"{a.image}",
                                alt:"",
                                class:"w-full h-full rounded-2xl object-cover blur-[1px] contrast-75 brightness-90"
                            }
                            span{
                                class:"w-[90%] h-20 flex flex-col absolute top-1/2 left-4 text-gray-50 text-ellipsis overflow-hidden",
                                span{
                                    class:"text-3xl font-sans block",
                                    "{a.title}"
                                }
                                span{
                                    class:"text-md font-mono block text-gray-200 flex-1 ",
                                                   "{a.introduction}"
                                }
                            }
                        }}
                                })
                    }
                }
            }
        )
            }
        }
    )
}
