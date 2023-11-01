#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::NAV;

pub fn Navigate(cx: Scope) -> Element {
    let sign = r###"<svg t="1689949917988" class="fill-amber-50 w-6 h-6 absolute right-4 top-4 cursor-pointer" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2413" width="200" height="200"><path d="M886.784 746.496q29.696 30.72 43.52 56.32t-4.608 58.368q-4.096 6.144-11.264 14.848t-14.848 16.896-15.36 14.848-12.8 9.728q-25.6 15.36-60.416 8.192t-62.464-34.816l-43.008-43.008-57.344-57.344-67.584-67.584-73.728-73.728-131.072 131.072q-60.416 60.416-98.304 99.328-38.912 38.912-77.312 48.128t-68.096-17.408l-7.168-7.168-11.264-11.264-11.264-11.264q-6.144-6.144-7.168-8.192-11.264-14.336-13.312-29.184t2.56-29.184 13.824-27.648 20.48-24.576q9.216-8.192 32.768-30.72l55.296-57.344q33.792-32.768 75.264-73.728t86.528-86.016q-49.152-49.152-93.696-93.184t-79.872-78.848-57.856-56.832-27.648-27.136q-26.624-26.624-27.136-52.736t17.92-52.736q8.192-10.24 23.552-24.064t21.504-17.92q30.72-20.48 55.296-17.92t49.152 28.16l31.744 31.744q23.552 23.552 58.368 57.344t78.336 76.288 90.624 88.576q38.912-38.912 76.288-75.776t69.632-69.12 58.368-57.856 43.52-43.008q24.576-23.552 53.248-31.232t55.296 12.8q1.024 1.024 6.656 5.12t11.264 9.216 10.752 9.728 7.168 5.632q27.648 26.624 27.136 57.856t-27.136 57.856q-18.432 18.432-45.568 46.08t-60.416 60.416-70.144 69.632l-77.824 77.824q37.888 36.864 74.24 72.192t67.584 66.048 56.32 56.32 41.472 41.984z" p-id="2414"></path></svg>"###;
    let navigator = use_shared_state::<NAV>(cx).unwrap().read().0.clone();
    let nav_list = navigator.iter().map(|url| {
        rsx!(
            Link { class: "text-center align-middle p-3", to: "{url.1}", "{url.0}" }
        )
    });
    cx.render(
        rsx!(
            nav {
                id: "navigator",
                class: "w-10 h-8 fixed right-0 top-0 md:top-1/2 z-50 flex flex-col justify-evenly cursor-pointer",
                onclick: |e| {
                    gloo_utils::document()
                        .get_element_by_id("navigator_sidebar")
                        .unwrap()
                        .class_list()
                        .remove_1("hidden");
                },
                (1..4).map(|i|{
                    rsx!(
                    div{
                        class:"w-3/4 h-1 border-2 border-gray-800  bg-black rounded-lg mx-auto dark:bg-gray-50 dark:border-gray-50",
                    }
                    )
                }
                ),
                div {
                    id: "navigator_sidebar",
                    class: "hidden fixed right-0 top-0 w-72 bg-gray-900 rounded-l-xl shadow-zinc-700 shadow-2xl h-screen",
                    div {
                        dangerous_inner_html: "{sign}",
                        onclick: |e| {
                            gloo_utils::document()
                                .get_element_by_id("navigator_sidebar")
                                .unwrap()
                                .class_list()
                                .add_1("hidden");
                        }
                    }
                    div { class: "w-3/4 h-1/2 mx-auto translate-y-1/2",
                        ul { class: "text-neutral-200 text-2xl flex flex-col justify-evenly",
                            nav_list
                        }
                    }
                }
            }
        )
    )
}
