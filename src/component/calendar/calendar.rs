use dioxus::prelude::*;

#[component]
pub fn Calendar() -> Element {
    rsx!{
        div { id: "calendar",
            div { id: "calendar_box" }
            div { id: "calendar_button" }
        }
    }
}
