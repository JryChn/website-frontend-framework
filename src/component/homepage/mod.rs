use std::convert::Into;
use std::ops::Deref;
use std::string::ToString;

use dioxus::core::{Element, Scope};
use dioxus::prelude::*;
use futures::future::join_all;
use futures::StreamExt;

use crate::component::homepage::about_me::AboutMe;
use crate::component::homepage::articles::Articles;
use crate::component::homepage::navigation::Navigate;
use crate::component::homepage::single_welcome::SingleWelcome;
use crate::component::timer::Timer::Timer;
use crate::Route::Article;
use crate::utils::encryptedUtils::fetch_configuration;
use crate::utils::netUtils::parse_to_data_url;

pub mod single_welcome;
pub mod navigation;
pub mod articles;
pub mod about_me;

pub fn HomePage(cx:Scope) -> Element{
    let config = use_future(cx, (), |_| async {
        let mut configuration = fetch_configuration().await;
        configuration.welcome.animation_url.dark = parse_to_data_url(configuration.welcome.animation_url.dark).await;
        configuration.welcome.animation_url.light = parse_to_data_url(configuration.welcome.animation_url.light).await;
        configuration.about_me.video = parse_to_data_url(configuration.about_me.video).await;
        join_all(
            configuration.articles.article.iter_mut().map(|a| async {
                a.image = parse_to_data_url(a.image.clone()).await;
            })).await;
        join_all(
            configuration.timer.posts.iter_mut().map(|a| async {
                a.image = parse_to_data_url(a.image.clone()).await;
            })).await;
        configuration
    });
    cx.render(rsx!(
        match config.value() {
            None => {
        rsx!( div { class:"w-screen h-screen relative",
                    div{ class:"absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold",
                     "Loading...."
                    }
                    })
            },
            Some(configuration) => {
                rsx! {
                    main { id: "djeremy",
                        SingleWelcome{welcome:configuration.welcome.clone()},
                        Navigate{},
                        Articles{articles:configuration.articles.article.clone()},
                        AboutMe{ name:configuration.about_me.name.clone(),description:configuration.about_me.description.clone(),video:configuration.about_me.video.clone()},
                        Timer{posts:configuration.timer.posts.clone()},
                    }
                }
            }
        }
    ))
}