use anyhow::Result;
use async_trait::async_trait;

use super::DatabaseRepositoryImpl;
use crate::adapter::model::childcare::{ChildCareTable, NewChildCareRecord, RenewChildCareRecord};
use crate::domain::{
    model::{
        Id,
        childcare::{ChildCare, NewChildCare, RenewChildCare}
    },
    repository::childcare::ChildCareRepository
};

#[async_trait]
impl ChildCareRepository for DatabaseRepositoryImpl<ChildCare> {
    async fn find(&self, id: &Id<ChildCare>) -> Result<Option<ChildCare>> {
        let pool = self.pool.0.clone();

        let child_care_table = sqlx::query_as::<_, ChildCareTable>(
            r#"
            SELECT * FROM child_cares WHERE id = ?;
            "#
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        child_care_table
            .map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
    }

    async fn register(&self, new_child_care: NewChildCare) -> Result<()> {
        let pool = self.pool.0.clone();

        let new_child_care_record: NewChildCareRecord = new_child_care.try_into()?;

        sqlx::query(
            r#"
                INSERT INTO child_cares (id, baby_id, day, action_type, quantity) VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(new_child_care_record.id)
        .bind(new_child_care_record.baby_id)
        .bind(new_child_care_record.day)
        .bind(new_child_care_record.action_type)
        .bind(new_child_care_record.quantity)
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn update(&self, id: &Id<ChildCare>, renew_child_care: RenewChildCare) -> Result<()> {
        let pool = self.pool.0.clone();

        let renew_child_care_record: RenewChildCareRecord = renew_child_care.try_into()?;

        sqlx::query(
            r#"
                UPDATE child_cares SET action_type = ?, day = ?, quantity = ? WHERE id = ?;
            "#,
        )
        .bind(renew_child_care_record.action_type)
        .bind(renew_child_care_record.day)
        .bind(renew_child_care_record.quantity)
        .bind(id.value.to_string())
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &Id<ChildCare>) -> Result<()> {
        let pool = self.pool.0.clone();

        sqlx::query(
            r#"
                DELETE FROM child_cares WHERE id = ?;
            "#,
        )
        .bind(id.value.to_string())
        .execute(&*pool)
        .await?;

        Ok(())
    }
}

