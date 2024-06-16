use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, NaiveDateTime};
use validator::Validate;

use crate::app::model::childcare::{CreateChildCare, UpdateChildCare};
use crate::domain::model::childcare::ChildCare;

#[derive(Debug, Deserialize, Validate)]
pub struct JsonCreateChildCare {
    pub baby_id: String,
    pub day: String,
    pub action_type: String,
    #[validate(range(max = 255))]
    pub quantity: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct JsonUpdateChildCare {
    pub day: String,
    pub action_type: String,
    pub quantity: Option<u8>,
}

#[derive(Serialize)]
pub struct JsonChildCareView {
    pub id: String,
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

impl From<JsonCreateChildCare> for CreateChildCare {
    fn from(jcc: JsonCreateChildCare) -> Self {
        CreateChildCare {
            baby_id: jcc.baby_id,
            day: NaiveDateTime::parse_from_str(&jcc.day, "%Y-%m-%d %H:%M:%S").unwrap(),
            action_type: jcc.action_type,
            quantity: jcc.quantity.map_or(0, |q| q),
        }
    }
}

impl From<JsonUpdateChildCare> for UpdateChildCare {
    fn from(juc: JsonUpdateChildCare) -> Self {
        UpdateChildCare {
           day: NaiveDateTime::parse_from_str(&juc.day, "%Y-%m-%d %H:%M:%S").unwrap(),
           action_type: juc.action_type,
           quantity: juc.quantity.map_or(0, |q| q), 
        } 
    }
}

impl From<ChildCare> for JsonChildCareView {
    fn from(cc: ChildCare) -> Self {
        JsonChildCareView {
            id: cc.id.value.to_string(),
            baby_id: cc.baby_id,
            day: cc.day,
            action_type: cc.action_type,
            quantity: cc.quantity,
            create_date: cc.create_date,
            update_date: cc.update_date,
        }
    }
}