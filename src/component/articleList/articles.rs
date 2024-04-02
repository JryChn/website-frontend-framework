use std::collections::HashSet;

use dioxus::prelude::*;

use crate::component::articleList::ArticlesPage;
use crate::model::Article::Article;
use crate::Route;

#[component]
pub fn Articles(
    articles: Signal<Vec<ArticlesPage>>,
    tags_filter: Signal<HashSet<String>>,
) -> Element {
    let articles_after_filter = calculate_filtered_articles(articles, tags_filter);
    rsx! {ul {id: "article_list_content",
        class: "absolute h-[1600px] w-[90%]  md:w-[65%] left-4 top-12 p-5 flex flex-col justify-start gap-5",
        for article in articles_after_filter{
            RenderArticles{article}
        }
        ArticlePages{articles}
    }
    }
}

fn calculate_filtered_articles(
    articles: Signal<Vec<ArticlesPage>>,
    tags_filter: Signal<HashSet<String>>,
) -> Vec<Article> {
    let mut result = Vec::new();
    let articles = articles.read();
    articles
        .iter()
        .filter(|a| a.is_on_showing)
        .flat_map(|t| t.articles_in_page.as_slice())
        .filter(|a| {
            let mut check_tags = true;
            tags_filter.with(|tags_filter| {
                for t in tags_filter {
                    if !a.tags.contains(&t) {
                        check_tags = false;
                    }
                }
            });
            check_tags
        })
        .for_each(|a| result.push(a.clone()));
    result
}

#[component]
fn RenderArticles(article: Article) -> Element {
    rsx! {
        Link {
            to: Route::Article { id: article.id.clone() },
            class: "relative w-full h-44 border border-black/10 rounded-2xl shadow-[0_4px_4px_0_rgba(0,0,0,0.25)]",
            img {
            src: "{article.image}",
            alt: "",
            class: "w-full h-full rounded-2xl object-cover contrast-50 brightness-50 saturate-50"
        }
        span { class: "w-[90%] h-20 flex flex-col absolute top-1/2 left-4 text-white text-ellipsis overflow-hidden",
            span { class: "text-3xl block", "{article.title}" }
        span { class: "text-md block text-gray-300 flex-1 ", "{article.introduction}" }

            }
        }
    }
}

#[component]
fn ArticlePages(articles: Signal<Vec<ArticlesPage>>) -> Element {
    if !articles.is_empty() {
        rsx! {
        div {
            id: "article_list_table",
            class: "relative w-full h-8 my-16 flex rounded-[30px] shadow-[0_-4px_4px_0_rgba(0,0,0,0.25)] justify-center text-lg",
            for (i , mut t) in articles.iter().enumerate() {
            if t.is_on_showing {
            div { class: "mx-2 text-gray-500 select-none cursor-default", "{i+1}" }
            } else {
            div {
            class: "mx-2 text-black cursor-pointer",
            onclick: move |_| {
            let mut articles = articles.write();
            articles.iter_mut().for_each(|t| { t.is_on_showing = false });
            articles.get_mut(i).unwrap().is_on_showing = true;
            },
            "{i+1}"
            }
            }
            }
        }
        }
    } else {
        rsx! {}
    }
}
