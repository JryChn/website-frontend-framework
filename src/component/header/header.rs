#![allow(non_snake_case)]

use std::ops::Deref;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{NAV, Route};

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    let title :String= "DJEREMY".into();
    let navigate = use_shared_state::<NAV>(cx).unwrap().read().deref().0.clone();
    let header_list = navigate.iter().map(|url| {
        rsx!(
            Link { class: "hover:border-b-black hover:border-b dark:text-gray-50", to: url.1.clone(), "{url.0}" }
        )
    });
    cx.render(rsx!(
        header { id: "header", class: "bg-white w-screen h-14 shadow-xl fixed top-0 z-50 dark:bg-black",
            Link {
                to: Route::HomePage {},
                id: "header_title",
                class: "inline-block absolute top-4 left-3 uppercase font-bold text-xl dark:text-gray-50",
                "{title}"
            }
            div {
                id: "header_content",
                class: "inline-block absolute top-4 left-1/4 w-2/3",
                ul { class: "flex flex-row flex-nowrap justify-around uppercase font-medium",
                    header_list
                }
            }
        }
        Outlet::<Route>{}
        footer{}
    ))
}
