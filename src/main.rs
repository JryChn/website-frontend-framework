#![allow(non_snake_case)]

use dioxus::html::{div, link, thead};
use dioxus::html::input_data::keyboard_types::Key::Link;
use dioxus::prelude::*;

use component::welcome::WelcomeContent;
use component::aboutme::AboutMe;

mod component;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}


fn App(cx: Scope) -> Element {
    cx.render(rsx! {
            link {
                href: "https://cdnjs.cloudflare.com/ajax/libs/meyer-reset/2.0/reset.min.css",
                rel:"stylesheet"
            },
        main { width: "100%",  background_color: "#f1f1f1",
            min_width:"200px",
            WelcomeContent{},
            // AboutMe{}
        }
    }
    )
}