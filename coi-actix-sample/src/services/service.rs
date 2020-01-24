use crate::{
    models::data::Data,
    repositories::repo::IRepository,
};
use async_trait::async_trait;
use coi::Inject;
use std::sync::Arc;

#[async_trait]
pub trait IService: Inject {
    async fn get(&self, id: i64) -> Result<Data, String>;
    async fn get_all(&self) -> Result<Vec<Data>, String>;
}

#[derive(Inject)]
#[provides(pub dyn IService with Service::new(repository))]
struct Service {
    #[inject]
    repository: Arc<dyn IRepository>,
}

#[async_trait]
impl IService for Service {
    async fn get(&self, id: i64) -> Result<Data, String> {
        self.repository.get(id).await.map(Into::into)
    }

    async fn get_all(&self) -> Result<Vec<Data>, String> {
        self.repository.get_all().await.map(|v| v.into_iter().map(Into::into).collect())
    }
}

impl Service {
    fn new(repository: Arc<dyn IRepository>) -> Self {
        Self { repository }
    }
}
