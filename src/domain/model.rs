use std::marker::PhantomData;
use derive_new::new;
use ulid::Ulid;

pub mod baby;
pub mod childcare;

#[derive(new)]
pub struct Id<T> {
    pub value: Ulid,
    _marker: PhantomData<T>
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Ulid::new())
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ulid::from_string(&value)
            .map(|id| Self::new(id))
            .map_err(|err| anyhow::anyhow!("{:?}", err))
    }
}