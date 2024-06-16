use anyhow::Result;
use async_trait::async_trait;

use super::DatabaseRepositoryImpl;
use crate::adapter::model::baby::{BabyTable, NewBabyRecord, RenewBabyRecord};
use crate::domain::{
    model::{
        Id,
        baby::{Baby, NewBaby, RenewBaby}
    },
    repository::baby::BabyRepository
};

#[async_trait]
impl BabyRepository for DatabaseRepositoryImpl<Baby> {
    async fn find(&self, id: &Id<Baby>) -> Result<Option<Baby>> {
        let pool = self.pool.0.clone();

        let baby_table = sqlx::query_as::<_, BabyTable>(
            r#"
            SELECT * FROM babies WHERE id = ?;
            "#
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        baby_table
            .map_or(Ok(None), |data| Ok(Some(data.try_into()?)))

    }

    async fn register(&self, new_baby: NewBaby) -> Result<()> {
        let pool = self.pool.0.clone();

        let new_baby_record: NewBabyRecord = new_baby.try_into()?;

        sqlx::query(
            r#"
            INSERT INTO babies (id, name, birthday, sex) VALUES(?, ?, ?, ?);
            "#,
        )
        .bind(new_baby_record.id)
        .bind(new_baby_record.name)
        .bind(new_baby_record.birthday)
        .bind(new_baby_record.sex)
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: &Id<Baby>, renew_baby: RenewBaby) -> Result<()> {
        let pool = self.pool.0.clone();

        let renew_baby_record: RenewBabyRecord = renew_baby.try_into()?;

        sqlx::query(
            r#"
            UPDATE babies SET name = ?, birthday = ?, sex = ? WHERE id = ?;
            "#,
        )
        .bind(renew_baby_record.name)
        .bind(renew_baby_record.birthday)
        .bind(renew_baby_record.sex)
        .bind(id.value.to_string())
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Id<Baby>) -> Result<()> {
        let pool = self.pool.0.clone();

        sqlx::query(
            r#"
                DELETE FROM babies WHERE id = ?
            "#,
        )
        .bind(id.value.to_string())
        .execute(&*pool)
        .await?;

        Ok(())
    }
}