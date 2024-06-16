use std::sync::Arc;

use crate::adapter::{
    database::mysql::Db,
    module::{RepositoriesModule, RepositoriesModuleExt},
};
use crate::app::usecase::{
    baby::BabyUseCase,
    childcare::ChildCareUseCase,
};

pub struct Modules {
    pub baby_use_case: BabyUseCase<RepositoriesModule>,
    pub child_care_use_case: ChildCareUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn baby_use_case(&self) -> &BabyUseCase<Self::RepositoriesModule>;
    fn child_care_use_case(&self) -> &ChildCareUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn baby_use_case(&self) -> &BabyUseCase<Self::RepositoriesModule> {
        &self.baby_use_case
    }

    fn child_care_use_case(&self) -> &ChildCareUseCase<Self::RepositoriesModule> {
        &self.child_care_use_case
    }
}

impl Modules {
    pub async fn new() -> Self {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let baby_use_case = BabyUseCase::new(repositories_module.clone());
        let child_care_use_case = ChildCareUseCase::new(repositories_module.clone());

        Self {
            baby_use_case,
            child_care_use_case,
        }
    }
}