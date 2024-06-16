use chrono::{DateTime, Local, NaiveDateTime};
use derive_new::new;

use crate::domain::model::Id;

#[derive(new)]
pub struct ChildCare {
    pub id: Id<ChildCare>,
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

#[derive(new)]
pub struct NewChildCare {
    pub id: Id<ChildCare>,
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}

#[derive(new)]
pub struct RenewChildCare {
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}