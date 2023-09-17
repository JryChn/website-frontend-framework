#![allow(non_snake_case)]

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::model::config::Welcome;
use crate::Route;

#[inline_props]
pub fn SingleWelcome(cx: Scope,welcome:Welcome) -> Element {
    let typing_words = use_state(cx, || "".to_string());
    use_future(cx, (), |_| {
        to_owned![typing_words];
        let whole_str = welcome.subtitle.to_owned();
        async move {
            loop {
                for i in whole_str.iter() {
                    let whole_str = i;
                    gloo_timers::future::sleep(Duration::from_millis(1000)).await;
                    let mut init_string = "".to_string();
                    for i in whole_str.chars() {
                        if i != '_' {
                            init_string.push(i);
                        }
                        gloo_timers::future::sleep(Duration::from_millis(300)).await;
                        typing_words.set(init_string.to_string());
                    }
                    gloo_timers::future::sleep(Duration::from_millis(1500)).await;
                    loop {
                        if init_string.len() == 0 {
                            break;
                        }
                        init_string.pop();
                        gloo_timers::future::sleep(Duration::from_millis(200)).await;
                        typing_words.set(init_string.to_string());
                    }
                }
            }
        }
    });
    let animation_url = if gloo_utils::document_element().class_list().contains("dark") {
        &welcome.animation_url.dark
    } else {
        &welcome.animation_url.light
    };
    let svg_bold = r###"
        <svg t="1689862785707" class="hidden md:block w-12 h-12 -translate-x-[1px] -translate-y-3" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3640" width="200" height="200"><path d="M510.3 327.1c-8.3 0-15-6.7-15-15V34.6c0-8.3 6.7-15 15-15s15 6.7 15 15v277.6c0 8.2-6.7 14.9-15 14.9z" fill="" p-id="3641"></path><path d="M762 881.8H260.6c-11.1 0-21.6-4.4-29.4-12.3-7.9-7.9-12.1-18.4-12.1-29.5 0.2-31.8 2.8-59.6 15-107.7 17.4-68.7 60.5-177.8 98.7-214v-180c0-22.7 18.5-41.1 41.1-41.1h273.4c22.7 0 41.2 18.5 41.2 41.1V517c38.5 35 82.4 145.8 100 215.3 12.2 48.2 14.8 76 15 107.7 0.1 11.1-4.2 21.6-12.1 29.6-7.8 7.9-18.2 12.2-29.4 12.2zM353.4 540c-29.4 27.4-71.9 127.6-90.2 199.7-11.5 45.4-13.8 70.4-14 100.6 0 3.1 1.2 6 3.3 8.2 2.2 2.2 5.1 3.4 8.1 3.4H762c3.1 0 6-1.2 8.1-3.4 2.2-2.2 3.4-5.1 3.3-8.2-0.2-30.1-2.5-55.1-14.1-100.5-18.7-73.7-62.7-176.7-92.2-201.4-5.3-3.1-8.8-8.9-8.8-15.2V338.3c0-6.1-5-11.1-11.2-11.1H374c-6.1 0-11.1 5-11.1 11.1V510h224.6c8.3 0 15 6.7 15 15s-6.7 15-15 15H353.4z" fill="" p-id="3642"></path><path d="M511.6 985.9c-65.6 0-119.1-53.4-119.1-119.1 0-8.3 6.7-15 15-15s15 6.7 15 15c0 49.1 40 89.1 89.1 89.1s89.1-40 89.1-89.1c0-8.3 6.7-15 15-15s15 6.7 15 15c0 65.7-53.4 119.1-119.1 119.1zM715.7 777.3c-7 0-13.3-5-14.7-12.1l-4.2-21.6c-3.3-16.6-9-32.6-17.2-47.5l-10.3-18.8c-4-7.3-1.3-16.4 6-20.4s16.4-1.3 20.4 6l10.3 18.8c9.6 17.6 16.4 36.5 20.3 56.2l4.2 21.6c1.6 8.1-3.7 16-11.8 17.6-1.1 0.2-2.1 0.2-3 0.2z" fill="" p-id="3643"></path><path d="M657.6 627.6m-17.5 0a17.5 17.5 0 1 0 35 0 17.5 17.5 0 1 0-35 0Z" fill="" p-id="3644"></path></svg>
        <svg t="1689863317694" class="w-8 h-8 -translate-y-4 translate-x-2 md:hidden" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4815" width="200" height="200"><path d="M513.123 795.991c-76.156 0-147.753-29.658-201.603-83.508-53.849-53.849-83.504-125.444-83.504-201.596 0-76.153 29.657-147.749 83.504-201.598 53.85-53.851 125.446-83.508 201.603-83.508 76.147 0 147.742 29.658 201.593 83.508 53.853 53.852 83.511 125.447 83.511 201.598s-29.659 147.747-83.511 201.597c-53.852 53.85-125.445 83.507-201.593 83.507zM513.123 272.352c-131.529 0-238.534 107.007-238.534 238.535s107.006 238.533 238.534 238.533 238.533-107.005 238.533-238.533c0-131.529-107.006-238.535-238.533-238.535z" fill="#CCCCCC" p-id="4816"></path><path d="M513.123 149.007c-12.861 0-23.285-10.426-23.285-23.285v-91.359c0-12.861 10.426-23.285 23.285-23.285s23.285 10.426 23.285 23.285v91.358c0 12.862-10.426 23.286-23.285 23.286z" fill="#CCCCCC" p-id="4817"></path><path d="M513.123 1010.674c-12.861 0-23.285-10.426-23.285-23.285v-91.346c0-12.861 10.426-23.285 23.285-23.285s23.285 10.426 23.285 23.285v91.346c0 12.861-10.426 23.285-23.285 23.285z" fill="#CCCCCC" p-id="4818"></path><path d="M240.764 261.825c-5.959 0-11.919-2.274-16.466-6.821l-64.592-64.591c-9.094-9.093-9.094-23.838 0-32.931 9.093-9.094 23.838-9.094 32.931 0l64.593 64.592c9.094 9.093 9.094 23.838 0 32.931-4.546 4.545-10.507 6.82-16.467 6.82z" fill="#CCCCCC" p-id="4819"></path><path d="M850.050 871.111c-5.958 0-11.919-2.272-16.467-6.82l-64.593-64.593c-9.093-9.094-9.093-23.839 0-32.932 9.094-9.094 23.839-9.094 32.932 0l64.593 64.593c9.093 9.094 9.093 23.839 0 32.932-4.546 4.545-10.507 6.82-16.466 6.82z" fill="#CCCCCC" p-id="4820"></path><path d="M36.59 534.183c-12.861 0.001-23.286-10.423-23.289-23.283-0.001-12.861 10.423-23.286 23.283-23.289l91.348-0.011c12.861-0.001 23.286 10.423 23.289 23.283 0.001 12.861-10.423 23.286-23.283 23.289l-91.348 0.011z" fill="#CCCCCC" p-id="4821"></path><path d="M989.613 534.173h-91.346c-12.861 0-23.285-10.426-23.285-23.285s10.426-23.285 23.285-23.285h91.346c12.861 0 23.285 10.426 23.285 23.285s-10.426 23.285-23.285 23.285z" fill="#CCCCCC" p-id="4822"></path><path d="M176.173 871.122c-5.959 0-11.919-2.272-16.466-6.82-9.094-9.094-9.094-23.839 0-32.932l64.593-64.593c9.094-9.094 23.838-9.094 32.931 0s9.094 23.839 0 32.932l-64.593 64.593c-4.546 4.546-10.506 6.82-16.466 6.82z" fill="#CCCCCC" p-id="4823"></path><path d="M785.457 261.825c-5.959 0-11.918-2.272-16.467-6.821-9.094-9.094-9.093-23.838 0.001-32.931l64.593-64.592c9.094-9.093 23.838-9.094 32.931 0s9.093 23.838-0.001 32.931l-64.593 64.592c-4.545 4.546-10.506 6.821-16.465 6.821z" fill="#CCCCCC" p-id="4824"></path></svg>
        <svg t="1689863463497" class="w-8 h-8 -translate-y-[47px]  translate-x-12 fill-amber-200 md:hidden" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="5910" width="200" height="200"><path d="M644.5056 70.528C834.4064 127.488 972.8 303.5648 972.8 512c0 254.4896-206.3104 460.8-460.8 460.8-222.4128 0-408.0128-157.568-451.2768-367.1296A433.4848 433.4848 0 0 0 230.4 640c240.3584 0 435.2-194.8416 435.2-435.2 0-44.2112-6.5792-86.8608-18.8416-127.0528z" p-id="5911"></path></svg>
        "###;
    let svg_calendar = r###"<svg t="1689862655113" class="w-8 h-8 inline-block" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2297" width="200" height="200"><path d="M400.756383 521.126868c-17.171078 0-31.092136 13.922081-31.092136 31.092136l0 30.332842c0 17.171078 13.922081 31.092136 31.092136 31.092136 17.170055 0 31.092136-13.922081 31.092136-31.092136l0-30.332842C431.848519 535.047926 417.926438 521.126868 400.756383 521.126868z" fill="#2c2c2c" p-id="2298"></path><path d="M622.698195 521.126868c-17.171078 0-31.092136 13.922081-31.092136 31.092136l0 30.332842c0 17.171078 13.922081 31.092136 31.092136 31.092136 17.170055 0 31.092136-13.922081 31.092136-31.092136l0-30.332842C653.791353 535.047926 639.869273 521.126868 622.698195 521.126868z" fill="#2c2c2c" p-id="2299"></path><path d="M588.406181 661.936871c-14.20042-9.351995-33.420157-5.374404-42.964534 8.714476-0.132006 0.197498-13.552667 19.669992-34.021861 19.669992-19.903306 0-32.281217-18.025539-33.121352-19.286252-9.134031-14.362102-28.157293-18.724457-42.635029-9.716292-14.589276 9.063423-19.068288 28.233018-10.004865 42.817178 11.158131 17.965164 41.785685 48.368614 85.762269 48.368614 43.763736 0 74.764797-30.176276 86.186941-48.004317C606.78169 690.169889 602.62195 671.299099 588.406181 661.936871z" fill="#2c2c2c" p-id="2300"></path><path d="M366.630145 220.097814c17.171078 0 31.092136-13.922081 31.092136-31.092136l0-10.364045 228.010017 0 0 10.364045c0 17.171078 13.922081 31.092136 31.092136 31.092136 17.170055 0 31.092136-13.922081 31.092136-31.092136L687.916569 95.728248c0-17.171078-13.922081-31.092136-31.092136-31.092136-17.171078 0-31.092136 13.922081-31.092136 31.092136l0 20.72809L397.72228 116.456339 397.72228 95.728248c0-17.171078-13.922081-31.092136-31.092136-31.092136s-31.092136 13.922081-31.092136 31.092136l0 93.276407C335.538009 206.175733 349.46009 220.097814 366.630145 220.097814z" fill="#2c2c2c" p-id="2301"></path><path d="M779.675412 100.035344c-17.170055 0-31.092136 13.922081-31.092136 31.092136s13.922081 31.092136 31.092136 31.092136c52.063773 0 94.415346 43.191708 94.415346 96.28288l0 121.04382L149.363819 379.546316 149.363819 258.430864c0-53.090149 42.351574-96.28288 94.415346-96.28288 17.171078 0 31.092136-13.922081 31.092136-31.092136s-13.922081-31.092136-31.092136-31.092136c-86.348624 0-156.599617 71.086012-156.599617 158.467151l0 543.03176c0 87.38114 70.250994 158.472268 156.594501 158.472268l535.901363 0.070608c86.348624 0 156.599617-71.091128 156.599617-158.472268L936.27503 258.502495C936.27503 171.121356 866.024036 100.035344 779.675412 100.035344zM779.680529 897.821229l-535.901363-0.070608c-52.063773 0-94.415346-43.191708-94.415346-96.287997L149.363819 441.730587l724.726939 0 0 359.802646C874.090758 854.629521 831.739185 897.821229 779.680529 897.821229z" fill="#2c2c2c" p-id="2302"></path></svg>"###;
    cx.render(rsx!(
        style { include_str!("css/single_welcome.css") }
        div {
            id: "welcome_page",
            class: "w-screen min-h-[1000px] bg-gray-200 dark:bg-black",
            Link { to: Route::HomePage {},
                h1 {
                    id: "title_logo",
                    class: "relative w-20 h-20 uppercase font-bold break-words md:text-4xl md:top-3.5 md:left-3.5 md:w-36 md:h-36 dark:text-gray-50",
                    "{welcome.title}"
                }
            }
            div {
                id: "welcome_video",
                class: "w-3/5 h-1/2  max-w-[900px] max-h-[450px] left-1/2 relative -translate-x-1/2",
                mix_blend_mode: "multiply",
                video {
                    class: "max-w-[900px] max-h-[450px] w-full h-full object-fill",
                    src: "{animation_url}",
                    autoplay: "true",
                    muted: "true",
                    "loop": "true",
                    playsinline: "true",
                }
            }
            div {
                id: "subtitle_wordplay_box",
                class: "w-1/2 h-16 border-t relative border-black left-1/2 -translate-x-1/2 dark:border-gray-50",
                h2 {
                    id: "subtitle_wordplay",
                    class: "mx-auto block  absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-2xl font-mono my-10 min-h-[34px] dark:text-gray-50",
                    "/*"
                    span {
                        class: "border-r border-black px-0 mx-2 dark:border-gray-50",
                        animation: "blink 0.75s infinite step-end",
                        style { include_str!("./css/single_welcome.css") }
                        "{typing_words}"
                    }
                    "*/"
                }
            }
            div {
                id: "light_line_box",
                class: "w-24 h-10 absolute top-1 right-20 border border-b-gray-300 rounded-3xl shadow-gray-500 hover:shadow-inner md:shadow-none md:hover:shadow-none md:border-none md:right-1/3 md:h-1/4 md:top-0 dark:border-b-gray-50 z-50",
                div {
                    id: "light_line",
                    class: "relative h-1/2 md:left-1/2 md:w-0 md:h-4/5 md:border md:border-black md:cursor-pointer md:dark:border-gray-50"
                }
                button {
                    id: "light_bold",
                    class: "md:w-11 md:h-11 md:relative md:left-1/2 md:-translate-x-1/2 md:dark:after:block dark:md:after:w-32 dark:md:after:h-32 dark:md:after:bg-amber-200 dark:md:after:relative dark:md:after:top-1/2 dark:md:after:rounded-[50%] dark:md:after:right-1/2 dark:md:after:-translate-x-4 dark:md:after:-translate-y-2/3 dark:md:after:blur-2xl dark:md:after:opacity-80 dark:md:after:animate-[pulse_6s_cubic-bezier(0.4,0,0.6,1)_infinite] dark:stroke-gray-50 dark:fill-gray-50",
                    dangerous_inner_html: "{svg_bold}",
                    onclick: |e| {
                        let dom = gloo_utils::document_element();
                        dom.class_list().toggle("dark");
                    }
                }
            }
            Link {
                id: "book_my_time",
                to: Route::Calendar {},
                class: "hidden md:block md:absolute md:right-10 md:top-2 dark:text-gray-50",
                div { class: "inline", dangerous_inner_html: "{svg_calendar}" }
                span { "book my time" }
            }
        }
    ))
}
