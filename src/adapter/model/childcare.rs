use sqlx::FromRow;
use chrono::{DateTime, Local, NaiveDateTime};

use crate::domain::model::childcare::{ChildCare, NewChildCare, RenewChildCare};

#[derive(FromRow)]
pub struct ChildCareTable {
    pub id: String,
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

pub struct NewChildCareRecord {
    pub id: String,
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}

pub struct RenewChildCareRecord {
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}

impl TryFrom<ChildCareTable> for ChildCare {
    type Error = anyhow::Error;

    fn try_from(cct: ChildCareTable) -> Result<Self, Self::Error> {
        Ok(
            ChildCare {
                id: cct.id.try_into()?,
                baby_id: cct.baby_id,
                day: cct.day,
                action_type: cct.action_type,
                quantity: cct.quantity,
                create_date: cct.create_date,
                update_date: cct.update_date,
            }
        )
    }
}

impl TryFrom<NewChildCare> for NewChildCareRecord {
    type Error = anyhow::Error;

    fn try_from(ncc: NewChildCare) -> Result<Self, Self::Error> {
        Ok(
            NewChildCareRecord {
                id: ncc.id.value.to_string(),
                baby_id: ncc.baby_id,
                day: ncc.day,
                action_type: ncc.action_type,
                quantity: ncc.quantity,
            }
        )
    }
}

impl TryFrom<RenewChildCare> for RenewChildCareRecord {
    type Error = anyhow::Error;

    fn try_from(rcc: RenewChildCare) -> Result<Self, Self::Error> {
        Ok(
            RenewChildCareRecord {
                day: rcc.day,
                action_type: rcc.action_type,
                quantity: rcc.quantity,
            }
        )
    }
}