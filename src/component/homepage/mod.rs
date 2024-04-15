use dioxus::prelude::*;

use crate::component::homepage::icons::Icons;
use crate::component::homepage::single_welcome::WelcomePage;
use crate::component::navigation::Navigate;
use crate::model::ConfigurationTemplate;

mod icons;
pub mod single_welcome;

#[component]
pub fn HomePage() -> Element {
    let config = consume_context::<Signal<ConfigurationTemplate>>();
            let welcome = config().welcome.to_owned();
            let contact = config().contact.to_owned();
            rsx! {
                    main { id: "welcome", class: "overflow-hidden",
                        WelcomePage { welcome }
                        Navigate {}
                        Icons { contact }
                }
            }
}
