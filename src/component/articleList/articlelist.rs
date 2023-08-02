#![allow(non_snake_case)]

use charming::element::{LineStyle, ScaleLimit};
use charming::series::{Graph, GraphData, GraphLayout};
use charming::{Chart, WasmRenderer};
use dioxus::prelude::*;
use dioxus_router::{Link, Router};

#[derive(Props, PartialEq)]
pub struct ArticlesListContext {
    url: String,
}

pub fn ArticleList(cx: Scope<ArticlesListContext>) -> Element {
    gloo_utils::window().scroll_with_x_and_y(0f64, 0f64);
    cx.render(
        rsx!(
            div { id: "article_list",
                class:"bg-gray-200 w-screen min-h-[2000px] relative",
                div { id: "article_list_box",
                    class:"w-[90%] h-[1800px] mx-auto relative top-48",
                 div{ id: "article_list_sidebar",
                     class:"h-[1600px] w-[30%] black bg-gray-100 absolute right-4 top-12 shadow-lg hidden md:block",
                        div{
                            id:"article_list_sidebar_key_words",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "key words:"
                            }
                            div{
                                class:"w-11/12 h-[90%]"
                            }
                        }
                        div{
                            id:"article_list_sidebar_tag",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "tag:"
                            }
                            ul{
                                class:"w-11/12 h-4/5 p-8",
                                li{
                                    class:"m-3 inline-block hover:underline",
                                    "java(10)"
                                }
                            }
                        }
                        div{
                            id:"article_list_sidebar_recommend",
                            class:"w-11/12 h-96 mx-auto my-10",
                            span{
                                class:"text-gray-800 text-xl font-sans",
                                "recommend:"
                            }
                            ul{
                                class:"w-11/12 h-4/5 p-8",
                                li{
                                    class:"my-1 hover:underline",
                                    "Java 的前世今生"
                                }

                            }
                        }
                }
                    ul{
                        id:"article_list_content",
                        class:"h-[1600px] w-[90%]  md:w-[65%] absolute left-4 top-12 p-5 flex flex-col justify-start gap-5",
                        Link{
                            to:"/article/adf"
                           class:"h-[200px] w-full border-stone-900 rounded-2xl shadow-xl relative" ,
                            img{
                                src:"https://images.unsplash.com/photo-1689956533687-48a209951899?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2069&q=80",
                                alt:"",
                                class:"w-full h-full rounded-2xl object-cover blur-[1px] contrast-75 brightness-90"
                            }
                            span{
                                class:"w-[90%] h-24 block absolute top-1/2 left-4 text-gray-50",
                                span{
                                    class:"text-3xl font-sans block",
                                    "This is the Title"
                                }
                                span{
                                    class:"text-md font-mono block text-gray-200",
                                    "this is the description"
                                }
                            }
                        }

                    }
                }
            }
        )
    )
}
