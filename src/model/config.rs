use crate::model::Article::Article;
use dioxus::prelude::Props;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    pub encrypted_key: String,
    pub configuration_fetching_url: String,
    pub config_template: ConfigurationTemplate,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationTemplate {
    pub calendar_url: String,
    pub welcome_page: WelcomePage,
    pub articles_page: ArticlePage,
    pub about_me_page: AboutMePage,
    pub timer: TimerPage,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomePage {
    pub title: String,
    pub subtitle_description: Vec<String>,
    pub animation_video_light_url: String,
    pub animation_video_dark_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ArticlePage {
    pub all_article_url: String,
    pub one_article_url: String,
    pub first_article: Article,
    pub second_article: Article,
    pub third_article: Article,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AboutMePage {
    pub about_me_intro_url: String,
    pub about_me_title: String,
    pub about_me_description: String,
    pub about_me_video_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TimerPage {
    pub timer_url: String,
    pub timer_intro: Vec<TimerContent>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
pub struct TimerContent {
    pub image: String,
    pub content: String,
}
