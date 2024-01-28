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
use crate::utils::encryptedUtils::fetch_configuration;

mod component;
mod model;
mod utils;


struct NAV(Vec<(String,Route)>);
fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let configuration = fetch_configuration();
    let navigator:Vec<(String, Route)>= vec![
        ("Articles".to_string(), Route::ArticleList {}),
        ("Calendar".to_string(), Route::Calendar {}),
        ("Zone".to_string(), Route::HomePage{}),
        ("AboutMe".to_string(), Route::AboutMeContent{}),
    ];
    use_shared_state_provider(cx,||{
        configuration
    });
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
    #[end_layout]
    // todo: may fix page not found error later
    #[route("/:route")]
    PageNotFound {
        route: String
    }
}
