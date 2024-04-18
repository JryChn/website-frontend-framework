use std::ops::Sub;
use std::time::Duration;

use charming::{Chart, WasmRenderer};
use charming::component::{RadarCoordinate, RadarIndicator, Title};
use charming::element::Color;
use charming::series::Radar;
use charming::theme::Theme;
use dioxus::prelude::*;
use gloo::console::__macro::JsValue;
use gloo::console::console_dbg;
use rand::prelude::SliceRandom;
use uuid::Uuid;
use web_sys::{HtmlElement, NodeList};
use web_sys::wasm_bindgen::JsCast;

use crate::component::aboutMeContent::SkillContent;

#[derive(Clone, PartialEq)]
struct Skills {
    title: String,
    description: String,
    chart: Option<VNode>,
}

#[component]
pub fn Skill(skill_content: Vec<SkillContent>) -> Element {
    let mut charts = Vec::new();
    let mut md_charts = Vec::new();
    for skill in skill_content {
        charts.push(create_radar(&skill));
        md_charts.push(create_radar(&skill));
    }
    rsx! {
        div { class: "relative bg-transparent -top-48 select-none cursor-default",
            div { class: "hidden text-5xl font-medium m-12 md:inline-block dark:text-white",
                "技能和技巧"
            }
            div { class: "w-full bg-[rgb(27,46,77)] min-h-[400px] mx-auto flex md:w-3/4",
                div { class: "flex flex-col item-center md:hidden",
                    for chart in charts {
                        div { class: "w-screen flex flex-col",
                            {chart.chart},
                            div { class: "flex flex-col my-8 mx-16 items-center",
                                div { class: "text-4xl font-normal text-white", "{chart.title}" }
                                div { class: "w-96 text-lg text-white font-light my-4",
                                    "{chart.description}"
                                }
                            }
                        }
                    }
                }
                div { class: "hidden item-center w-full h-[400px] overflow-hidden md:block",
                    div { class: "relative flex w-full h-full flex-row justify-around",
                        MdScreenRadarRender { md_charts }
                    }
                }
            }
        }
    }
}

fn create_radar(skill: &SkillContent) -> Skills {
    let mut indicators = Vec::new();
    let mut actual_values = Vec::new();
    for r in skill.skill_content.clone() {
        let indicator = RadarIndicator::new().name(r.name).min(0).max(10);
        indicators.push(indicator);
        // filter values
        let mut value = r.value as f64;
        while value > 10f64 {
            value = value / 10f64;
        }
        actual_values.push(value);
    }
    let id = Uuid::new_v4().as_u128().to_string();
    let chart = Chart::new()
        .background_color(Color::from("rgb(0,0,0,0)"))
        .color(vec![get_random_color()])
        .radar(RadarCoordinate::new().indicator(indicators))
        .series(Radar::new().data(vec![(actual_values)]));
    let chart = rsx! {
        div {
            id: "{id}",
            class: "w-96 h-96 mx-auto",
            onmounted: move |_| {
                WasmRenderer::new(400, 400).theme(Theme::Essos).render(&id, &chart).unwrap();
            }
        }
    };
    Skills {
        title: skill.clone().skill_name,
        description: skill.clone().description,
        chart,
    }
}

fn get_random_color() -> Color {
    let color = vec![
        "blue", "yellow", "#c23531", "#2f4554", "#61a0a8", "#d48265", "#91c7ae", "#749f83",
        "#ca8622", "#bda29a", "#6e7074", "#546570", "#c4ccd3",
    ];
    Color::from(*color.choose(&mut rand::thread_rng()).unwrap_or(&"yellow"))
}

#[component]
fn MdScreenRadarRender(md_charts: Vec<Skills>) -> Element {
    let mut current_radar_index = use_signal(|| 0);
    let radars = use_signal(|| md_charts);
    let _ = use_resource(move || async move {
        loop {
            let current = current_radar_index();
            let new = (current + 1) % radars().len();
            current_radar_index.set(new);
            let current_index = current_radar_index() as u32;
            let left_index = ((current_index as i32 - 1).rem_euclid(radars().len() as i32)) as u32;
            let right_index = (((current_index + 1) as i32) % radars().len() as i32) as u32;
            let all_radars = gloo::utils::document_element().query_selector_all("#md_radar");
            match all_radars {
                Ok(radars) => {
                    for i in 0..radars.length() {
                        let radar = radars.get(i).unwrap().dyn_into::<HtmlElement>().unwrap();
                        radar.style().set_property("display", "hidden").unwrap();
                        radar.style().remove_property("transform").unwrap();
                        radar.style().remove_property("opacity").unwrap();
                    }
                    let current_radar = radars.get(current_index);
                    let left_radar = radars.get(left_index);
                    let right_radar = radars.get(right_index);
                    if current_radar.is_some() {
                        let current_radar =
                            current_radar.unwrap().dyn_into::<HtmlElement>().unwrap();
                        current_radar
                            .style()
                            .set_property("display", "block")
                            .unwrap();
                    }
                    if left_radar.is_some() {
                        let left_radar = left_radar.unwrap().dyn_into::<HtmlElement>().unwrap();
                        left_radar.style().set_property("display", "block").unwrap();
                        left_radar
                            .style()
                            .set_property(
                                "transform",
                                "rotateY(30deg) translateX(-140px) translateZ(-300px)",
                            )
                            .unwrap();
                        left_radar.style().set_property("opacity", "0.2").unwrap();
                    }
                    if right_radar.is_some() {
                        let right_radar = right_radar.unwrap().dyn_into::<HtmlElement>().unwrap();
                        right_radar
                            .style()
                            .set_property("display", "block")
                            .unwrap();
                        right_radar
                            .style()
                            .set_property(
                                "transform",
                                "rotateY(-30deg) translateX(140px) translateZ(-300px)",
                            )
                            .unwrap();
                        right_radar.style().set_property("opacity", "0.2").unwrap();
                    }
                }
                Err(_) => {}
            }
            gloo::timers::future::sleep(Duration::from_secs(5)).await;
        }
    });
    rsx! {
        div {
            class: "relative w-[600px] h-[400px] flex justify-around",
            style: "perspective: 500px;transform-style:preserve-3d",
            for chart in radars() {
                div { id: "md_radar", class: "absolute hidden duration-1000", {chart.chart} }
            }
        }
        div { class: "flex flex-col my-8 mx-16 items-center",
            div { class: "text-4xl font-normal text-white", "{radars()[current_radar_index()].title}" }
            div { class: "w-96 text-lg text-white font-light my-4",
                "{radars()[current_radar_index()].description}"
            }
        }
    }
}
