use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Calendar {
    pub year: u32,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Event {
    pub day: String,
    pub duration:Vec<Duration>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Duration {
    pub start_number: u32,
    pub end_number: u32,
}
