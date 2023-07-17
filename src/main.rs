#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use dioxus_use_window::use_browser;
use crate::component::aboutMe::AboutMe::AboutMe;
use crate::component::aboutMeContent::aboutmecontent::AboutMeContent;
use crate::component::article::article::Article;
use crate::component::articleList::articlelist::ArticleList;
use crate::component::articles::articles::Articles;
use crate::component::calendar::calendar::Calendar;
use crate::component::footer::footer::Footer;
use crate::component::header::header::Header;
use crate::component::homepage::single_welcome::SingleWelcome;
use crate::component::navigation::navigation::Navigate;
use crate::component::story::Story::Story;

mod component;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

struct DarkMode(bool);

struct ThinMode(bool);

fn App(cx: Scope) -> Element {
    let window_width = use_browser(&cx).width();
    use_shared_state_provider(cx, || DarkMode(false));
    use_shared_state_provider(cx, ||
        if window_width <768{
            ThinMode(true)
        }else{
            ThinMode(false)
        }
    );
    cx.render(rsx! {
        link {
            // this is a common link for css to format style
            href: "https://cdnjs.cloudflare.com/ajax/libs/meyer-reset/2.0/reset.min.css",
            rel: "stylesheet"
        }
        main { id: "djeremy",
        Router{
            Route{
                to:"/",
            SingleWelcome {
                title: "".to_string(),
                animation_video_url: ("".to_string(), "".to_string()),
                content: vec!["".to_string(), "".to_string()],
            }
            Navigate {}
            Articles{}
            AboutMe {}
            Story{}
            Footer{}
            }
                Route{
                    to:"/calender",
                    Header{}
                    Calendar{}
                }
                Route{
                    to:"/articles",
                    Header{}
                    ArticleList{}
                }
                Route{
                    to:"/aboutMe",
                    Header{}
                    AboutMeContent{}
                }
                Route{
                    to:"/article/*",
                    Header{}
                    Article{}
                }
        }
        }
    }
    )
}