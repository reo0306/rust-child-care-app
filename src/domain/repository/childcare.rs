use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::{
    Id,
    childcare::{ChildCare, NewChildCare, RenewChildCare,}
};

#[async_trait]
pub trait ChildCareRepository {
    async fn find(&self, id: &Id<ChildCare>) -> Result<Option<ChildCare>>;
    async fn register(&self, new_child_care: NewChildCare) -> Result<()>;
    async fn update(&self, id: &Id<ChildCare>, renew_child_care: RenewChildCare) -> Result<()>;
    async fn delete(&self, id: &Id<ChildCare>) -> Result<()>;
}