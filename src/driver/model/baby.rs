use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, NaiveDateTime};
use validator::Validate;

use crate::{app::model::baby::{CreateBaby, UpdateBaby}, domain::model::baby::Baby};

#[derive(Debug, Deserialize, Validate)]
pub struct JsonCreateBaby {
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    pub birthday: String,
    pub sex: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonUpdateBaby {
    pub name: String,
    pub birthday: String,
    pub sex: String,
}

#[derive(Serialize)]
pub struct JsonBabyView {
    pub id: String,
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

impl From<JsonCreateBaby> for CreateBaby {
    fn from(jcb: JsonCreateBaby) -> Self {
        CreateBaby {
            name: jcb.name,
            birthday: NaiveDateTime::parse_from_str(&jcb.birthday, "%Y-%m-%d %H:%M:%S").unwrap(),
            sex: jcb.sex,
        }
    }
}

impl From<JsonUpdateBaby> for UpdateBaby {
    fn from(jub: JsonUpdateBaby) -> Self {
        UpdateBaby {
            name: jub.name,
            birthday: NaiveDateTime::parse_from_str(&jub.birthday, "%Y-%m-%d %H:%M:%S").unwrap(),
            sex: jub.sex,
        } 
    }
}

impl From<Baby> for JsonBabyView {
    fn from(b: Baby) -> Self {
        JsonBabyView {
            id: b.id.value.to_string(),
            name: b.name,
            birthday: b.birthday,
            sex: b.sex,
            create_date: b.create_date,
            update_date: b.update_date,
        }
    }
}