#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
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
fn App(cx: Scope) -> Element {
        cx.render(rsx! {
            style { include_str!("css/tailwindout.css") }
            main { id: "djeremy",
                Router { 
                    Route { to: "/",
                        SingleWelcome {
                            title: "djeremychen.com".into(),
                            animation_video_url: ("day_tul".into(), "/test.mp4".into()),
                            content: vec!["俊俏的冷枪手".into(), "无敌风火轮".into()]
                        }
                        Navigate {
                            url_jumper: vec![
    ("AboutMe".into(), "/aboutMe".into()), ("Calendar".into(), "/calendar".into()),
    ("Articles".into(), "/articles".into()), ("Timer".into(), "/timer".into()), ("Zone"
    .into(), "/zone".into())
]
                        }
                        Articles { article_list_url: "".into() }
                        AboutMe { about_me_video_url: "".into(), about_me_title: "".into(), about_me_description: "".into() }
                        Timer {}
                        Story {}
                        Footer {}
                    }
                    Route { to: "/calender",
                        Header {}
                        Calendar {}
                    }
                    Route { to: "/articles",
                        Header {}
                        ArticleList {}
                    }
                    Route { to: "/aboutMe",
                        Header {}
                        AboutMeContent {}
                    }
                    Route { to: "/article/*",
                        Header {}
                        Article {}
                    }
                }
            }
        }
    )
}