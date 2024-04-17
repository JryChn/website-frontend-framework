use dioxus::prelude::*;
use futures::future::join_all;

use crate::component::aboutMeContent::aboutme::AboutMe;
use crate::component::aboutMeContent::experience::Experience;
use crate::component::aboutMeContent::hobby::Hobby;
use crate::component::aboutMeContent::musicArt::MusicAndArt;
use crate::component::aboutMeContent::quote::Quote;
use crate::component::aboutMeContent::skill::Skill;
use crate::component::loading::Loading;
use crate::model::AboutMe::AboutMePage;
use crate::model::ConfigurationTemplate;
use crate::utils::cache::Cache;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType;
use crate::utils::resourceType::ResourceType::{IMAGE, MP4};

mod aboutme;
mod experience;
mod hobby;
mod musicArt;
mod quote;
mod skill;

#[derive(Clone, PartialEq)]
struct HobbyContent {
    image_url: String,
    title: String,
    description: String,
}
#[derive(Clone, PartialEq)]
struct ExperienceContent {
    start_time: String,
    title: String,
    keywords: Vec<String>,
    description: String,
}
#[derive(Clone, PartialEq)]
struct SkillContent {
    skill_name: String,
    description: String,
    skill_content: Vec<RadarContent>,
}
#[derive(Clone, PartialEq)]
struct RadarContent {
    name: String,
    value: u32,
}
#[component]
pub fn AboutMeContent() -> Element {
    let configuration = use_context::<Signal<ConfigurationTemplate>>();
    let result = use_resource(move || async move {
        let api = configuration().about_me_api;
        let aboutMe: AboutMePage
        = Cache::fetch_or_cache(api.as_str(),|| async{
            let mut aboutMe;
            if api.is_empty() {
                 aboutMe = serde_json::from_str::<AboutMePage>(include_str!(
                    "../../defaultConfig/aboutMe.json"
                ))
                    .unwrap()
            } else {
                aboutMe= fetch_and_decrypt(api.as_str()).await.unwrap();
                aboutMe.image1 = parse_to_data_url(aboutMe.image1.clone(),IMAGE).await;
                aboutMe.image2 = parse_to_data_url(aboutMe.image2.clone(),IMAGE).await;
                join_all(aboutMe.hobby.iter_mut().map(|a|async{
                    a.image = parse_to_data_url(a.image.clone(),IMAGE).await
                })).await;
                aboutMe.music_art_1 = parse_to_data_url(aboutMe.music_art_1.clone(),MP4).await;
                aboutMe.music_art_2 = parse_to_data_url(aboutMe.music_art_2.clone(),MP4).await;
            }
            serde_json::to_vec(&aboutMe).unwrap()
        }).await.unwrap();
        aboutMe
    });
    match &*result.value().read() {
        Some(ab) => {
            rsx! {
                div { class: "dark:bg-gray-950",
                    AboutMe {
                        title: ab.about_me_title.as_str(),
                        subtitle: ab.about_me_motto.as_str(),
                        image1: ab.image1.as_str(),
                        image2: ab.image2.as_str()
                    }
                    Quote { description_quote: ab.description.as_str() }
                    Experience {
                        experiences: ab.experience
                            .iter()
                            .map(|e| ExperienceContent {
                                start_time: "Since ".to_string() + &*e.time_of_year.to_string(),
                                title: e.title.clone(),
                                keywords: e.keywords.clone(),
                                description: e.description.clone(),
                            })
                            .collect()
                    }
                    Skill {
                        skill_content: ab.skill_radar
                            .iter()
                            .map(|r| {
                                SkillContent {
                                    skill_name: r.skill_name.clone(),
                                    description: r.skill_description.clone(),
                                    skill_content: r
                                        .dimensions
                                        .iter()
                                        .map(|d| {
                                            RadarContent {
                                                name: d.name.clone(),
                                                value: d.value,
                                            }
                                        })
                                        .collect(),
                                }
                            })
                            .collect()
                    }
                    Hobby {
                        hobbys: ab.hobby
                            .iter()
                            .map(|h| HobbyContent {
                                image_url: h.image.to_string(),
                                title: h.title.clone(),
                                description: h.description.clone(),
                            })
                            .collect()
                    }
                    MusicAndArt { video_url: ab.music_art_1.clone(), video2_url: ab.music_art_2.clone() }
                }
            }
        }
        _ => {
            rsx! { Loading {} }
        }
    }
}
