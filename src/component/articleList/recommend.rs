use dioxus::prelude::*;
use dioxus::signals::Signal;

use crate::component::articleList::ArticlesPage;
use crate::Route;

#[component]
pub fn Recommend(articles: Signal<Vec<ArticlesPage>>) -> Element {
    let articles = articles.read();
    let articles_all = articles
        .iter()
        .flat_map(|t| t.articles_in_page.as_slice())
        .take(10)
        .map(|a| {
            rsx! {
                li { class: "my-3 underline",
                    Link { to: Route::Article { id: a.id.clone() }, "{a.title}" }
                }
            }
        });
    rsx! {
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
}
