use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AboutMePage {
    pub about_me_title:String,
    pub about_me_motto:String,
    pub image1:String,
    pub image2:String,
    pub description:String,
    pub experience:Vec<Experience>,
    pub skill_radar:Vec<Radar>,
    pub hobby: Vec<Hobby>,
    pub music_art_1: String,
    pub music_art_2: String,
}
#[derive(Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub description: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Radar {
    pub skill_name: String,
    pub skill_description: String,
    pub dimensions: Vec<Dimensions>,
}
#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub name: String,
    pub value: u32,
}
#[derive(Serialize, Deserialize)]
pub struct Experience {
    pub time_of_year: u32,
    pub title: String,
    pub keywords: Vec<String>,
    pub description:String
}
#[derive(Serialize, Deserialize)]
pub struct Hobby {
    pub title: String,
    pub description:String,
    pub image:String
}
