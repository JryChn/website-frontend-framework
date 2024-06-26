#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Router};
use gloo::console::console_dbg;
use gloo::utils::{document, document_element, window};
use manganis::mg;
use web_sys::MediaQueryList;

use crate::component::aboutMeContent::AboutMeContent;
use crate::component::article::article::Article;
use crate::component::articleList::ArticleList;
use crate::component::calendar::Calendar;
use crate::component::header::header::Header;
use crate::component::homepage::HomePage;
use crate::component::loading::Loading;
use crate::component::pageNotFound::pageNotFound::PageNotFound;
use crate::utils::cache::Cache;
use crate::utils::encryptedUtils::fetch_configuration;

mod component;
mod model;
mod utils;

const _TAILWIND_CSS: &str = mg!(file("src/css/tailwind_output.css"));
static NAVIGATOR: GlobalSignal<Vec<(String, Route)>> = Signal::global(|| {
    vec![
        ("Articles".to_string(), Route::ArticleList {}),
        ("Calendar".to_string(), Route::Calendar {}),
        ("Zone".to_string(), Route::HomePage {}),
        ("AboutMe".to_string(), Route::AboutMeContent {}),
    ]
});
fn main() {
    // launch the web app
    launch(App);
}

fn App() -> Element {
    enable_dark_mode();

    // Init CACHE
    let config = use_resource(|| fetch_configuration());
    match &*config.value().read() {
        None => {
            rsx! { Loading {} }
        }
        Some(config) => {
            let configuration = Signal::new(config.clone());
            use_context_provider(|| configuration);
            rsx!( Router::<Route> {} )
        }
    }
}

fn enable_dark_mode() {
    // enable dark mode for system preference
    match window().match_media("(prefers-color-scheme: dark)").unwrap() {
        None => {
            document_element().class_list().remove_1("dark").expect("disable dark mode failed");
        }
        Some(m) => {
           if m.matches(){
               document_element().class_list().add_1("dark").expect("enable dark mode failed");
           }
        }
    };

}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    HomePage {},
    #[layout(Header)]
    #[route("/calendar")]
    Calendar {},
    #[nest("/articles")]
    #[route("/")]
    ArticleList {},
    #[route("/:id")]
    Article { id: String },
    #[end_nest]
    #[nest("/article")]
    #[redirect("/",|| Route::ArticleList{})]
    #[redirect("/:id",|id:String| Route::Article{id})]
    #[end_nest]
    #[route("/aboutMe")]
    AboutMeContent {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
