#![allow(non_snake_case)]

use charming::element::{Emphasis, ItemStyle, Label, LabelPosition};
use charming::series::Pie;
use charming::theme::Theme;
use charming::{Chart, WasmRenderer};
use dioxus::prelude::*;
use crate::component::homepage::about_me::AboutMeContext;
use crate::model;
use crate::model::config::AboutMePage;


use crate::utils::encryptedUtils::{fetch_and_decrypt, fetch_configuration};
use crate::utils::netUtils::parse_to_data_url;


#[inline_props]
pub fn AboutMeContent(cx: Scope) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64, 0f64);
    // use special decoration to replace **/test/** in paragraph.
    let special_content_wrapper_start = r##"<span class="before:block before:absolute before:-inset-1 before:-skew-y-3 before:bg-pink-500 relative inline-block mx-3"><span class="relative text-white">"##;
    let special_content_wrapper_end = r##"</span></span>"##;
    let fetch = use_future(cx, (), |_| async  {
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
                rsx!( div { class:"w-screen h-screen relative",
                    div{ class:"absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 text-5xl font-bold",
                     "Loading...."
                    }
                    } ),
        Some(aboutMe)=> {
            let aboutMeContent = aboutMe.description.replace(" **/", special_content_wrapper_start);
            let aboutMeContent = aboutMeContent.replace("/** ", special_content_wrapper_end);
            let _ = aboutMe.stage.iter().map(|stage| {
                let mut stages = Vec::new();
                let _ = stage.value.iter().map(|s| {
                    stages.push((s.value as i32, &s.name))
                });
                let chart = Chart::new().series(Pie::new().name(&stage.category).radius(vec!["40%", "70%"]).avoid_label_overlap(true).item_style(ItemStyle::new().border_radius(4).border_color("#fff").border_width(1)).emphasis(Emphasis::new().label(Label::new().show(true).font_size(40))).label(Label::new().show(false).position(LabelPosition::Center))
                    .data(
                        stages.to_vec()));
                WasmRenderer::new(250, 250).theme(Theme::Vintage).render(&stage.category, &chart).unwrap();
            });
            rsx!(
            div { id: "aboutme_content",
                class:"bg-gray-200 w-screen min-h-[2000px]",
                div { id: "aboutme_content_contact" }
                div { id: "aboutme_content_main",
                     class:"w-5/6 min-h-[800px] mx-auto translate-y-44 relative flex flex-wrap justify-around",
                    img { id: "aboutme_content_image",
                        class:"border border-black w-[400px] h-[500px] rounded-lg shadow-xl mx-10 my-10",
                        src:"{aboutMe.image}",
                        alt:"",
                    }
                    pre { id: "aboutme_content_description",
                        class:"block w-1/2 h-[400px] text-xl p-14 mx-10 my-10 font-sans",
                        dangerous_inner_html:"{aboutMeContent}",
                    }
                }
                div { id: "aboutme_content_stage",
                    class:"w-5/6 h-[500px] mx-auto flex flex-wrap flex-row justify-around items-center",
                        aboutMe.stage.iter().map(|stage|{
                            rsx!(
                                div{
                                    id:"{stage.category}"
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
