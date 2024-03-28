use dioxus::prelude::*;
use futures::future::join_all;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::component::aboutMeContent::aboutme::AboutMe;
use crate::component::aboutMeContent::experience::Experience;
use crate::component::aboutMeContent::hobby::Hobby;
use crate::component::aboutMeContent::musicArt::MusicAndArt;
use crate::component::aboutMeContent::quote::Quote;
use crate::component::aboutMeContent::skill::Skill;
use crate::component::loading::Loading;
use crate::model;
use crate::model::ConfigurationTemplate;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

mod aboutme;
mod quote;
mod experience;
mod skill;
mod hobby;
mod musicArt;


#[derive(Clone,PartialEq)]
struct HobbyContent{
    image_url:String,
    title:String,
    description:String
}
#[derive(Clone,PartialEq)]
struct ExperienceContent{
    start_time: String,
    title:String,
    keywords: Vec<String>,
    description:String
}
#[derive(Clone,PartialEq)]
struct SkillContent{
    skill_name:String,
    description:String,
    skill_content: Vec<RadarContent>,
}
#[derive(Clone,PartialEq)]
struct RadarContent{
    name:String,
    value: u32
}
#[component]
pub fn AboutMeContent() -> Element {
    // let configuration = use_context::<Signal<ConfigurationTemplate>>();
    // let fetch = use_resource( |_|  async {
    //     let mut content :model::AboutMe::AboutMePage;
    //     let url = configuration.about_me_api;
    //     if url.is_empty() {
    //         content = serde_json::from_str(include_str!("../../defaultConfig/aboutMe.json")).unwrap()
    //     }else{
    //         content =  fetch_and_decrypt(url.as_str()).await;
    //
    //     }
    //     content.image1 = parse_to_data_url(content.image1.clone(),IMAGE).await;
    //     content.image2 = parse_to_data_url(content.image2.clone(),IMAGE).await;
    //     join_all(content.stage.iter_mut().flat_map(|stage| {
    //         stage.children.iter_mut()
    //     }).map(|child|async {
    //         child.image =
    //             parse_to_data_url(child.image.clone(),IMAGE).await;
    //     })).await;
    //     content
    // });
    // let github_username = configuration.contact.github_username.clone();
    rsx! {
        AboutMe {
            title: "Some test Title",
            subtitle: "this is a test descirtption adjaslkdjsalfhs lkjsdlkgsjdlkgjsd gshsacsdcdscndscsdlkcnsdc",
            image1: "",
            image2: ""
        }
        Quote { description_quote: "关于我是谁，这话怎么说呢？这里可以用英文，也是可以使用中文来描述自己， of course there are some different special effect can be added into the words, or over the sentence.比如像下面这样，重点的文字可以用不一样的/** 颜色 **/进行一个标注，当然标注的单词可以根据配置进行选择， but if you want more effect here, you should try to create a new merge request on github of this project." }
        Experience { experiences: Vec::new() }
        Skill { skill_content: Vec::new() }
        Hobby { hobbys: Vec::new() }
        MusicAndArt { video_url: "", video2_url: "" }
    }
}
