use dioxus::prelude::*;
use dioxus_router::prelude::GoBackButton;
use gloo::console::console_dbg;
use manganis::mg;
use web_sys::ScrollBehavior;
use web_sys::ScrollToOptions;

use crate::model::Article::Article;
use crate::model::ConfigurationTemplate;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

#[component]
pub fn Article(id: String, article: Option<Article>) -> Element {
    //todo: make article can click from outside
    if article.is_some() {
        let content = article.unwrap();
        return rsx! { RenderArticle { content } };
    }
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let id = use_signal(|| id);
    let content = use_resource(move || async move {
        let mut article;
        let api = configuration().article_api;
        if api.is_empty() {
            article = serde_json::from_str::<Vec<Article>>(include_str!(
                "../../defaultConfig/article.json"
            ))
            .unwrap()
            .iter()
            .filter(|a| a.id == id())
            .last()
            .unwrap_or(&DefaultArticle())
            .clone();
        } else {
            let api_with_id = api + "/" + id().as_str() + ".json";
            article = fetch_and_decrypt::<Article>(&api_with_id)
                .await
                .unwrap_or(DefaultArticle());
        }
        article.image = parse_to_data_url(article.image.clone(), IMAGE).await;
        article
    });
    let enable_align_top_button = use_signal(|| false);
    match &*content.value().read() {
        None => {
            rsx!( div { "Nothing Here" } )
        }
        Some(content) => {
            rsx! { RenderArticle { content: content.clone() } }
        }
    }
}

#[component]
fn RenderArticle(content: Article) -> Element {
    console_dbg!(gloo::utils::document_element().class_name());
    let css_themes: &str = if gloo::utils::document_element().class_name().contains("dark")
    {
        include_str!("css/markdown-theme-dark.css")
    }else{
        include_str!("css/markdown-theme-light.css")
    };
    rsx! {
        div {
            id: "article",
            class: "w-screen min-h-[800px] relative scroll-smooth",
            style { "{css_themes}" }
            img {
                id: "article_image",
                class: "w-full h-72 object-cover shadow-[inset_9px_4px_14px_6px_rgba(0,0,0,0.25),0_4px_4px_0_rgba(0,0,0,0.25)] contrast-75",
                src: "{content.image}",
                alt: ""
            }
            div {
                id: "go_back_button",
                class: "absolute my-20 mx-8 font-light text-lg text-center align-middle hidden md:block",
                GoBackButton { 
                    img { class: "inline-block dark:invert", src: mg!(file("src/assets/svg/go_back.svg")) }
                    "Back to list"
                }
            }
            //todo: show when scroll to proper location later
            div {
                id: "align_top_button",
                class: "fixed right-8 bottom-5 cursor-pointer dark:invert",
                onclick: |_e| {
                    gloo::utils::window()
                        .scroll_to_with_scroll_to_options(
                            ScrollToOptions::new().behavior(ScrollBehavior::Smooth).top(0f64),
                        );
                },
                img { src: mg!(file("src/assets/svg/align_top.svg")) }
            }
            div { id: "article_content", class: "w-full min-h-screen shadow-t",
                div {
                    id: "article_content_index",
                    class: "w-full h-20 text-center text-4xl font-semibold align-middle p-4 my-8",
                    span { "{content.title}" }
                }
                div {
                    id: "article_content_box",
                    class: "w-full h-full p-4 my-10 mx-auto",
                    // div{
                    //  class:"bg-gray-300 w-[80%] h-100 block mx-auto border-[#2F4858] my-4 p-10 border-l-4",
                    //  "{content.introduction}",
                    // },
                    div {
                        class: "w-[95%] min-h-200 mx-auto p-4",
                        id: "article_content_content",
                        dangerous_inner_html: "{content.content}"
                    }
                }
            }
        }
    }
}

fn DefaultArticle() -> Article {
    Article {
        id: "".to_string(),
        image: "".to_string(),
        title: "Wrong Happened!!! Nothing Here".to_string(),
        introduction: "".to_string(),
        tags: vec![],
        keywords: vec![],
        post_time: "".to_string(),
        content: "".to_string(),
    }
}
