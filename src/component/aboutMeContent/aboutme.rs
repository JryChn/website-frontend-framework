use dioxus::prelude::*;

#[component]
pub fn AboutMe(title:String,subtitle:String,image1:String,image2:String) ->Element{
    rsx!{
        div{
        class: "relative w-screen min-h-[1000px] top-18",
        AboutMeTitle{title,subtitle,image:&image2},
        Circle{}
        AboutMeImages{image_url:image1,image2_url:&image2},
        }
    }
}


#[component]
fn AboutMeTitle(title:String,subtitle:String,image:String) ->Element{
    rsx!{
            div {
                class:"absolute w-96 h-64 top-44 left-24",
                div {
                    class: "h-1/3 flex flex-row items-center justify-center text-center font-semibold text-3xl ",
                    "{title}"
                }
            img{
                class: "w-72 h-96 rounded-tl-[149px] shadow-[6px_1px_8px_3px_rgba(0,0,0,0.25)] mx-auto my-20 scale-125 md:hidden",
                src:"{image}"
            }
                div { class: "absolute w-full md:w-12 border-t border-black" }
                    div { class: "absolute w-full h-full my-8 md:my-4 text-sm font-normal text-left tracking-normal",
                        "/* {subtitle}"
                }
            }
    }
}

#[component]
fn Circle()->Element{
    rsx!{
            div { id: "circle", class: "hidden absolute w-96 h-40 top-[460px] left-[50vw] md:block",
                div {
                    id: "circle_1",
                    class: "border w-40 h-40 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                    div {
                        id: "",
                        class: "border w-32 h-32 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                        div {
                            id: "",
                            class: "border w-24 h-24 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                            div {
                                id: "",
                                class: "border w-16 h-16 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                                div {
                                    id: "",
                                    class: "border w-8 h-8 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center bg-[rgb(96,24,123)]"
                                }
                            }
                        }
                    }
                }
                div {
                    id: "circle_2",
                    class: "border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-full translate-x-2/3"
                }
                div {
                    id: "circle_3",
                    class: "border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-[200%] translate-x-[120%]"
                }
            }

    }

}
#[component]
fn AboutMeImages(image_url:String,image2_url:String)->Element{
rsx!{
    div{
        class:"hidden md:flex absolute top-52 right-14",
            div {
                id: "image_1",
                class: "w-56 h-96 rounded-tl-[110px] shadow-[-9px_8px_25px_3px_rgba(0,0,0,0.25)] mx-6 my-2",
                img { src: "{image_url}" }
            }
            div {
                id: "image_2",
                class: "w-72 h-96 rounded-tl-[149px] shadow-[6px_1px_8px_3px_rgba(0,0,0,0.25)] mx-6 my-2",
                img { src: "{image2_url}" }
            }
    }
}
}
