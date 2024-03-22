use dioxus::prelude::*;

#[component]
pub fn ArticleList() -> Element {
    // let configuration = consume_context::<Signal<ConfigurationTemplate>>();
    // let tags_filter = use_signal(||HashSet::<String>::new());
    // let mut articles = use_signal(||Vec::<Article>::new());
    // let content = use_resource(move || async move{
    //     let mut tags = HashMap::new();
    //     let mut keywords = HashMap::new();
    //     let api = configuration().article_api;
    //     if api.is_empty() {
    //         articles.set(serde_json::from_str::<Vec<Article>>(include_str!("../../defaultConfig/article.json")).unwrap());
    //     }else{
    //         articles.set(fetch_and_decrypt::<Vec<Article>>(&api).await);
    //     }
    //     join_all(articles.iter_mut().map(|a| async {
    //         a.image = parse_to_data_url(a.image.clone(),IMAGE).await;
    //     })).await;
    //     articles.iter().for_each(|a|{
    //         a.tags.iter().for_each(|t|{
    //             if tags.contains_key(t){
    //                 *tags.get_mut(t).unwrap() +=1;
    //             }else{
    //                 tags.insert(t.clone(),1);
    //             }
    //         });
    //         a.keywords.iter().for_each(|k|{
    //             if keywords.contains_key(k){
    //                 *keywords.get_mut(k).unwrap() +=1;
    //             }else{
    //                 keywords.insert(k.clone(),1);
    //             }
    //         });
    //     });
    //     (articles,tags,keywords)
    // });
    //     match &*content.value().read() {
    //         None => {
    //             rsx!( Loading {} ) }
    //         Some((article,tags,keywords)) => {
    //             rsx! {
    //                 div {
    //                     id: "article_list",
    //                     class: "w-screen h-screen min-h-[400px] relative",
    //                     div {
    //                         id: "article_title",
    //                         class:"relative top-48 mx-auto w-1/2 text-4xl font-semibold capitalize text-center md:hidden",
    //                         "article"
    //                     }
    //                     div {
    //                         id: "article_list_box",
    //                         class: "w-[90%] h-auto mx-auto relative top-48",
    //                         div {
    //                             id: "article_list_sidebar",
    //                             class: "absolute h-auto w-[30%] bg-white right-4 top-12 shadow-[-2px_4px_4px_2px_rgba(0,0,0,0.25)] hidden md:flex md:flex-col",
    //                             div {
    //                                 id: "article_list_sidebar_tag",
    //                                 class: "w-11/12 mx-auto my-10 flex-1",
    //                                 div {
    //                                     img {
    //                                         class: "inline-block w-8 h-8 my-2.5 mr-[2%]",
    //                                         src: "/static/tag.svg"
    //                                     }
    //                                     for t in tags_filter(){
    //                                             div{
    //                                                 id:"tags_block",
    //                                                 class: "inline-flex h-8 mx-2 my-2 rounded-sm shadow-[0_4px_10px_0_rgba(0,0,0,0.25)] flex-row items-center cursor-pointer",
    //                                                 span{
    //                                                     class:"text-sm font-normal align-middle px-2 flex-1",
    //                                                     "{t}"
    //                                                 }
    //                                                 img{
    //                                                     class: "w-3.5 h-3.5 flex-1 pr-1 ",
    //                                                     src: "/static/close_black.svg",
    //                                                     onclick:  |_e|{
    //                                                         //todo: clear from tag filter, now its hard remove from map iterator
    //                                                     }
    //                                     
    //                                                 }
    //                                             }
    //                                     }
    //                                 }
    //                                 ul { class: "w-11/12 h-4/5 p-8",
    //                                     // tags.iter().map(|t|{
    //                                     //     rsx!(
    //                                     // li { class: "m-3 inline-block hover:underline cursor-pointer",
    //                                     //             onclick:|_e|{
    //                                     //                 tags_filter.with_mut(|tf|tf.take(t.0).or_else(||{
    //                                     //                     tf.insert(t.0.clone());
    //                                     //                     Some("Ok".to_string())
    //                                     //                 })
    //                                     //                 );
    //                                     //             },
    //                                     //             span{
    //                                     //                 class: "text-base whitespace-pre-wrap",
    //                                     //             "●  {t.0}({t.1})"
    //                                     //             }
    //                                     // }
    //                                     //     )
    //                                     // })
    //                                 }
    //                             }
    //                             div {
    //                                 id: "article_list_sidebar_key_words",
    //                                 class: "w-11/12 h-[20vw] mx-auto my-10 flex-1",
    //                                 img {
    //                                     class: "inline-block w-8 h-8 my-2 mr-[2%]",
    //                                     src: "/static/keywords.svg"
    //                                 }
    //                                 div {
    //                                     id: "article_list_keys",
    //                                     class: "w-11/12 h-[90%]",
    //                                     onmounted: |_e| {
    //                                         eval(&word_cloud_maker(keywords));
    //                                     }
    //                                 }
    //                             }
    //                             div {
    //                                 id: "article_list_sidebar_recommend",
    //                                 class: "w-11/12 mx-auto my-10 flex-1",
    //                                 img {
    //                                     class: "inline-block w-8 h-8 my-2 mr-[2%]",
    //                                     src: "/static/editor.svg"
    //                                 }
    //                                 ul { class: "w-11/12 h-4/5 p-4 px-20",
    //                                     // article.iter().take(10).map(|a|{
    //                                     //     rsx!{
    //                                     //     li { class: "my-3 underline",
    //                                     //             Link{
    //                                     //                 to: Route::Article {id : a.id.clone()},
    //                                     //                 "{a.title}"
    //                                     //             }
    //                                     //         }
    //                                     //     }
    //                                     // })
    //                                 }
    //                             }
    //                         }
    //                         ul {
    //                             id: "article_list_content",
    //                             class: "absolute h-[1600px] w-[90%]  md:w-[65%] left-4 top-12 p-5 flex flex-col justify-start gap-5",
    //                             // article.iter()
    //                             // .filter(|a|{
    //                             //     let mut check_tags = true;
    //                             //     tags_filter.with(|tags_filter|{
    //                             //         for t in tags_filter{
    //                             //             if !a.tags.contains(&t){
    //                             //                 check_tags = false;
    //                             //             }
    //                             //         }
    //                             //     });
    //                             //     check_tags
    //                             // })
    //                             // .map(|a|{
    //                             //     rsx!{
    //                             // Link{
    //                             // to:"/article/{a.id}",
    //                             // class:"relative w-full h-44 border border-black/10 rounded-2xl shadow-[0_4px_4px_0_rgba(0,0,0,0.25)]" ,
    //                             // img{
    //                             // src:"{a.image}",
    //                             // alt:"",
    //                             // class:"w-full h-full rounded-2xl object-cover contrast-50 brightness-50 saturate-50"
    //                             // }
    //                             // span{
    //                             // class:"w-[90%] h-20 flex flex-col absolute top-1/2 left-4 text-white text-ellipsis overflow-hidden",
    //                             // span{
    //                             //     class:"text-3xl block",
    //                             //     "{a.title}"
    //                             // }
    //                             // span{
    //                             //     class:"text-md block text-gray-300 flex-1 ",
    //                             //                 "{a.introduction}"
    //                             // }
    //                             // }
    //                             // }}
    //                             // }),
    //                             // todo: implement page feature
    //                             div {
    //                                 id: "article_list_table",
    //                                 class: "relative w-full h-8 my-16 flex rounded-[30px] shadow-[0_-4px_4px_0_rgba(0,0,0,0.25)] justify-center text-lg",
    //                                 div { class: "mx-2 text-gray-500", "1" }
    //                                 div { class: "mx-2 text-black", "2" }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    rsx!()
}
