use dioxus::core::{Element, Scope};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::model::ConfigurationTemplate;
use crate::Route;

#[component]
pub fn Icons(cx: Scope) -> Element {
    let configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read().clone();
    let github_url = String::from("https://github.com/".to_owned() +configuration.contact.github_username.as_str());
    let telegram_url = String::from("https://t.me/".to_owned() +configuration.contact.telegram_username.as_str());
    let email = String::from("mailto:".to_owned() +configuration.contact.email.as_str());
    cx.render(rsx!(
        div {
            id: "icons",
            class: "fixed bottom-[15vh] right-5 h-[10vw] w-[2vw] flex-col justify-evenly hidden md:flex",
            div { class: "flex justify-center items-center ",
                Link { class: "w-full h-full", to: email, img {
                    class: "w-full h-full p-2 cursor-pointer",
                    src: "/static/email.svg"
                } }
            }
            div { class: "flex justify-center items-center",
                Link { class: "w-full h-full", to: github_url, img {
                    class: "w-full h-full p-2 cursor-pointer",
                    src: "/static/github.svg"
                } }
            }
            div { class: "flex justify-center items-center",
                Link { to: telegram_url, class: "w-full h-full", img {
                    class: "w-full h-full p-2 cursor-pointer",
                    src: "/static/telegram.svg"
                } }
            }
            div { class: "flex justify-center items-center",
                Link { class: "w-full h-full", to: Route::Calendar {}, img {
                    class: "w-full h-full p-2 cursor-pointer",
                    src: "/static/calendar.svg"
                } }
            }
        }
    ))
}
