use serde::{Deserialize, Serialize};

pub mod AboutMe;
pub mod Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    pub encrypted_key: &'static str,
    pub configuration_fetching_url: &'static str
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ConfigurationTemplate {
    pub contact:Contact,
    pub welcome: Welcome,
    pub article_api: String,
    pub about_me_api: String,
    pub calendar_api:String,
    pub zone_api:String,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Contact {
    pub github_username: String,
    pub email: String,
    pub telegram_username: String,
}
#[derive(Serialize, Deserialize, Debug,PartialEq,Clone)]
pub struct Welcome {
    pub title: String,
    pub subtitle: String,
    pub animation_url:ModeUrl
}
#[derive(Serialize, Deserialize, Debug,PartialEq,Clone)]
pub struct ModeUrl {
    pub dark: String,
    pub light: String,
}