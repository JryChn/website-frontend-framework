use dioxus::prelude::*;

#[component]
pub fn Skill() ->Element{
    rsx!{
        div { class: "relative bg-transparent -top-48",
            div { class: "hidden text-5xl font-medium m-12 md:inline-block", "技能和技巧" }
            div { class: "w-full bg-[rgb(27,46,77)] min-h-[400px] mx-auto flex md:w-5/6",
                // small screen render
                div{
                    class:"flex flex-col item-center md:hidden",
                    div{
                       class:"w-screen my-16 flex flex-col",
                    div { class: "border-gray-950 border-2 w-72 h-72 mx-auto"}
                div { class: "flex flex-col my-8 items-center",
                    div { class: "text-4xl font-normal text-white",
                        "Skill Title"
                    }
                    div { class: "w-96 text-lg text-white font-light my-4",
                        "Skill Description, descripte this skill is for what and the meaning of it, it should be long setence or just a few word."
                    }
                }
                    }
                }
                // big screen render
            }
        }
    }
}
