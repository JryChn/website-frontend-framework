use std::collections::{HashMap, HashSet};
use std::ops::Add;

use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::{use_resource, use_signal};
use dioxus::html::set;
use dioxus::prelude::*;
use futures::future::join_all;

use crate::component::articleList::articles::Articles;
use crate::component::articleList::keywords::Keywords;
use crate::component::articleList::recommend::Recommend;
use crate::component::articleList::tags::Tags;
use crate::model::Article::Article;
use crate::model::ConfigurationTemplate;
use crate::utils::cache::Cache;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

mod articles;
mod keywords;
mod recommend;
mod tags;

const MAX_ARTICLES_ONE_PAGE: usize = 7;

struct ArticlesPage {
    articles_in_page: Vec<Article>,
    is_on_showing: bool,
}

#[component]
pub fn ArticleList() -> Element {
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let tags_filter = use_signal(|| HashSet::<String>::new());
    let mut articles = use_signal(|| Vec::<ArticlesPage>::new());
    let mut tags = use_signal(|| HashMap::<String, i32>::new());
    let mut keywords = use_signal(|| HashMap::<String, i32>::new());
    let content = use_resource(move || async move {
        let all_articles = fetch_articles(configuration().article_list_api).await;
        tags.set(init_tags(&all_articles));
        keywords.set(init_keywords(&all_articles));
        articles.set(construct_article_split_page(all_articles));
    });
    rsx! {
        div {
            id: "article_list",
            class: "w-screen min-h-screen relative dark:bg-gray-950",
            div {
                id: "article_title",
                class: "relative top-48 mx-auto w-1/2 text-4xl font-semibold capitalize text-center md:hidden dark:text-gray-100",
                "article"
            }
            div {
                id: "article_list_box",
                class: "w-[90%] h-auto mx-auto relative top-48",
                match &*content.value().read(){
                    Some(_)=>{
                        rsx!{
                    div {
                        id: "article_list_sidebar",
                        class: "absolute h-auto w-[30%] bg-white right-4 top-12 shadow-[-2px_4px_4px_2px_rgba(0,0,0,0.25)] hidden md:flex md:flex-col md:animate-showFromRight md:delay-1000 dark:bg-[rgb(27,38,59)]",
                        Tags { tags, tags_filter }
                        Keywords { keywords }
                        Recommend { articles }
                    }
                        }
                        
                    },
                    None=>{
                        rsx!{}
                    }
                    },
                Articles { articles, tags_filter }
            }
        }
    }
}

async fn fetch_articles(configuration_api: String) -> Vec<Article> {
    Cache::fetch_or_cache(configuration_api.as_str(),||async{
        let mut all_articles = Vec::<Article>::new();
        if configuration_api.is_empty() {
            all_articles =
                serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json"))
                    .expect("loading failed");
        } else {
            all_articles = fetch_and_decrypt::<Vec<Article>>(&configuration_api)
                .await
                .unwrap();
        }
        join_all(all_articles.iter_mut().map(|a| async {
            a.image = parse_to_data_url(a.image.clone(), IMAGE).await;
        }))
            .await;
        serde_json::to_vec(&all_articles).unwrap()
        
    }).await.unwrap()
}

fn construct_article_split_page(mut articles: Vec<Article>) -> Vec<ArticlesPage> {
    let mut article_vec = Vec::<ArticlesPage>::new();
    while articles.len() > MAX_ARTICLES_ONE_PAGE {
        let (a, b) = articles.split_at(MAX_ARTICLES_ONE_PAGE);
        let new_articles_page = ArticlesPage {
            articles_in_page: Vec::from(a),
            is_on_showing: false,
        };
        article_vec.push(new_articles_page);
        articles = Vec::from(b);
    }
    let last_articles_page = ArticlesPage {
        articles_in_page: articles,
        is_on_showing: false,
    };
    article_vec.push(last_articles_page);
    article_vec.first_mut().unwrap().is_on_showing = true;
    article_vec
}

fn init_tags(articles: &Vec<Article>) -> HashMap<String, i32> {
    let mut tags = HashMap::<String, i32>::new();

    articles.iter().for_each(|a| {
        a.tags.iter().for_each(|t| {
            if tags.contains_key(t) {
                let count = tags.get(t).unwrap().add(1);
                tags.insert(t.clone(), count);
            } else {
                tags.insert(t.clone(), 1);
            }
        })
    });
    tags
}
fn init_keywords(articles: &Vec<Article>) -> HashMap<String, i32> {
    let mut keywords = HashMap::<String, i32>::new();
    articles.iter().for_each(|a| {
        a.keywords.iter().for_each(|k| {
            if keywords.contains_key(k) {
                let count = keywords.get(k).unwrap().add(1);
                keywords.insert(k.clone(), count);
            } else {
                keywords.insert(k.clone(), 1);
            }
        });
    });
    keywords
}
