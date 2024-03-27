use dioxus::prelude::*;

#[component]
pub fn Habbit() ->Element{
    rsx!{
        div { class: "h-[1000px]",
            div { class: "absolute w-5/6 border border-b-black right-0" }
            div { class: "relative left-[20%] top-6 text-4xl ", "Sport" }
            div { class: "relative h-96 bg-green-800 top-14" }
            div { class: "h-96 bg-blue-800" }
            div { class: "h-96 bg-yellow-800" }
            div { class: "h-96 bg-blue-800" }
        }
    }
}
