use std::env;

use anyhow::Error;
use async_trait::async_trait;
use elasticsearch::{
    auth::Credentials, cert::CertificateValidation, http::transport::SingleNodeConnectionPool,
    http::transport::TransportBuilder, http::Url, Elasticsearch, IndexParts, SearchParts,
};
use serde_json::json;
use taskboard_core_lib::{uuid::Uuid, Task};

use crate::store::TaskStore;

pub fn create_client() -> Result<Elasticsearch, Error> {
    let url = Url::parse(&env::var("ELASTIC_URL")?)?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials =
        Credentials::Basic(env::var("ELASTIC_USERNAME")?, env::var("ELASTIC_PASSWORD")?);
    let validation = CertificateValidation::None; // ECK uses self-signed cert

    let transport = TransportBuilder::new(conn_pool)
        .auth(credentials)
        .cert_validation(validation)
        .build()?;

    let client = Elasticsearch::new(transport);

    Ok(client)
}

#[async_trait]
impl TaskStore for Elasticsearch {
    async fn ping(&self) -> Result<(), Error> {
        let res = self.ping().send().await?;
        res.error_for_status_code()?;
        Ok(())
    }
    async fn fetch_task(&self, project_id: &Uuid, number: usize) -> Result<Option<Task>, Error> {
        let res = self
            .search(SearchParts::Index(&[&format!("tasks-{}", project_id)]))
            .body(json!({
                "query": {
                    "match": {
                        "_id": number,
                    },
                }
            }))
            .send()
            .await?;
        res.error_for_status_code_ref()?;
        let task = res.json::<Task>().await?;
        Ok(Some(task))
    }
    async fn fetch_tasks(&self, project_id: &Uuid) -> Result<Vec<Task>, Error> {
        let res = self
            .search(SearchParts::Index(&[&format!("task-{}", project_id)]))
            .body(json!({
                "query": {
                    "match_all": {}
                }
            }))
            .send()
            .await?;
        res.error_for_status_code_ref()?;
        let tasks = res.json::<Vec<Task>>().await?;
        Ok(tasks)
    }
    async fn persist(&self, project_id: &Uuid, task: &Task) -> Result<(), Error> {
        let res = self
            .index(IndexParts::IndexId(
                &format!("task-{}", project_id),
                &task.number.to_string(),
            ))
            .body(&task)
            .send()
            .await?;
        res.error_for_status_code()?;
        Ok(())
    }
}
