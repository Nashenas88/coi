use crate::{
    models::data::Data,
    postgres::PostgresPool,
};
use async_trait::async_trait;
use coi::Inject;
use mobc_postgres::tokio_postgres::NoTls;
use serde::Deserialize;
use serde_tokio_postgres::from_row;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DbData {
    id: i64,
    name: String,
}

impl Into<Data> for DbData {
    fn into(self) -> Data {
        Data {
            id: self.id,
            name: self.name,
        }
    }
}

#[async_trait]
pub trait IRepository: Inject {
    async fn get(&self, id: i64) -> Result<DbData, String>;
    async fn get_all(&self) -> Result<Vec<DbData>, String>;
}

#[derive(Inject)]
#[provides(pub dyn IRepository with Repository::new(pool))]
struct Repository {
    #[inject]
    pool: Arc<PostgresPool<NoTls>>,
}

#[async_trait]
impl IRepository for Repository {
    async fn get(&self, id: i64) -> Result<DbData, String> {
        let client = self.pool.get().await.map_err(|e| format!("{:?}", e))?;
        let statement = client
            .prepare("SELECT id, name FROM data WHERE id=$1::BIGINT")
            .await
            .map_err(|e| format!("{:?}", e))?;
        let row = client
            .query_one(&statement, &[&id])
            .await
            .map_err(|e| format!("{:?}", e))?;
        let data = from_row::<DbData>(row).map_err(|e| format!("{}", e))?;
        Ok(data.into())
    }

    async fn get_all(&self) -> Result<Vec<DbData>, String> {
        let client = self.pool.get().await.map_err(|e| format!("{:?}", e))?;
        let statement = client
            .prepare("SELECT id, name FROM data LIMIT 50")
            .await
            .map_err(|e| format!("{:?}", e))?;
        let rows = client
            .query(&statement, &[])
            .await
            .map_err(|e| format!("{:?}", e))?;
        let names = rows
            .into_iter()
            .map(|r| {
                from_row::<DbData>(r)
                    .map(Into::into)
                    .map_err(|e| format!("{:?}", e))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(names)
    }
}

impl Repository {
    fn new(pool: Arc<PostgresPool<NoTls>>) -> Self {
        Self { pool }
    }
}
