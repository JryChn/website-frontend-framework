use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq,  Clone)]
pub struct Article {
    pub id: String,
    pub image: String,
    pub title: String,
    pub introduction: String,
    pub tags: Vec<String>,
    pub keywords: Vec<String>,
    pub post_time:String,
    pub content:String
}
