use dioxus::prelude::*;

use crate::component::aboutMeContent::ExperienceContent;

#[component]
pub fn Experience(experiences: Vec<ExperienceContent>) -> Element {
    let local_experience: Vec<(&ExperienceContent, (Signal<bool>, Signal<(&str, &str)>))> =
        experiences
            .iter()
            .map(|a| {
                (
                    a,
                    (use_signal(|| false), use_signal(|| css_for_hidden(false))),
                )
            })
            .collect();
    rsx! {
        div { class: "bg-[rgb(195,201,195)] flex flex-col select-none cursor-default",
            div { class: "w-screen h-[500px] md:h-56" }
            div { class: "flex justify-around ",
                div { class: "text-5xl font-medium md:absolute md:right-20", "Experience" }
            }
            div { class: "w-[90%] mx-auto border-b border-black mt-16 md:w-5/6 md:mt-32 " }
            div { class: "w-3/4 mx-auto mb-64",
                for mut exp in local_experience {
                    li { class: "border-gray-600 border-b w-full py-10 flex flex-col my-16",
                        div { class: "h-1/5 flex flex-row font-normal text-sm",
                            div { class: "w-1/4 flex items-center justify-start", "{exp.0.start_time}" }
                            div { class: "w-1/4 flex items-center justify-center",
                                div {
                                    class: "cursor-pointer rounded-full w-4 h-4 bg-white -translate-x-1/2 before:block before:rounded-full before:w-4 before:h-4 before:bg-[rgb(195,201,195)] before:-translate-x-1/3 duration-700 {exp.1.1().1}",
                                    onclick: move |_| {
                                        exp.1.0.set(!exp.1.0());
                                        exp.1.1.set(css_for_hidden(exp.1.0()));
                                    }
                                }
                                "{exp.0.title}"
                            }
                            ul { class: "flex-1 flex justify-end items-center",
                                for key in &exp.0.keywords {
                                    li { class: "border border-[rgb(20,89,41)] rounded-2xl px-4 py-0.5 mx-1 hover:bg-[rgb(20,89,41)]",
                                        "{key}"
                                    }
                                }
                            }
                        }
                        div { class: "flex items-start justify-center mt-10 group/description overflow-hidden duration-700 {exp.1.1().0}",
                            div { class: "w-5/6 text-sm font-light", "{exp.0.description}" }
                        }
                    }
                }
            }
        }
    }
}

fn css_for_hidden(hidden: bool) -> (&'static str, &'static str) {
    if hidden {
        (" h-0 ", " rotate-90 ")
    } else {
        (" h-auto ", " rotate-0")
    }
}
