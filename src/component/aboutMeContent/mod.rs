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

#[component]
pub fn AboutMeContent() -> Element {
    rsx! {
        AboutMe{title:"Some test Title",subtitle:"this is a test descirtption adjaslkdjsalfhs lkjsdlkgsjdlkgjsd gshsacsdcdscndscsdlkcnsdc",image1:"",image2:""}
        Quote{description_quote:"关于我是谁，这话怎么说呢？这里可以用英文，也是可以使用中文来描述自己， of course there are some different special effect can be added into the words, or over the sentence.比如像下面这样，重点的文字可以用不一样的/** 颜色 **/进行一个标注，当然标注的单词可以根据配置进行选择， but if you want more effect here, you should try to create a new merge request on github of this project."}
        Experience{}
        Skill{}
        Hobby{}
        MusicAndArt{video_url:"",video2_url:""}
    }
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
    // let configuration = use_context::<Signal<ConfigurationTemplate>>();
    // let color_generator = ||{
    //     let mut value = Vec::new();
    //         let color_char = vec![
    //             "a","b","c","d","e","f","0","1","2","3","4","5","6","7","8","9"
    //         ];
    //     (1 ..7).for_each( |_| {
    //         value.push(
    //         color_char.choose(&mut thread_rng()).unwrap().to_string().clone());
    //     });
    //     value.into_iter().collect::<String>()
    // };
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
    // match fetch.value() {
    //     None =>
    //             render!{ Loading {} },
    //     Some(aboutMe)=> {
    //         let github_states="https://github-readme-stats.vercel.app/api?username=".to_string()+&github_username+"&count_private=true&show_icons=true&title_color=ffffff&text_color=ffffff&icon_color=ffa502&bg_color=009432,9980FA,6F1E51";
    //     }
    // }
}
