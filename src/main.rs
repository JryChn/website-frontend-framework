#![allow(non_snake_case)]

use charming::element::Padding::Single;
use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Router};
use log::LevelFilter;
use manganis::mg;

use crate::component::aboutMeContent::aboutmecontent::AboutMeContent;
use crate::component::article::article::Article;
use crate::component::articleList::articlelist::ArticleList;
use crate::component::calendar::calendar::Calendar;
use crate::component::header::header::Header;
use crate::component::homepage::HomePage;
use crate::component::pageNotFound::pageNotFound::PageNotFound;
use crate::utils::encryptedUtils::fetch_configuration;

mod component;
mod model;
mod utils;

const _TAILWIND_CSS :&str = mg!(file("public/tailwind.css"));
static NAVIGATOR: GlobalSignal<Vec<(String,Route)>> = Signal::global(||
    vec![
        ("Articles".to_string(), Route::ArticleList {}),
        ("Calendar".to_string(), Route::Calendar {}),
        ("Zone".to_string(), Route::HomePage{}),
        ("AboutMe".to_string(), Route::AboutMeContent{}),
    ]
);
fn main() {
    // launch the web app
    launch(App);
}

fn App() -> Element {
    // Init debug
    let configuration = Signal::new(fetch_configuration());
    use_context_provider(||{
        configuration
    });
    rsx!( Router::<Route> {} )
}
#[derive(Routable,Clone,PartialEq)]
enum Route {
    #[route("/")]
    HomePage{},
    #[layout(Header)]
    #[route("/calendar")]
    Calendar{},
    #[nest("/articles")]
    #[route("/")]
    ArticleList{},
    #[route("/:id")]
    Article{id:String},
    #[end_nest]
    #[nest("/article")]
    #[redirect("/",|| Route::ArticleList{})]
    #[redirect("/:id",|id:String| Route::Article{id})]
    #[end_nest]
    #[route("/aboutMe")]
    AboutMeContent {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>
    }
}
