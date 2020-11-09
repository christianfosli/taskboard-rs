use std::env;

use anyhow::{anyhow, Error};
use async_trait::async_trait;
use elasticsearch::{
    auth::Credentials, cert::CertificateValidation, http::transport::SingleNodeConnectionPool,
    http::transport::TransportBuilder, http::StatusCode, http::Url, Elasticsearch, IndexParts,
    SearchParts,
};
use serde_json::{json, Value};
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

        match res.status_code() {
            StatusCode::OK => {
                let response_body = res.json::<Value>().await?;
                let hits = response_body["hits"]["hits"]
                    .as_array()
                    .ok_or(anyhow!("res has no hits array"))?;
                let tasks = hits
                    .into_iter()
                    .map(|t| {
                        serde_json::from_value::<Task>(t["_source"].clone())
                            .expect("task not mappable")
                    })
                    .collect();
                Ok(tasks)
            }
            StatusCode::NOT_FOUND => Ok(vec![]),
            _ => {
                res.error_for_status_code_ref()?;
                unreachable!("unexpected result from elasticsearch");
            }
        }
    }

    async fn get(&self, project_id: &Uuid, task_number: usize) -> Result<Option<Task>, Error> {
        let res = self
            .search(SearchParts::Index(&[&format!("task-{}", project_id)]))
            .body(json!({
                "query": {
                    "ids": {
                        "values": [
                            task_number,
                        ]
                    }
                }
            }))
            .send()
            .await?;

        match res.status_code() {
            StatusCode::OK => {
                let response_body = res.json::<Value>().await?;
                let task = response_body["hits"]["hits"]
                    .as_array()
                    .ok_or(anyhow!("res has no hits array"))?
                    .first()
                    .and_then(|t| serde_json::from_value::<Task>(t["_source"].clone()).ok());

                return Ok(task);
            }
            StatusCode::NOT_FOUND => {
                return Ok(None);
            }
            _ => Err(anyhow!(
                "An unexpected error occured while trying to get task {:?} {:?}",
                project_id,
                task_number
            )),
        }
    }

    async fn persist(&self, project_id: &Uuid, task: &Task) -> Result<(), Error> {
        let res = self
            .index(IndexParts::IndexId(
                &format!("task-{}", project_id),
                &task.number.to_string(),
            ))
            .body(task)
            .send()
            .await?;
        res.error_for_status_code()?;
        Ok(())
    }
}
