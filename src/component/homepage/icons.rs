use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use manganis::mg;

use crate::model::Contact;
use crate::Route;

#[component]
pub fn Icons(contact: Contact) -> Element {
    let github_url =
        String::from("https://github.com/".to_owned() + contact.github_username.as_str());
    let telegram_url =
        String::from("https://t.me/".to_owned() + contact.telegram_username.as_str());
    let email = String::from("mailto:".to_owned() + contact.email.as_str());
    rsx!(
        div {
            id: "icons",
            class: "fixed bottom-[15vh] right-5 h-[10vw] w-9 flex-col justify-evenly hidden md:flex",
            div { class: "flex justify-center items-center ",
                Link { class: "w-full h-full", to: email, img { class: "w-full h-full p-2 cursor-pointer", src: mg!(file("src/assets/svg/email.svg")) } }
            }
            div { class: "flex justify-center items-center",
                Link { class: "w-full h-full", to: github_url, img { class: "w-full h-full p-2 cursor-pointer", src: mg!(file("src/assets/svg/github.svg")) } }
            }
            div { class: "flex justify-center items-center",
                Link { to: telegram_url, class: "w-full h-full", img { class: "w-full h-full p-2 cursor-pointer", src: mg!(file("src/assets/svg/telegram.svg")) } }
            }
            div { class: "flex justify-center items-center",
                Link { class: "w-full h-full", to: Route::Calendar {}, img { class: "w-full h-full p-2 cursor-pointer", src: mg!(file("src/assets/svg/calendar.svg")) } }
            }
        }
    )
}
