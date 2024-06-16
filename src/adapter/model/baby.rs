use sqlx::FromRow;
use chrono::{DateTime, Local, NaiveDateTime};

use crate::domain::model::baby::{Baby, NewBaby, RenewBaby};

#[derive(FromRow)]
pub struct BabyTable {
    pub id: String,
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

pub struct NewBabyRecord {
    pub id: String,
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}

pub struct RenewBabyRecord {
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}

impl TryFrom<BabyTable> for Baby {
    type Error = anyhow::Error;

    fn try_from(bt: BabyTable) -> Result<Self, Self::Error> {
        Ok(
            Baby {
                id: bt.id.try_into()?,
                name: bt.name,
                birthday: bt.birthday,
                sex: bt.sex,
                create_date: bt.create_date,
                update_date: bt.update_date,
            }
        )
    }
}
impl TryFrom<NewBaby> for NewBabyRecord {
    type Error = anyhow::Error;

    fn try_from(nb: NewBaby) -> Result<Self, Self::Error> {
        Ok(
            NewBabyRecord {
                id: nb.id.value.to_string(),
                name: nb.name,
                birthday: nb.birthday,
                sex: nb.sex
            }
        )
    }
}

impl TryFrom<RenewBaby> for RenewBabyRecord {
    type Error = anyhow::Error;

    fn try_from(rb: RenewBaby) -> Result<Self, Self::Error> {
        Ok(
            RenewBabyRecord {
                name: rb.name,
                birthday: rb.birthday,
                sex: rb.sex,
            }
        )
    }
}