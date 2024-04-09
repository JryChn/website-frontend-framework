use charming::{Chart, WasmRenderer};
use charming::component::{RadarCoordinate, RadarIndicator, Title};
use charming::element::Color;
use charming::series::Radar;
use charming::theme::Theme;
use dioxus::prelude::*;
use rand::prelude::SliceRandom;
use uuid::Uuid;

use crate::component::aboutMeContent::SkillContent;

struct Skills {
    title: String,
    description: String,
    chart: Option<VNode>,
}

#[component]
pub fn Skill(skill_content: Vec<SkillContent>) -> Element {
    // todo: make skill can scroll
    let mut charts = Vec::new();
    for skill in skill_content {
        charts.push(create_radar(&skill));
    }
    rsx! {
        div { class: "relative bg-transparent -top-48 select-none cursor-default",
            div { class: "hidden text-5xl font-medium m-12 md:inline-block", "技能和技巧" }
            div { class: "w-full bg-[rgb(27,46,77)] min-h-[400px] mx-auto flex md:w-3/4",
                div { class: "flex flex-col item-center md:block md:w-full md:h-[400px] md:overflow-hidden",
                    for chart in charts {
                        div { class: "w-screen flex flex-col md:w-full md:h-full md:flex-row",
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
