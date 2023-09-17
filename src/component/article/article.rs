#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::model::Article::Article;
use crate::utils::encryptedUtils::{fetch_and_decrypt, fetch_configuration};
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

#[inline_props]
pub fn Article(cx: Scope, id:String) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64,0f64);
    let content = use_future(cx, (id), |id| async move{
        let mut article;
        let api = fetch_configuration().await.articles.api ;
        if api.is_empty() {
            article = serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json")).unwrap().iter().filter(|a|{a.id == id}).last().unwrap().clone();
        }else{
            article = fetch_and_decrypt::<Article>(&(api+"/article/"+&id)).await;
        }
            article.image = parse_to_data_url(article.image.clone(),IMAGE).await;
        article
    });
    cx.render(
        match content.value() { None => {
            rsx!( div { "Nothing Here" } )
        },
            Some(content)=>{
                rsx!(
                    div {
                        id: "article",
                        class: "bg-gray-200 w-screen min-h-[2000px] relative",
                        style { include_str!("css/markdown-theme-light.css") }
                        img {
                            id: "article_image",
                            class: "w-full h-[400px] object-cover shadow-inner  shadow-zinc-50 border-b-2 border-black contrast-75 ",
                            src: "{content.image}",
                            alt: ""
                        }
                        div {
                            id: "article_content",
                            class: "w-full min-h-[1000px] shadow-t",
                            div {
                                id: "article_content_index",
                                class: "w-full h-[50px] text-center text-3xl font-bold font-mono align-middle p-4 my-12",
                                h1 { "{content.title}" }
                            }
                            div {
                                id: "article_content_box",
                                class: "w-[70%] h-full p-4 my-10 mx-auto",
                                // div{
                                //  class:"bg-gray-300 w-[80%] h-100 block mx-auto border-[#2F4858] my-4 p-10 border-l-4",
                                //  "{content.introduction}",
                                // },
                                div {
                                    class: "w-[90%] min-h-200 mx-auto p-4",
                                    id: "article_content_content",
                                    dangerous_inner_html: "{content.content}"
                                }
                            }
                        }
                    }
                )

            }
        }
    )
}
