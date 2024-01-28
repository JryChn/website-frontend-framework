use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{NAV, Route};
use crate::model::ConfigurationTemplate;

#[component]
pub fn Header(cx: Scope) -> Element {
    // todo: let the nav who be selected become bolder or underline
    let configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read();
    let title = &configuration.welcome.subtitle;
    let navigate = use_shared_state::<NAV>(cx).unwrap().read().deref().0.clone();
    let header_list = navigate.iter().map(|url| {
        rsx!(
            Link {
                class: "text-lg capitalize font-normal hover:border-b-black hover:border-b",
                to: url.1.clone(),
                "{url.0}"
            }
        )
    });
    cx.render(rsx!(
        header {
            id: "header",
            class: "bg-white w-screen h-14 shadow-[0_4px_20px_0_rgba(0,0,0,0.25)] fixed top-0 z-50",
            Link {
                to: Route::HomePage {},
                class: "absolute w-auto h-11 top-1/2 -translate-y-1/2 flex uppercase font-bold text-lg items-center text-center p-3",
                id: "header_title",
                "{title}"
            }
            // todo: make hamburger when mobile size
            div {
                id: "header_content",
                class: "inline-block absolute top-4 left-1/3 w-1/2",
                ul { class: "flex flex-row flex-nowrap justify-around uppercase font-medium",
                    header_list
                }
            }
        }
        Outlet::<Route> {}
    ))
}
