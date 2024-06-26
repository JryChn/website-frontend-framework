use dioxus::prelude::*;

#[component]
pub fn Quote(description_quote: String) -> Element {
    let quote = check_and_generate_effect(description_quote);
    rsx! {
        div { class: "absolute w-full h-[500px] flex justify-center mx-auto bg-[rgb(249,248,248)] select-none cursor-default overflow-hidden md:shadow-[0_4px_4px_0_rgba(0,0,0,0.25)] md:h-96 md:translate-x-1/2 md:right-1/2 md:-translate-y-2/3 md:w-5/6 md:justify-normal dark:bg-[rgb(27,36,50)]",
            div { class: "hidden w-1/3 items-center justify-center md:flex md:animate-showFromDown",
                div { class: "w-36 h-36 bg-white rounded-full flex items-center justify-center dark:bg-black",
                    div { class: "w-1/2 h-1/2 bg-black rounded-full flex items-center justify-center dark:bg-white",
                        div { class: "relative border border-white w-1/2 left-1/4 origin-left animate-rotated dark:border-black" }
                    }
                }
            }
            div { class: "relative w-2/3 md:animate-showFromDown",
                div { class: "relative h-1/3",
                    div { class: "absolute w-20 h-14 flex flex-row items-center justify-center bottom-0 text-lg font-medium left-1/2 -translate-x-1/2 md:left-0 md:translate-x-0 dark:text-white",
                        "About Me"
                    }
                }
                div { class: "border border-b-black w-full md:w-5/6 dark:border-b-gray-100" }
                div {
                    class: "relative flex-1 w-full my-6 font-normal text-left md:w-5/6 md:left-8 dark:text-white",
                    dangerous_inner_html: "{quote}"
                }
            }
        }
    }
}

fn check_and_generate_effect(quote: String) -> String {
    // use special decoration to replace /** test **/ in paragraph.
    let special_content_wrapper_start = r##"<span class="before:block before:absolute before:-inset-1 before:-skew-y-3 before:bg-green-800 relative inline-block mx-3"><span class="relative text-white">"##;
    let special_content_wrapper_end = r##"</span></span>"##;
    let quote = quote.replace(" **/", special_content_wrapper_end);
    let quote = quote.replace("/** ", &*special_content_wrapper_start);
    quote
}
