use dioxus::prelude::*;

use crate::{NAVIGATOR, Route};
use crate::component::navigation::Navigate;
use crate::model::ConfigurationTemplate;

#[component]
pub fn Header() -> Element {
    let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    let title = &configuration().welcome.subtitle;
    let navigate = &*NAVIGATOR.read();
    let current_route = use_route::<Route>();
    let underline_css = |r1: Route| {
        if r1.eq(&current_route) {
            " border-b border-b-black "
        } else {
            ""
        }
    };
    let header_list = navigate.iter().map(|url| {
        rsx!(
            Link {
                class: "text-lg capitalize font-normal hover:border-b-black hover:border-b {underline_css(url.1.clone())}",
                to: url.1.clone(),
                "{url.0}"
            }
        )
    });
    rsx! {
        header {
            id: "header",
            class: "bg-transparent w-screen h-14 fixed top-0 shadow-none z-50 md:bg-white md:shadow-[0_4px_20px_0_rgba(0,0,0,0.25)] dark:md:bg-black",
            Link {
                to: Route::HomePage {},
                class: "absolute w-auto h-11 top-1/2 -translate-y-1/2 flex uppercase font-bold text-lg items-center text-center p-3 dark:text-white",
                id: "header_title",
                "{title}"
            }
            div {
                id: "header_content",
                class: "absolute top-4 left-1/3 w-1/2 hidden md:inline-block",
                ul { class: "flex flex-row flex-nowrap justify-around uppercase font-medium dark:text-white",
                    {header_list}
                }
            }
            // make hamburger when mobile size
            div { class: "block z-60 md:hidden", Navigate {} }
        }
        Outlet::<Route> {}
    }
}
