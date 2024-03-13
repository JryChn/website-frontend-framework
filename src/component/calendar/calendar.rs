use dioxus::prelude::*;

#[component]
pub fn Calendar(cx: Scope) -> Element {
    render!(
        div { id: "calendar",
            div { id: "calendar_box" }
            div { id: "calendar_button" }
        }
    )
}
