use coi::{Container, Error, Inject, Provide};
use mobc_postgres::{
    mobc::{Connection, Error as MobcError, Manager, Pool},
    PgConnectionManager,
};
use std::sync::Arc;

pub struct PostgresPool<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPool<T>
where
    PgConnectionManager<T>: Manager,
{
    pub async fn get(
        &self,
    ) -> Result<
        Connection<PgConnectionManager<T>>,
        MobcError<<PgConnectionManager<T> as Manager>::Error>,
    > {
        self.0.get().await
    }
}

impl<T> Inject for PostgresPool<T> where PgConnectionManager<T>: Manager {}

pub struct PostgresPoolProvider<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPoolProvider<T>
where
    PgConnectionManager<T>: Manager,
{
    pub fn new(pool: Pool<PgConnectionManager<T>>) -> Self {
        Self(pool)
    }
}

impl<T> Provide for PostgresPoolProvider<T>
where
    PgConnectionManager<T>: Manager,
{
    type Output = PostgresPool<T>;

    fn provide(&self, _: &Container) -> Result<Arc<PostgresPool<T>>, Error> {
        Ok(Arc::new(PostgresPool(self.0.clone())))
    }

    #[cfg(feature = "debug")]
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
}
