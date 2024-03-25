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
pub fn AboutMeContent() -> Element {
    rsx! {
        div { id: "about_me_content",
            class: "w-screen min-h-[800px]",
            div {
                id: "about_me_title",
                class: "absolute w-full min-h-[800px] top-18",
                div { id: "title",
                    class: "absolute w-96 h-28 top-28 left-4 flex flex-row items-center justify-center text-center font-semibold text-3xl ",
                    "Some Title About Me" }
                div { id: "description", class: "absolute w-96 h-28 top-56 left-4",
                    div { class: "relative w-12 border-t border-black" }
                    div { class: "w-full h-full my-2 text-sm font-normal text-left tracking-normal", "/* Some words" }
                }
            }
            div{
                id:"circle",
                class:"absolute w-96 h-40 top-[411px] left-96",
                div{
                    id:"circle_1",
                    class:"relative border w-40 h-40 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                div{
                    id:"",
                    class:"border w-32 h-32 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                div{
                    id:"",
                    class:"border w-24 h-24 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                div{
                    id:"",
                    class:"border w-16 h-16 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center",
                div{
                    id:"",
                    class:"border w-8 h-8 border-[rgb(96,24,123)] rounded-full flex justify-evenly items-center bg-[rgb(96,24,123)]",
                    }
                    }
                    }
                    }
                }
                div{
                    id:"circle_2",
                    class:"relative border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-full translate-x-2/3"
                }
                div{
                    id:"circle_3",
                    class:"relative border w-40 h-40 border-[rgb(37,71,49)] rounded-full -translate-y-[200%] translate-x-[120%]"
                }
            }
            div{
                id:"image_1",
                class:"absolute w-56 h-96 right-96 top-44 rounded-tl-[110px] shadow-[-9px_8px_25px_3px_rgba(0,0,0,0.25)] bg-pink-100",
                img{
                    src:""
                }

            }
            div{
                id:"image_2",
                class:"absolute w-72 h-96 right-14 top-44 rounded-tl-[149px] shadow-[6px_1px_8px_3px_rgba(0,0,0,0.25)] bg-blue-400",
                img{
                    src:"",
                }
            }
        },
        div{
            class:"absolute w-5/6 h-60 mx-auto bg-[rgb(249,248,248)] shadow-[0_4px_4px_0_rgba(0,0,0,0.25)] translate-x-1/2 right-1/2 -translate-y-2/3 flex",
            div{
                class:"w-1/3 flex items-center justify-center",
                div{
                    class:"w-36 h-36 bg-white rounded-full flex items-center justify-center",
                div{
                    class:"w-1/2 h-1/2 bg-black rounded-full flex items-center justify-center",
                div{
                    class:"border border-white w-1/2 translate-x-1/2 origin-left -rotate-45"
                }
                }
                }
            }
            div{
                class:"relative w-2/3 flex flex-col",
                div{
                class:"relative h-1/3",
                    div{
                        class:"absolute w-20 h-14 flex flex-row items-center justify-center bottom-0 text-lg font-medium",
                        "About Me"
                    }
                }
                div{
                class:"border border-b-black w-5/6",
                }
                div{
                class:"relative left-8 flex-1 w-5/6 my-6 font-normal text-left",
                    "关于我是谁，这话怎么说呢？这里可以用英文，也是可以使用中文来描述自己， of course there are some different special effect can be added into the words, or over the sentence.比如像下面这样，重点的文字可以用不一样的"
                }
            }

        }
        div{
            class:"bg-[rgb(195,201,195)] h-[1000px]"
        }
        div{
            class:"relative bg-transparent -top-48",
            div{
                class:"inline-block text-5xl font-medium m-12",
                "技能和技巧"
            }
            div{
                class:"w-5/6 bg-[rgb(27,46,77)] h-96 mx-auto flex",
                div{
                class:"w-1/2 flex items-center justify-center",
                    div{
                        class:"border-gray-950 border-2 w-56 h-56"
                    }
                }
                div{
                class:"w-1/2",
                    div{
                        class:"absolute top-52 -translate-x-12 text-4xl font-normal text-white",
                        "Skill Title"
                    }
                    div{
                        class:"absolute top-72 w-96 text-lg text-white font-light",
                        "Skill Description, descripte this skill is for what and the meaning of it, it should be long setence or just a few word."
                    }
                }
            }
        }
                div{
                    class:"h-[1000px]",
            div{
                class:"absolute w-5/6 border border-b-black right-0",
            }
                div{
                    class:"relative left-[20%] top-6 text-4xl ",
                    "Sport"
                }
            div{
                class:"relative h-96 bg-green-800 top-14",
            }
            div{
                class:"h-96 bg-blue-800",
            }
            div{
                class:"h-96 bg-yellow-800",
            }
            div{
                class:"h-96 bg-blue-800",
            }
                }
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
    // use special decoration to replace **/test/** in paragraph.
    // let special_content_wrapper_start =r##"<span class="before:block before:absolute before:-inset-1 before:-skew-y-3 before:bg-green-800 relative inline-block mx-3"><span class="relative text-white">"##;
    // let special_content_wrapper_end = r##"</span></span>"##;
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
    //         let aboutMeContent = aboutMe.description.replace(" **/", special_content_wrapper_end);
    //         let aboutMeContent = aboutMeContent.replace("/** ", &*special_content_wrapper_start);
    //         let github_states="https://github-readme-stats.vercel.app/api?username=".to_string()+&github_username+"&count_private=true&show_icons=true&title_color=ffffff&text_color=ffffff&icon_color=ffa502&bg_color=009432,9980FA,6F1E51";
    //     }
    // }
}
