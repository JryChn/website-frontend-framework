use std::convert::Into;
use std::ops::Deref;
use std::string::ToString;

use dioxus::core::{Element, Scope};
use dioxus::prelude::*;
use futures::StreamExt;

use crate::component::homepage::navigation::Navigate;
use crate::component::homepage::single_welcome::SingleWelcome;
use crate::model::config::ConfigurationTemplate;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::MP4;

pub mod single_welcome;
pub mod navigation;
pub mod articles;
pub mod about_me;

pub fn HomePage(cx:Scope) -> Element{
    let mut configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read().clone();
    let config = use_future(cx, (), |_| async {
        configuration.welcome.animation_url.dark = parse_to_data_url(configuration.welcome.animation_url.dark,MP4).await;
        configuration.welcome.animation_url.light = parse_to_data_url(configuration.welcome.animation_url.light,MP4).await;
        // configuration.about_me.video = parse_to_data_url(configuration.about_me.video,MP4).await;
        // join_all(
        //     configuration.articles.article.iter_mut().map(|a| async {
        //         a.image = parse_to_data_url(a.image.clone(),IMAGE).await;
        //     })).await;
        // join_all(
        //     configuration.timer.posts.iter_mut().map(|a| async {
        //         a.image = parse_to_data_url(a.image.clone(),IMAGE).await;
        //     })).await;
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
                        SingleWelcome{},
                        Navigate{},
                        // Articles{articles:configuration.articles.article.clone()},
                        // AboutMe{ name:configuration.about_me.name.clone(),description:configuration.about_me.description.clone(),video:configuration.about_me.video.clone()},
                        // Timer{posts:configuration.timer.posts.clone()},
                    }
                }
            }
        }
    ))
}