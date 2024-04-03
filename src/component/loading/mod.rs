use dioxus::prelude::*;

#[component]
pub fn Loading(black_bg: Option<bool>) -> Element {
    let black_bg = if black_bg.unwrap_or(false) {
        " bg-black text-white "
    } else {
        ""
    };
    rsx! {
        div { class: "w-screen h-screen relative {black_bg}",
            div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold animate-pulse",
                "Loading...."
            }
        }
    }
}
