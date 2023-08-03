#![allow(non_snake_case)]

use dioxus::core::IntoDynNode;
use dioxus::html::div;
use dioxus::prelude::*;
use std::process::id;
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
use crate::utils::encryptedUtils::{fetch_and_decrypt};
use crate::utils::netUtils::parse_to_data_url;

mod component;
mod model;
mod utils;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// todo: fetching img/video resourceat first to optimize performance
// todo: using AES and base64 to encrypt/encode the request/response
fn App(cx: Scope) -> Element {
    let config = use_future(cx, (), |_| async {
        let config: CommonConfig = serde_json::from_str(include_str!("config.json")).expect("Error when loading config file");
        let mut configuration;
        if config.configuration_fetching_url.is_empty() {
            configuration = config.config_template
        } else {
            configuration =  fetch_and_decrypt::<ConfigurationTemplate>(config.configuration_fetching_url,config.encrypted_key).await;
        }
        configuration.articles_page.first_article.article_image = parse_to_data_url(configuration.articles_page.first_article.article_image).await;
        configuration.articles_page.second_article.article_image = parse_to_data_url(configuration.articles_page.second_article.article_image).await;
        configuration.articles_page.third_article.article_image = parse_to_data_url(configuration.articles_page.third_article.article_image).await;
        configuration.welcome_page.animation_video_light_url = parse_to_data_url(configuration.welcome_page.animation_video_light_url).await;
        configuration.welcome_page.animation_video_dark_url = parse_to_data_url(configuration.welcome_page.animation_video_dark_url).await;
        configuration.about_me_page.about_me_video_url = parse_to_data_url(configuration.about_me_page.about_me_video_url).await;
        configuration
    });
    let header_url = vec![
        ("AboutMe".into(), "/aboutMe".into()),
        ("Calendar".into(), "/calendar".into()),
        ("Articles".into(), "/articles".into()),
        ("Timer".into(), "/timer".into()),
        ("Zone".into(), "/zone".into()),
    ];
    cx.render(
        match config.value() {
            None => {
                rsx!( div { class:"w-screen h-screen relative",
                    div{ class:"absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold",
                     "Loading...."
                    }
                    } )
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
                                Articles { articles_intro: vec![configuration.articles_page.first_article.clone(),configuration.articles_page.second_article.clone(),configuration.articles_page.third_article.clone()] }
                                AboutMe { about_me_video_url: configuration.about_me_page.about_me_video_url.clone(), about_me_title: configuration.about_me_page.about_me_title.clone(), about_me_description: configuration.about_me_page.about_me_description.clone() }
                                Timer {timer_intro: configuration.timer.timer_intro.clone()}
                                Footer {}
                            }
                            Route { to: "/calender",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                Calendar {
                                url: configuration.calendar_url.clone()
                            } }
                            Route { to: "/articles",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                ArticleList {url:configuration.articles_page.all_article_url.clone()}
                            }
                            Route { to: "/aboutMe",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                AboutMeContent {url:configuration.about_me_page.about_me_intro_url.clone()}
                            }
                            Route { to: "/article/:id",
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                Article {
                                    url:configuration.articles_page.one_article_url.clone(),
                                }
                            }
                            Route { to: "/timer"
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                }
                            Route { to: "/zone"
                                Header { title: "DJEREMY".into(), url_jumper: header_url.clone() }
                                }
                        }
                    }
                }
            }
        }
    )
}

