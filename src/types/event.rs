use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    #[serde(rename="features")]
    pub event: Vec<EventDetails>,
}

pub fn gen_id() -> String {
   uuid::Uuid::new_v4().to_string()
}
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct EventId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventDetails {
    #[serde(default = "gen_id")]
    pub exposure_id: String,
    #[serde(rename="type")]
    pub place_name: String,
    pub address_line1: String,
    pub suburb: String,
    pub postcode: String,
    pub exposure_time_from: Option<NaiveDateTime>,
    pub exposure_time_to: Option<NaiveDateTime>,
    pub posted_time: Option<NaiveDateTime>,
}
