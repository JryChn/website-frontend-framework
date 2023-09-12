#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::component::aboutMeContent::aboutmecontent::AboutMeContent;
use crate::component::article::article::Article;
use crate::component::articleList::articlelist::ArticleList;
use crate::component::calendar::calendar::Calendar;
use crate::component::header::header::Header;
use crate::component::homepage::HomePage;
use crate::component::pageNotFound::pageNotFound::PageNotFound;

mod component;
mod model;
mod utils;


struct NAV(Vec<(String,Route)>);
fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// todo: add keyword word-cloud
// todo: make article more readable
// todo: add github and radar at about me page
fn App(cx: Scope) -> Element {
    let navigator:Vec<(String, Route)>= vec![
        ("AboutMe".to_string(), Route::AboutMeContent {}),
        ("Calendar".to_string(), Route::Calendar {}),
        ("Articles".to_string(), Route::ArticleList{}),
        ("Timer".to_string(), Route::HomePage{}),
        ("Zone".to_string(), Route::HomePage{}),
    ];
    use_shared_state_provider(cx,||NAV(navigator));
    render!( Router::<Route> {} )
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
    // #[route("/timer")]
    // Timer{},
    // #[route("/zone")]
    // Zone{},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    }
}
