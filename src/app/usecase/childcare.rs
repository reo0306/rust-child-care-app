use derive_new::new;
use std::sync::Arc;
use anyhow::Result;

use crate::domain::{
    model::childcare::ChildCare,
    repository::childcare::ChildCareRepository
};
use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::childcare::{CreateChildCare, UpdateChildCare};

#[derive(new)]
pub struct ChildCareUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl <R: RepositoriesModuleExt> ChildCareUseCase<R> {
    pub async fn find_child_care(&self, id: String) -> Result<Option<ChildCare>> {
        self.repositories
            .child_care_repository()
            .find(&id.try_into()?)
            .await
            .map(|child_care| child_care.map(|c| c.into()))
    }

    pub async fn create_child_care(&self, data: CreateChildCare) -> Result<()> {
        self.repositories.child_care_repository().register(data.try_into()?).await
    }

    pub async fn update_child_care(&self, id: String, data: UpdateChildCare) -> Result<()> {
        self.repositories
            .child_care_repository()
            .update(&id.try_into()?, data.try_into()?)
            .await
    }

    pub async fn delete_child_care(&self, id: String) -> Result<()> {
        self.repositories
            .child_care_repository()
            .delete(&id.try_into()?).await
    }
}