use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonConfig {
    pub encrypted_key:String,
    pub configuration_fetching_url:String,
    pub config_template: ConfigurationTemplate
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationTemplate{
    pub welcome_page:WelcomePage,
    pub articles_url:String,
    pub calendar_url:String,
    pub about_me_page:AboutMePage,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomePage{
    pub title:String,
    pub subtitle_description:Vec<String>,
    pub animation_video_light_url:String,
    pub animation_video_dark_url:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AboutMePage{
    about_me_intro_url:String,
    about_me_title:String,
    about_me_description:String,
    about_me_url:String

}