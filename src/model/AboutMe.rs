use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AboutMeContent {
    pub about_me: AboutMe,
    pub stage: Vec<MyStage>,
}
#[derive(Serialize, Deserialize)]
pub struct AboutMe {
    pub about_me_image: String,
    pub about_me_content: String,
}

#[derive(Serialize, Deserialize)]
pub struct MyStage {
    pub legend_title: String,
    pub stages: Vec<Stage>,
}
#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub category: String,
    pub value: u32,
}
