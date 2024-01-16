#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dioxus_router::components::Link;
use futures::future::join_all;

use crate::model::Article::Article;
use crate::model::config::ConfigurationTemplate;
use crate::Route;
use crate::utils::encryptedUtils::fetch_and_decrypt;
use crate::utils::netUtils::parse_to_data_url;
use crate::utils::resourceType::ResourceType::IMAGE;

#[inline_props]
pub fn ArticleList(cx: Scope) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64, 0f64);
    let configuration = use_shared_state::<ConfigurationTemplate>(cx).unwrap().read().clone();
    let tags_filter = use_ref(cx,||HashSet::<String>::new());
    let content = use_future(cx, (), |_| async {
        let mut articles;
        let mut tags = HashMap::new();
        let mut keywords = HashMap::new();
        let api = configuration.articles.article_api ;
        if api.is_empty() {
            articles = serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json")).unwrap();
        }else{
            articles = fetch_and_decrypt::<Vec<Article>>(&api).await;
        }
        join_all(articles.iter_mut().map(|a| async {
            a.image = parse_to_data_url(a.image.clone(),IMAGE).await;
        })).await;
        articles.iter().for_each(|a|{
            a.tags.iter().for_each(|t|{
                if tags.contains_key(t){
                    *tags.get_mut(t).unwrap() +=1;
                }else{
                    tags.insert(t.clone(),1);
                }
            });
            a.keywords.iter().for_each(|k|{
                if keywords.contains_key(k){
                    *keywords.get_mut(k).unwrap() +=1;
                }else{
                    keywords.insert(k.clone(),1);
                }
            });
        });
        let mut keywords_string = String::new();
        keywords.iter().for_each(|kv|{
            keywords_string.push_str("{name: '");
            keywords_string.push_str(kv.0);
            keywords_string.push_str("',value:");
            keywords_string.push_str(kv.1.to_string().as_str());
            keywords_string.push_str(",emphasis: {textStyle: {color: '#9d2933'}}},");
        });
        (articles,tags,keywords_string)
    });
    let js_function_eval =  use_eval(cx);
    cx.render(
        match content.value() {
            None => { rsx!( div { "Loading" } ) }
            Some((article,tags,keywords)) => {
                rsx!(
                    div {
                        id: "article_list",
                        class: "bg-gray-200 w-screen min-h-[2000px] relative",
                        div {
                            id: "article_list_box",
                            class: "w-[90%] h-[1800px] mx-auto relative top-48",
                            div {
                                id: "article_list_sidebar",
                                class: "h-[1600px] w-[30%] black bg-gray-100 absolute right-4 top-12 shadow-lg hidden md:block",
                                div {
                                    id: "article_list_sidebar_key_words",
                                    class: "w-11/12 h-96 mx-auto my-10",
                                    span { class: "text-gray-800 text-xl font-sans", "key words:" }
                                    div {
                                        id: "article_list_keys",
                                        class: "w-11/12 h-[90%]",
                                        onmounted: |_e| {
                                            js_function_eval(
                                                    &*(r#"
                                                                                 var option = {
                                                                                    tooltip: {},
                                                                                    series: [ {
                                                                                        type: 'wordCloud',
                                                                                        gridSize: 2,
                                                                                        sizeRange: [12, 150],
                                                                                        rotationRange: [-90, 90],
                                                                                        shape: 'pentagon',
                                                                                        drawOutOfBound: true,
                                                                                        textStyle: {
                                                                                            color: function () {
                                                                                                return 'rgb(' + [
                                                                                                    Math.round(Math.random() * 160),
                                                                                                    Math.round(Math.random() * 160),
                                                                                                    Math.round(Math.random() * 160)
                                                                                                ].join(',') + ')';
                                                                                            }
                                                                                        },
                                                                                        emphasis: {
                                                                                            textStyle: {
                                                                                                shadowBlur: 10,
                                                                                                shadowColor: '#333'
                                                                                            }
                                                                                        },
                                                                                        data: [ "#
                                                        .to_owned() + keywords
                                                        + r#"
                                                                                        ]
                                                                                    } ]
                                                                                 };
                                        
                                                                                var chart = echarts.init(document.getElementById('article_list_keys'));
                                                                                chart.setOption(option);
                                                                                window.onresize = chart.resize;
                                                                                "#),
                                                )
                                                .unwrap();
                                        }
                                    }
                                }
                                div {
                                    id: "article_list_sidebar_tag",
                                    class: "w-11/12 h-96 mx-auto my-10",
                                    span { class: "text-gray-800 text-xl font-sans",
                                        "tag:"
                                        tags_filter.read().iter().map(|t|{
                                            rsx!("{t}")
                                        })
                                    }
                                    ul { class: "w-11/12 h-4/5 p-8",
                                        tags.iter().map(|t|{
                                            rsx!(
                                        li { class: "m-3 inline-block hover:underline cursor-pointer",
                                                    onclick:|_e|{
                                                        tags_filter.with_mut(|tf|tf.take(t.0).or_else(||{
                                                            tf.insert(t.0.clone());
                                                            Some("done".to_string())
                                                        })
                                                        );
                                                    },
                                                    "{t.0}({t.1})"
                                        }
                                            )
                                        })
                                    }
                                }
                                div {
                                    id: "article_list_sidebar_recommend",
                                    class: "w-11/12 h-96 mx-auto my-10",
                                    span { class: "text-gray-800 text-xl font-sans", "recommend:" }
                                    ul { class: "w-11/12 h-4/5 p-8",
                                        article.iter().take(10).map(|a|{
                                            rsx!{
                                            li { class: "my-1 underline",
                                                    Link{
                                                        to: Route::Article {id : a.id.clone()},
                                                        "{a.title}"
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            }
                            ul {
                                id: "article_list_content",
                                class: "h-[1600px] w-[90%]  md:w-[65%] absolute left-4 top-12 p-5 flex flex-col justify-start gap-5",
                                article.iter()
                                .filter(|a|{
                                    let mut check_tags = true;
                                    tags_filter.with(|tags_filter|{
                                        for t in tags_filter{
                                            if !a.tags.contains(&t){
                                                check_tags = false;
                                            }
                                        }
                                    });
                                    check_tags
                                })
                                .map(|a|{
                                    rsx!{
                        Link{
                            to:"/article/{a.id}",
                           class:"h-[200px] w-full border-stone-900 rounded-2xl shadow-xl relative" ,
                            img{
                                src:"{a.image}",
                                alt:"",
                                class:"w-full h-full rounded-2xl object-cover blur-[1px] contrast-75 brightness-90"
                            }
                            span{
                                class:"w-[90%] h-20 flex flex-col absolute top-1/2 left-4 text-gray-50 text-ellipsis overflow-hidden",
                                span{
                                    class:"text-3xl font-sans block",
                                    "{a.title}"
                                }
                                span{
                                    class:"text-md font-mono block text-gray-200 flex-1 ",
                                                   "{a.introduction}"
                                }
                            }
                        }}
                                })
                            }
                        }
                    }
                )
            }
        }
    )
}
