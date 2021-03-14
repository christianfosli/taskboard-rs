use reqwest::Client;
use std::convert::Infallible;

use anyhow::Error;
use async_trait::async_trait;
use taskboard_core_lib::uuid::Uuid;
use warp::Filter;

#[async_trait]
pub trait ITaskService {
    async fn delete_tasks(&self, project_id: &Uuid) -> Result<(), Error>;
}

pub fn with_task_service(
    task_service: impl ITaskService + Clone + Send,
) -> impl Filter<Extract = (impl ITaskService,), Error = Infallible> + Clone {
    warp::any().map(move || task_service.clone())
}

#[derive(Clone, Debug)]
pub struct TaskService {
    client: Client,
    base_url: String,
}

impl TaskService {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

#[async_trait]
impl ITaskService for TaskService {
    async fn delete_tasks(&self, project_id: &Uuid) -> Result<(), Error> {
        let res = self
            .client
            .delete(&format!("{}/project-tasks/{}", self.base_url, project_id))
            .send()
            .await?;

        res.error_for_status()?;

        Ok(())
    }
}
