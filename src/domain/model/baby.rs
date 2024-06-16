use chrono::{DateTime, Local, NaiveDateTime};
use derive_new::new;

use crate::domain::model::Id;

#[derive(new)]
pub struct Baby {
    pub id: Id<Baby>,
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

#[derive(new)]
pub struct NewBaby {
    pub id: Id<Baby>,
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}

#[derive(new)]
pub struct RenewBaby {
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}