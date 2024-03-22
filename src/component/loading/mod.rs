use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        div { class: "w-screen h-screen relative",
            div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold",
                "Loading...."
            }
        }
    }
}
