use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AboutMePage {
    pub image:String,
    pub description:String,
    pub stage: Vec<MyStage>,
    pub skill_radar:Radar
}
#[derive(Serialize, Deserialize)]
pub struct MyStage {
    pub category: String,
    pub value: Vec<Stage>,
}
#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Radar {
    pub languages: Vec<RadarContent>,
}
#[derive(Serialize, Deserialize)]
pub struct RadarContent {
    pub name: String,
    pub dimensions: Vec<Dimensions>,
}
#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub name: String,
    pub value: u32,
}
