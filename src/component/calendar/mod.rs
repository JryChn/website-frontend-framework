use dioxus::prelude::*;
use manganis::mg;

use crate::component::calendar::calendar::CalendarContent;
use crate::component::loading::Loading;
use crate::model::Calendar::Calendar;
use crate::model::ConfigurationTemplate;
use crate::utils::cache::Cache;
use crate::utils::encryptedUtils::fetch_and_decrypt;

mod calendar;
mod table;
mod timeTable;

#[component]
pub fn Calendar() -> Element {
    let started = use_signal(|| false);
    let configuration = use_context::<Signal<ConfigurationTemplate>>();
    let result = use_resource(move || async move {
        let api = configuration().calendar_api;

        let calendar: Calendar = Cache::fetch_or_cache(api.as_str(), || async {
            let mut calendar;
            if api.is_empty() {
                calendar = serde_json::from_str::<Calendar>(include_str!(
                    "../../defaultConfig/calendar.json"
                ))
                .unwrap();
            } else {
                calendar = fetch_and_decrypt(api.as_str()).await.unwrap();
            }
            serde_json::to_vec(&calendar).unwrap()
        })
        .await
        .unwrap();
        calendar
    });
    match &*result.value().read() {
        None => {
            rsx! { Loading {} }
        }
        Some(result) => {
            rsx! {
                if started() {
                    CalendarContent {
                        year: result.year,
                        days: result
                            .events
                            .iter()
                            .map(|e| {
                                (
                                    e.day.clone(),
                                    e.duration.iter().map(|d| { (d.start_number, d.end_number) }).collect(),
                                )
                            })
                            .collect()
                    }
                } else {
                    WelcomeCalendar { started }
                }
            }
        }
    }
}

#[component]
fn WelcomeCalendar(started: Signal<bool>) -> Element {
    rsx! {
        div {
            id: "calendar",
            class: " w-screen min-h-screen select-none cursor-default dark:bg-gray-950",
            div {
                id: "calendar_box",
                class: "w-3/4 mx-auto translate-y-1/2 flex flex-col justify-evenly",
                CalendarSVG {}
                CalendarMono {}
                div { class: "text-3xl font-light text-center w-full mx-auto my-5 md:w-2/3 dark:text-white",
                    "你可以在这里查看我的行程，看看某一天我有时间可以请我吃饭，甚至可以book我的时间一起活动，当然，如果你没有恶意的话，我欣然赴约"
                }
                div {
                    class: "w-auto h-auto bg-gradient-to-b from-purple-950 to-pink-900 from-30% rounded-2xl mx-auto shadow-[inset_0_4px_1px_0_rgba(0,0,0,0.25),0_7px_8px_0_rgba(0,0,0,0.25)] flex items-center justify-center p-2 my-10 cursor-pointer hover:shadow-zinc-800",
                    onclick: move |_| { started.set(!started()) },
                    span { class: "text-3xl font-medium text-[rgb(82,124,89)] dark:text-gray-100", "查看我的行程" }
                }
            }
        }
    }
}

#[component]
fn CalendarSVG() -> Element {
    rsx! {img { class: "w-12 h-12 mx-auto dark:invert", src: mg!(file("src/assets/svg/calendar_2.svg")) }}
}
#[component]
fn CalendarMono() -> Element {
    rsx! {
        div { class: "h-12 mx-auto text-3xl font-medium my-5 whitespace-nowrap dark:text-white",
            span { "Never Miss " }
            span { class: "bg-gradient-to-r from-pink-600 to-green-600 bg-clip-text text-transparent mx-1",
                "One"
            }
            span { " Thing" }
        }
    }
}
