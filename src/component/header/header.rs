#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;

#[derive(Props, PartialEq)]
pub struct HeaderContext {
    title:String,
    url_jumper: Vec<(String,String)>,
}
pub fn Header(cx: Scope<HeaderContext>) -> Element {
    let header_list = cx.props.url_jumper.iter().map(|url|{
        rsx!(
            Link { class: "hover:border-b-black hover:border-b", to: "{url.1}", "{url.0}" }
        )
    });
    cx.render(
        rsx!(
            header { id: "header", class: "bg-white w-screen h-14 shadow-xl fixed top-0 z-50",
                Link {
                    to:"/",
                    id: "header_title",
                    class: "inline-block absolute top-4 left-3 uppercase font-bold text-xl",
                    "{cx.props.title}"
                }
                div {
                    id: "header_content",
                    class: "inline-block absolute top-4 left-1/4 w-2/3",
                    ul { class: "flex flex-row flex-nowrap justify-around uppercase font-medium",
                        header_list
                    }
                }
            }
        )
    )
}
