use dioxus::prelude::Props;
use serde::{Deserialize, Serialize};

use crate::model::Article::Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    pub encrypted_key: &'static str,
    pub configuration_fetching_url: &'static str
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ConfigurationTemplate {
    pub contact:Contact,
    pub welcome: Welcome,
    pub articles: ArticleList,
    pub about_me: AboutMePage,
    pub timer: Timer,
    pub calendar:Calendar
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Contact {
    pub github_username: String,
    pub email: String,
    pub telegram: String,
}
#[derive(Serialize, Deserialize, Debug,Props,PartialEq,Clone)]
pub struct Welcome {
    pub title: String,
    pub subtitle: Vec<String>,
    pub animation_url:ModeUrl
}
#[derive(Serialize, Deserialize, Debug,Props,PartialEq,Clone)]
pub struct ModeUrl {
    pub dark: String,
    pub light: String,
}
#[derive(Serialize, Deserialize, Debug,Props,PartialEq,Clone)]
pub struct ArticleList {
    pub article_api:String,
    pub article_with_id_api:String,
    pub article:Vec<Article>,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct AboutMePage {
    pub api: String,
    pub name: String,
    pub description: String,
    pub video: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct Timer {
    pub api: String,
    pub posts: Vec<TimerPost>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct Calendar {
    pub api: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct TimerPost {
    pub image: String,
    pub content: String,
}
