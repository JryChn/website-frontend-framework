use dioxus::prelude::*;
use futures::future::join_all;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::component::loading::Loading;
use crate::model;
use crate::model::ConfigurationTemplate;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

#[component]
pub fn AboutMeContent(cx: Scope) -> Element {
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
        content.image1 = parse_to_data_url(content.image1.clone(),IMAGE).await;
        content.image2 = parse_to_data_url(content.image2.clone(),IMAGE).await;
        join_all(content.stage.iter_mut().flat_map(|stage| {
            stage.children.iter_mut()
        }).map(|child|async {
            child.image =
                parse_to_data_url(child.image.clone(),IMAGE).await;
        })).await;
        content
    });
    let github_username = configuration.contact.github_username.clone();
    render! {
        div { id: "about_me_content", class: "w-screen min-h-[800px]",
            div {
                id: "about_me_title",
                class: "absolute w-full min-h-[800px] top-18 bg-pink-400",
                div { id: "title", class: "", "SomeTitleAboutMe" }
                div { id: "description", class: "",
                    div { class: "" }
                    div { class: "", "Some words" }
                }
            }
        }
    }
    // match fetch.value() {
    //     None =>
    //             render!{ Loading {} },
    //     Some(aboutMe)=> {
    //         let aboutMeContent = aboutMe.description.replace(" **/", special_content_wrapper_end);
    //         let aboutMeContent = aboutMeContent.replace("/** ", &*special_content_wrapper_start);
    //         let github_states="https://github-readme-stats.vercel.app/api?username=".to_string()+&github_username+"&count_private=true&show_icons=true&title_color=ffffff&text_color=ffffff&icon_color=ffa502&bg_color=009432,9980FA,6F1E51";
    //     }
    // }
}
