use std::convert::Infallible;

use anyhow::Error;
use async_trait::async_trait;
use taskboard_core_lib::{uuid::Uuid, Task};
use warp::Filter;

pub mod es;

#[async_trait]
pub trait TaskStore {
    async fn ping(&self) -> Result<(), Error>;
    async fn fetch_tasks(&self, project_id: &Uuid) -> Result<Vec<Task>, Error>;
    async fn get(&self, project_id: &Uuid, task_number: usize) -> Result<Option<Task>, Error>;
    async fn persist(&self, project_id: &Uuid, task: &Task) -> Result<(), Error>;
    async fn delete(&self, project_id: &Uuid) -> Result<(), Error>;
}

pub fn with_store(
    store: impl TaskStore + Clone + Send,
) -> impl Filter<Extract = (impl TaskStore,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}
