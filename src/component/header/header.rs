use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{NAV, Route};
use crate::component::navigation::Navigate;
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
    render!(
        header {
            id: "header",
            class: "bg-transparent w-screen h-14 fixed top-0 shadow-none z-50 md:bg-white md:shadow-[0_4px_20px_0_rgba(0,0,0,0.25)] ",
            Link {
                to: Route::HomePage {},
                class: "absolute w-auto h-11 top-1/2 -translate-y-1/2 flex uppercase font-bold text-lg items-center text-center p-3",
                id: "header_title",
                "{title}"
            }
            div {
                id: "header_content",
                class: "absolute top-4 left-1/3 w-1/2 hidden md:inline-block",
                ul {
                    class: "flex flex-row flex-nowrap justify-around uppercase font-medium",
                    header_list: header_list
                }
            }
            // make hamburger when mobile size
            div { class: "block z-60 md:hidden", Navigate {} }
        }
        Outlet::<Route> {}
    )
}
