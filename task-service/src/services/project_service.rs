use std::convert::Infallible;

use anyhow::Error;
use async_trait::async_trait;
use reqwest::Client;
use taskboard_core_lib::uuid::Uuid;
use taskboard_core_lib::Project;
use warp::Filter;

#[async_trait]
pub trait IProjectService {
    async fn get_project(&self, id: &Uuid) -> Result<Project, Error>;
    async fn claim_task_number(&self, project_id: &Uuid) -> Result<usize, Error>;
}

pub fn with_project_service(
    project_service: impl IProjectService + Clone + Send,
) -> impl Filter<Extract = (impl IProjectService,), Error = Infallible> + Clone {
    warp::any().map(move || project_service.clone())
}

#[derive(Clone, Debug)]
pub struct ProjectService {
    client: Client,
    base_url: String,
}

impl ProjectService {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

#[async_trait]
impl IProjectService for ProjectService {
    async fn get_project(&self, id: &Uuid) -> Result<Project, Error> {
        Ok(self
            .client
            .get(&format!("{}/{}", self.base_url, id))
            .send()
            .await?
            .json::<Project>()
            .await?)
    }

    async fn claim_task_number(&self, project_id: &Uuid) -> Result<usize, Error> {
        Ok(self
            .client
            .post(&format!(
                "{}/{}/increment-counter",
                self.base_url, project_id,
            ))
            .send()
            .await?
            .json::<Project>()
            .await?
            .task_conter)
    }
}
