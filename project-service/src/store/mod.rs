use std::convert::Infallible;

use anyhow::Error;
use async_trait::async_trait;
use taskboard_core_lib::{uuid::Uuid, Project};
use warp::Filter;

pub mod es;

#[async_trait]
pub trait ProjectStore {
    async fn ping(&self) -> Result<(), Error>;
    async fn get(&self, project_id: &Uuid) -> Result<Option<Project>, Error>;
    async fn search(&self, name: &str) -> Result<Vec<Project>, Error>;
    async fn persist(&self, project: &Project) -> Result<(), Error>;
    async fn delete(&self, project_id: &Uuid) -> Result<(), Error>;
}

pub fn with_store(
    store: impl ProjectStore + Clone + Send,
) -> impl Filter<Extract = (impl ProjectStore,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}
