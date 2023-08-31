#![allow(non_snake_case)]

use std::ops::Deref;
use std::time::Duration;
use charming::element::{Color, Emphasis, ItemStyle, Label, LabelPosition, Tooltip, Trigger};
use charming::series::{Pie, PieRoseType};
use charming::theme::Theme;
use charming::{Chart, WasmRenderer};
use charming::component::Title;
use charming::element::TextVerticalAlign::Bottom;
use dioxus::prelude::*;
use futures::future::join_all;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::de::value;
use crate::component::homepage::about_me::AboutMeContext;
use crate::model;
use crate::model::AboutMe::Stage;
use crate::model::config::AboutMePage;


use crate::utils::encryptedUtils::{fetch_and_decrypt, fetch_configuration};
use crate::utils::netUtils::parse_to_data_url;


#[inline_props]
pub fn AboutMeContent(cx: Scope) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64, 0f64);

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
    let special_content_wrapper_start = r##"<span class="before:block before:absolute before:-inset-1 before:-skew-y-3 before:bg-pink-500 relative inline-block mx-3"><span class="relative text-white">"##;
    let special_content_wrapper_end = r##"</span></span>"##;
    let fetch = use_future(cx, (), |_| async {
        let mut content :model::AboutMe::AboutMePage;
        let url = fetch_configuration().await.about_me.api;
        if url.is_empty() {
            content = serde_json::from_str(include_str!("../../defaultConfig/aboutMe.json")).unwrap()
        }else{
            content =  fetch_and_decrypt(url.as_str()).await;

        }
        content.image = parse_to_data_url(content.image.clone()).await;
        content
    });
    cx.render(match fetch.value() {
        None =>
                rsx!(
                    div { class: "w-screen h-screen relative",
                        div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold",
                            "Loading...."
                        }
                    }
                ),
        Some(aboutMe)=> {
            let aboutMeContent = aboutMe.description.replace(" **/", special_content_wrapper_end);
            let aboutMeContent = aboutMeContent.replace("/** ", special_content_wrapper_start);
            rsx!(
                div { id: "aboutme_content", class: "bg-gray-200 w-screen min-h-[2000px]",
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
                    let chart = Chart::new().series(Pie::new().rose_type(PieRoseType::Area).name(&stage.category).radius(vec!["30%", "50%"]).avoid_label_overlap(false).item_style(ItemStyle::new().border_radius(4).border_color("#fff").border_width(1)).emphasis(Emphasis::new().label(Label::new().show(true).shadow_blur(10)))
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
                    div { id: "aboutme_content_github" }
                }
            )
        }
    }
    )
}
