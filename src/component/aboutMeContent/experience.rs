use dioxus::prelude::*;

use crate::component::aboutMeContent::ExperienceContent;

#[component]
pub fn Experience(experiences: Vec<ExperienceContent>) -> Element {
    let mut hidden_description = use_signal(|| false);
    let hidden_description_css = if hidden_description() {
        (" h-0 ", " rotate-0 ")
    } else {
        (" ", " rotate-90 ")
    };
    rsx! {
        div { class: "bg-[rgb(195,201,195)] flex flex-col select-none cursor-default",
            div { class: "w-screen h-[500px] md:h-56" }
            div { class: "flex justify-around ",
                div { class: "text-5xl font-medium md:absolute md:right-20", "Experience" }
            }
            div { class: "w-[90%] mx-auto border-b border-black mt-16 md:w-5/6 md:mt-32 " }
            div { class: "w-3/4 mx-auto mb-64",
                for exp in experiences {
                    li { class: "border-gray-600 border-b w-full py-10 flex flex-col my-16",
                        div { class: "h-1/5 flex flex-row font-normal text-sm",
                            div { class: "w-1/4 flex items-center justify-start", "{exp.start_time}" }
                            div { class: "w-1/4 flex items-center justify-center",
                                div {
                                    class: "cursor-pointer rounded-full w-4 h-4 bg-white -translate-x-1/2 before:block before:rounded-full before:w-4 before:h-4 before:bg-[rgb(195,201,195)] before:-translate-x-1/3 duration-700 {hidden_description_css.1}",
                                    onclick: move |_| {
                                        hidden_description.set(!hidden_description());
                                    }
                                }
                                "{exp.title}"
                            }
                            ul { class: "flex-1 flex justify-end items-center",
                                for key in exp.keywords {
                                    li { class: "border border-[rgb(20,89,41)] rounded-2xl px-4 py-0.5 mx-1 hover:bg-[rgb(20,89,41)]",
                                        "{key}"
                                    }
                                }
                            }
                        }
                        div { class: "flex items-start justify-center mt-10 group/description overflow-hidden duration-700 {hidden_description_css.0}",
                            div { class: "w-5/6 text-sm font-light", "{exp.description}" }
                        }
                    }
                }
            }
        }
    }
}
