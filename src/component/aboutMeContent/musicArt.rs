use dioxus::prelude::*;

#[component]
pub fn MusicAndArt(video_url:String,video2_url:String) ->Element{
    rsx! {
        div { class: "relative w-full flex flex-col bg-black top-14",
            div { class: "relative h-28 my-8 flex justify-evenly md:block",
                div { class: "absolute text-5xl font-medium text-white md:left-8", "Music And Art" }
            }
            div { class: "h-[60vw] w-[60vw] mx-auto flex justify-end",
                video {
                    class: "h-3/4 w-3/4 rounded-[36px]",
                    src: "{video_url}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true"
                }
            }
            div { class: "h-[60vw] w-[60vw] mx-auto flex justify-start",
                video {
                    class: "h-3/4 w-3/4 rounded-[36px]",
                    src: "{video2_url}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true"
                }
            }
        }
    }
}
