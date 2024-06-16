use std::marker::PhantomData;
use derive_new::new;

use super::database::mysql::Db;

pub mod baby;
pub mod childcare;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _maker: PhantomData<T>,
}