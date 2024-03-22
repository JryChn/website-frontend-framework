use std::cell::Ref;
use std::convert::Into;
use std::ops::Deref;
use std::string::ToString;

use dioxus::prelude::*;
use futures::StreamExt;

use crate::component::homepage::icons::Icons;
use crate::component::homepage::single_welcome::WelcomePage;
use crate::component::loading::Loading;
use crate::component::navigation::Navigate;
use crate::model::ConfigurationTemplate;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::MP4;

pub mod single_welcome;
mod icons;

#[component]
pub fn HomePage() -> Element{
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let config = use_resource(move ||async move{
            let mut configuration = configuration();
            configuration.welcome.animation_url.dark = parse_to_data_url(configuration.welcome.animation_url.dark,MP4).await;
            configuration.welcome.animation_url.light = parse_to_data_url(configuration.welcome.animation_url.light,MP4).await;
            configuration
    });
        match &*config.value().read() {
            None => {
                rsx!{ Loading {} }
            },
            Some(_) => {
                rsx! {
                    main { id: "welcome",
                        WelcomePage {}
                        Navigate {}
                        Icons {}
                    }
                }
            }
        }
}
