#![allow(non_snake_case)]

use dioxus::core::IntoDynNode;
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
use crate::component::timer::Timer::Timer;
use crate::model::config::{CommonConfig, ConfigurationTemplate};

mod component;
mod model;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let config = use_future(cx, (), |_| async {
        let config = gloo_net::http::Request::get("/config.json").send().await.expect("Error when load config file").json::<CommonConfig>().await.expect("Error when parse json file");
        if config.configuration_fetching_url.is_empty() {
            config.config_template
        } else {
            gloo_net::http::Request::get(&config.configuration_fetching_url).send().await.expect("Failed fetching config from remote").json::<ConfigurationTemplate>().await.expect("Failed, remote json file parsing error")
        }
    });
    let header_url = vec![
        ("AboutMe".into(), "/aboutMe".into()), ("Calendar".into(), "/calendar".into()),
        ("Articles".into(), "/articles".into()), ("Timer".into(), "/timer".into()), ("Zone".into(), "/zone".into())];
    cx.render(
        match config.value() {
            None => {
                rsx!( div { "Nothing" } )
            }
            Some(configuration) => {
                rsx! {
                    style { include_str!("css/tailwindout.css") }
                    main { id: "djeremy",
                        Router { 
                            Route { to: "/",
                                SingleWelcome {
                                    title: configuration.welcome_page.title.clone(),
                                    animation_video_url: (
    configuration.welcome_page.animation_video_light_url.clone(),
    configuration.welcome_page.animation_video_dark_url.clone(),
),
                                    content: configuration.welcome_page.subtitle_description.clone()
                                }
                                Navigate { url_jumper: header_url.clone() }
                                Articles { articles_intro: vec![(configuration.articles_page.first_article.article_img.clone(), configuration.articles_page.first_article.article_title.clone(),configuration.articles_page.first_article.article_description_index.clone()),
                                    (configuration.articles_page.second_article.article_img.clone(),configuration.articles_page.second_article.article_title.clone(),configuration.articles_page.second_article.article_description_index.clone()),(configuration.articles_page.third_article.article_img.clone(),configuration.articles_page.third_article.article_title.clone(),configuration.articles_page.third_article.article_description_index.clone())] }
                                AboutMe { about_me_video_url: configuration.about_me_page.about_me_url.clone(), about_me_title: configuration.about_me_page.about_me_title.clone(), about_me_description: configuration.about_me_page.about_me_description.clone() }
                                Timer {timer_intro: configuration.timer.timer_intro.clone()}
                                Footer {}
                            }
                            Route { to: "/calender", Calendar {} }
                            Route { to: "/articles",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                ArticleList {}
                            }
                            Route { to: "/aboutMe",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                AboutMeContent {}
                            }
                            Route { to: "/article/*",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                Article {}
                            }
                        }
                    }
                }
            }
        }
    )
}