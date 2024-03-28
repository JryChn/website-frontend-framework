use charming::{Chart, val};
use charming::component::{RadarCoordinate, RadarIndicator, Title};
use charming::series::Radar;
use charming::theme::Theme;
use dioxus::prelude::*;

use crate::component::aboutMeContent::SkillContent;

struct Skills{
    title:String,
    description:String,
    chart : Chart
}

#[component]
pub fn Skill(skill_content: Vec<SkillContent>) ->Element{
    let mut charts = Vec::new();
    for skill in skill_content{
        charts.push(create_radar(&skill));
    }
    rsx!{
        div { class: "relative bg-transparent -top-48 select-none cursor-default",
            div { class: "hidden text-5xl font-medium m-12 md:inline-block", "技能和技巧" }
            div { class: "w-full bg-[rgb(27,46,77)] min-h-[400px] mx-auto flex md:w-3/4",
                // small screen render
                div{
                    class:"flex flex-col item-center md:hidden",
                    for chart in charts{
                    div{
                       class:"w-screen my-16 flex flex-col",
                    div { class: "border-gray-950 border-2 w-72 h-72 mx-auto",

                            }
                div { class: "flex flex-col my-8 items-center",
                    div { class: "text-4xl font-normal text-white",
                        "{chart.title}"
                    }
                    div { class: "w-96 text-lg text-white font-light my-4",
                        "{chart.description}"
                    }
                }
                    }

                    }
                    // add for here
                }
                // big screen render
                div{
                   class:"hidden flex-row w-full h-full md:flex",
                    div{
                        class:"w-[800px] h-[400px]",
                       div{
                            class:"w-full h-full flex items-center",
                    div { class: "border-gray-950 border-2 w-72 h-72 mx-auto"}
                        }
                    }
                    div{
                        class:"flex-1 flex flex-col my-8",
                    div { class: "text-4xl font-normal text-white -translate-x-4",
                        "Skill Title"
                    }
                    div { class: "w-96 text-lg text-white font-light my-4",
                        "Skill Description, descripte this skill is for what and the meaning of it, it should be long setence or just a few word."
                    }
                    }
                }
            }
        }
    }
}

fn create_radar(skill: &SkillContent)->Skills{
    let mut indicators  = Vec::new();
    let mut actual_values = Vec::new();
    for r in skill.skill_content{
        let indicator = RadarIndicator::new().name(r.name).min(0).max(10);
        indicators.push(indicator);
        // filter values
        let mut value = r.value as f64;
        while value >10f64{
            value = value/10f64;
        }
        actual_values.push(value);
    }
     Skills{
        title:skill.clone().skill_name,
        description: skill.clone().description,
        chart:
        Chart::new().title(Title::new().text(skill.clone().skill_name)).radar(RadarCoordinate::new().indicator(indicators
        )).series(Radar::new().data(Vec::from(actual_values)))
    }
}
