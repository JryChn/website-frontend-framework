use dioxus::prelude::Props;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,PartialEq,Props,Clone)]
pub struct Article{
    pub article_id:String,
    pub article_image:String,
    pub article_title:String,
    pub article_intro:String
}
#[derive(Serialize, Deserialize)]
pub struct ArticleList{
    pub article_list:Vec<Article>
}
#[derive(Serialize, Deserialize)]
pub struct ArticleContent{
    pub article_id:String,
    pub article_tag:String,
    pub article_image:String,
    pub article_title:String,
    pub article_intro:String,
    pub article_create_time:String,
    pub article_content:String
}
