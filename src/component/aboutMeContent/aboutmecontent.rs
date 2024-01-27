#![allow(non_snake_case)]

use charming::{Chart, WasmRenderer};
use charming::component::Title;
use charming::element::{Color, Emphasis, ItemStyle, Label, Tooltip, Trigger};
use charming::series::{Pie, PieRoseType};
use charming::theme::Theme;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::component::loading::Loading;
use crate::model;
use crate::model::ConfigurationTemplate;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType;

#[inline_props]
pub fn AboutMeContent(cx: Scope) -> Element {
    gloo::utils::window().scroll_with_x_and_y(0f64, 0f64);
    // let typing_words = use_state(cx, || "".to_string());
    // let configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read();
    // let welcome = &configuration.welcome;
    // // words blink type animation
    // use_future(cx, (), |_| {
    //     to_owned![typing_words];
    //     let whole_str = welcome.subtitle.to_owned();
    //     async move {
    //         loop {
    //             for i in whole_str.iter() {
    //                 let whole_str = i;
    //                 gloo::timers::future::sleep(Duration::from_millis(1000)).await;
    //                 let mut init_string = "".to_string();
    //                 for i in whole_str.chars() {
    //                     if i != '_' {
    //                         init_string.push(i);
    //                     }
    //                     gloo::timers::future::sleep(Duration::from_millis(300)).await;
    //                     typing_words.set(init_string.to_string());
    //                 }
    //                 gloo::timers::future::sleep(Duration::from_millis(1500)).await;
    //                 loop {
    //                     if init_string.len() == 0 {
    //                         break;
    //                     }
    //                     init_string.pop();
    //                     gloo::timers::future::sleep(Duration::from_millis(200)).await;
    //                     typing_words.set(init_string.to_string());
    //                 }
    //             }
    //         }
    //     }
    // });
    let configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read().clone();
    let color_generator = ||{
        let mut value = Vec::new();
            let color_char = vec![
                "a","b","c","d","e","f","0","1","2","3","4","5","6","7","8","9"
            ];
        (1 ..7).for_each( |_| {
            value.push(
            color_char.choose(&mut thread_rng()).unwrap().to_string().clone());
        });
        value.into_iter().collect::<String>()
    };
    // use special decoration to replace **/test/** in paragraph.
    let special_content_wrapper_start =r##"<span class="before:block before:absolute before:-inset-1 before:-skew-y-3 before:bg-green-800 relative inline-block mx-3"><span class="relative text-white">"##;
    let special_content_wrapper_end = r##"</span></span>"##;
    let fetch = use_future(cx, (), |_|  async {
        let mut content :model::AboutMe::AboutMePage;
        let url = configuration.about_me_api;
        if url.is_empty() {
            content = serde_json::from_str(include_str!("../../defaultConfig/aboutMe.json")).unwrap()
        }else{
            content =  fetch_and_decrypt(url.as_str()).await;

        }
        content.image = parse_to_data_url(content.image.clone(),ResourceType::IMAGE).await;
        content
    });
    let github_username = configuration.contact.github_username.clone();
    cx.render(match fetch.value() {
        None =>
                rsx!( Loading {} ),
        Some(aboutMe)=> {
            let aboutMeContent = aboutMe.description.replace(" **/", special_content_wrapper_end);
            let aboutMeContent = aboutMeContent.replace("/** ", &*special_content_wrapper_start);
            let github_states="https://github-readme-stats.vercel.app/api?username=".to_string()+&github_username+"&count_private=true&show_icons=true&title_color=ffffff&text_color=ffffff&icon_color=ffa502&bg_color=009432,9980FA,6F1E51";
            rsx!(
                div {
                    id: "aboutme_content",
                    class: "bg-gray-200 w-screen min-h-[2000px]",
                    div { id: "aboutme_content_contact" }
                    div {
                        id: "aboutme_content_main",
                        class: "w-5/6 min-h-[800px] mx-auto translate-y-44 relative flex flex-wrap justify-around",
                        img {
                            id: "aboutme_content_image",
                            class: "border border-black w-[400px] h-[500px] rounded-lg shadow-xl mx-10 my-10",
                            src: "{aboutMe.image}",
                            alt: ""
                        }
                        div {
                            id: "aboutme_content_description",
                            class: "block w-1/2 h-[400px] text-xl p-14 mx-10 my-10 font-sans break-words",
                            dangerous_inner_html: "{aboutMeContent}"
                        }
                    }
                    div {
                        id: "aboutme_content_stage",
                        class: "w-5/6 h-[500px] mx-auto flex flex-wrap flex-row justify-around items-center",
                        aboutMe.stage.iter().map(|stage|{
                            rsx!(
                                div{
                                    id:"{stage.category}",
                                    onmounted: move |_| {
                        let mut stages = Vec::new();
                        stage.value.iter().for_each(|s| {
                        stages.push((s.value as i32, &s.name))
                        });
                        let chart = Chart::new().series(Pie::new().rose_type(PieRoseType::Area).name(&stage.category).radius(vec!["20%", "60%"]).avoid_label_overlap(false).item_style(ItemStyle::new().border_radius(4).border_color("#fff").border_width(1)).emphasis(Emphasis::new().label(Label::new().show(true).shadow_blur(10)))
                        .data(
                            stages)).title(Title::new().text(&stage.category).left("center").bottom("center")).tooltip(Tooltip::new().trigger(Trigger::Item)).color(
                        (1..10).map(|_|{
                        Color::from("#".to_string()+color_generator().as_str())
                        }).collect::<Vec<Color>>()
                        );
                        WasmRenderer::new(350, 350).theme(Theme::Essos).render(&stage.category, &chart).unwrap();
                                    }
                                }
                            )
                        })
                    }
                    div { id: "aboutme_content_github",
                        Link { to: "https://github.com/".to_string() + &github_username, img { alt: "", src: "{github_states}" } }
                    }
                }
            )
        }
    }
    )
}
