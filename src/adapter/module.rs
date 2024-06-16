use crate::domain::{
    model::{baby::Baby, childcare::ChildCare},
    repository::{baby::BabyRepository, childcare::ChildCareRepository},
};

use super::repository::DatabaseRepositoryImpl;
use crate::adapter::database::mysql::Db;

pub struct RepositoriesModule {
    baby_repository: DatabaseRepositoryImpl<Baby>,
    child_care_repository: DatabaseRepositoryImpl<ChildCare>,
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let baby_repository = DatabaseRepositoryImpl::new(db.clone());
        let child_care_repository = DatabaseRepositoryImpl::new(db.clone());

        Self {
            baby_repository,
            child_care_repository,
        }
    }
}

pub trait RepositoriesModuleExt {
   type BabyRepo: BabyRepository;
   type ChildCareRepo: ChildCareRepository;

   fn baby_repository(&self) -> &Self::BabyRepo;
   fn child_care_repository(&self) -> &Self::ChildCareRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type BabyRepo = DatabaseRepositoryImpl<Baby>;
    type ChildCareRepo =  DatabaseRepositoryImpl<ChildCare>;

    fn baby_repository(&self) -> &Self::BabyRepo {
        &self.baby_repository
    }

    fn child_care_repository(&self) -> &Self::ChildCareRepo {
        &self.child_care_repository
    }
}