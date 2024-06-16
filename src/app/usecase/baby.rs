use derive_new::new;
use std::sync::Arc;
use anyhow::Result;

use crate::domain::{
    model::baby::Baby,
    repository::baby::BabyRepository
};
use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::baby::{CreateBaby, UpdateBaby};

#[derive(new)]
pub struct BabyUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl <R: RepositoriesModuleExt> BabyUseCase<R> {
    pub async fn find_baby(&self, id: String) -> Result<Option<Baby>> {
        self.repositories
            .baby_repository()
            .find(&id.try_into()?)
            .await
            .map(|baby| baby.map(|b| b.into()))
    }

    pub async fn create_baby(&self, data: CreateBaby) -> Result<()> {
        self.repositories.baby_repository().register(data.try_into()?).await

    }

    pub async fn update_baby(&self, id: String, data: UpdateBaby) -> Result<()> {
        self.repositories
            .baby_repository()
            .update(&id.try_into()?, data.try_into()?)
            .await
    }

    pub async fn delete_baby(&self, id: String) -> Result<()> {
        self.repositories.baby_repository().delete(&id.try_into()?).await
    }
}