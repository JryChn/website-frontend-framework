use dioxus::prelude::*;

use crate::component::homepage::icons::Icons;
use crate::component::homepage::single_welcome::WelcomePage;
use crate::component::loading::Loading;
use crate::component::navigation::Navigate;
use crate::model::ConfigurationTemplate;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::MP4;

mod icons;
pub mod single_welcome;

#[component]
pub fn HomePage() -> Element {
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let config = use_resource(move || async move {
        let mut configuration = configuration();
        configuration.welcome.animation_url.dark =
            parse_to_data_url(configuration.welcome.animation_url.dark, MP4).await;
        configuration.welcome.animation_url.light =
            parse_to_data_url(configuration.welcome.animation_url.light, MP4).await;
        configuration
    });
    match &*config.value().read() {
        None => {
            rsx! { Loading {} }
        }
        Some(config) => {
            let welcome = config.welcome.to_owned();
            let contact = config.contact.to_owned();
            rsx! {
                    main { id: "welcome", class: "overflow-hidden",
                        WelcomePage { welcome }
                        Navigate {}
                        Icons { contact }
                }
            }
        }
    }
}
