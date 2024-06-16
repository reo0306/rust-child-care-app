use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::{
    Id,
    baby::{Baby,NewBaby,RenewBaby,}
};

#[async_trait]
pub trait BabyRepository {
    async fn find(&self, id: &Id<Baby>) -> Result<Option<Baby>>;
    async fn register(&self, new_baby: NewBaby) -> Result<()>;
    async fn update(&self, id: &Id<Baby>, renew_baby: RenewBaby) -> Result<()>;
    async fn delete(&self, id: &Id<Baby>) -> Result<()>;
}