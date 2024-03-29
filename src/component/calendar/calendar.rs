use dioxus::prelude::*;

use crate::component::calendar::{CalendarMono, CalendarSVG};
use crate::component::calendar::table::Table;
use crate::component::calendar::timeTable::TimeTable;
use crate::model::AboutMe::AboutMePage;
use crate::model::ConfigurationTemplate;
use crate::utils::encryptedUtils::fetch_and_decrypt;

#[component]
pub fn CalendarContent(year:u32,days:Vec<(String,Vec<(u32,u32)>)>) -> Element {
    rsx!{
        div { class: "w-screen min-h[800px] select-none cursor-default",
            div { class: "relative hidden mt-28 mb-10 left-10 w-[400px] h-20 flex-row items-center justify-start md:flex",
                CalendarSVG {}
                CalendarMono {}
            }
            div { class: "flex w-[90%] mx-auto flex-col translate-y-44 md:translate-y-0",
                div { class: "h-11 my-5 flex justify-end",
                    input {
                        class: "w-80 bg-white rounded-3xl shadow-[inset_0_4px_4px_0_rgba(0,0,0,0.25)] pl-10 invalid:border-2 invalid:border-red-800 invalid:animate-pulse peer",
                        "type": "email",
                        placeholder: "What is your Email?"
                    }
                    div { class: "absolute translate-y-11 -translate-x-4 w-72 h-0.5 rounded-[50px] bg-[linear-gradient(270deg,red,pink_36%,yellow,green,purple)] peer-invalid:hidden" }
                    button { class: "absolute -translate-x-4 translate-y-1 w-16 h-8 rounded-2xl shadow-[0_4px_4px_0_rgba(0,0,0,0.25)] flex items-center justify-center peer-invalid:hidden",
                        span { class: "text-sm font-light text-center text-[rgb(97,63,63)]",
                            "book"
                        }
                    }
                }
                div { class: "w-full rounded-xl bg-gray-100 flex flex-col",
                    div { class: " h-14 mt-8 mb-4",
                        div { class: "w-[90%] h-full mx-auto flex items-center px-3 border-b-2 border-black text-3xl font-medium",
                            "{year}"
                        }
                    }
                    div { class: "flex-1 w-[90%] mx-auto flex",
                        div { class: "relative top-16", TimeTable {} }
                        Table { date_time: days }
                    }
                }
            }
        }
    }
}

