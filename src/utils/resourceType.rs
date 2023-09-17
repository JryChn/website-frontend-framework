use crate::utils::resourceType::ResourceType::*;

pub enum ResourceType {
    MP4,
    IMAGE,
}

impl ResourceType {
    pub fn get_reources_type(&self) -> String{
        match self {
            MP4 => String::from("video/mp4"),
            IMAGE=> String::from("image")
        }
    }
}