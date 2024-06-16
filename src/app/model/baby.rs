use chrono::NaiveDateTime;
use derive_new::new;

use crate::domain::model::{
    Id,
    baby::{NewBaby, RenewBaby}
};

#[derive(new)]
pub struct CreateBaby {
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}

#[derive(new)]
pub struct UpdateBaby {
    pub name: String,
    pub birthday: NaiveDateTime,
    pub sex: String,
}

impl TryFrom<CreateBaby> for NewBaby {
    type Error = anyhow::Error;

    fn try_from(cb: CreateBaby) -> anyhow::Result<Self, Self::Error> {
        let new_baby_id = Id::gen();

        Ok(
            NewBaby::new(
                new_baby_id,
                cb.name,
                cb.birthday,
                cb.sex,
            )
        )
    }
}

impl TryFrom<UpdateBaby> for RenewBaby {
    type Error = anyhow::Error;

    fn try_from(ub: UpdateBaby) -> anyhow::Result<Self, Self::Error> {
        Ok(
            RenewBaby::new(
                ub.name,
                ub.birthday,
                ub.sex,
            )
        )
    }
}