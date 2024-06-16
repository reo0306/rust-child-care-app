use chrono::NaiveDateTime;
use derive_new::new;

use crate::domain::model::{
    Id,
    childcare::{NewChildCare, RenewChildCare}
};

#[derive(new)]
pub struct CreateChildCare {
    pub baby_id: String,
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}

#[derive(new)]
pub struct UpdateChildCare {
    pub day: NaiveDateTime,
    pub action_type: String,
    pub quantity: u8,
}

impl TryFrom<CreateChildCare> for NewChildCare {
    type Error = anyhow::Error;

    fn try_from(ccc: CreateChildCare) -> anyhow::Result<Self, Self::Error> {
        let new_child_care_id = Id::gen();

        Ok(
            NewChildCare::new(
                new_child_care_id,
                ccc.baby_id,
                ccc.day,
                ccc.action_type,
                ccc.quantity,
            )
        )        
    }
}

impl TryFrom<UpdateChildCare> for RenewChildCare {
    type Error = anyhow::Error;

    fn try_from(ucc: UpdateChildCare) -> anyhow::Result<Self, Self::Error> {
        Ok(
            RenewChildCare::new(
                ucc.day,
                ucc.action_type,
                ucc.quantity,
            )
        ) 
    }
}