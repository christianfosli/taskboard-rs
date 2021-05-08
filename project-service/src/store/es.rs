use std::env;

use anyhow::{anyhow, Error};
use async_trait::async_trait;
use elasticsearch::{
    auth::Credentials, cert::CertificateValidation, http::transport::SingleNodeConnectionPool,
    http::transport::TransportBuilder, http::StatusCode, http::Url, DeleteParts, Elasticsearch,
    IndexParts, SearchParts,
};
use serde_json::{json, Value};
use taskboard_core_lib::{uuid::Uuid, Project};

use super::ProjectStore;

const INDEX: &str = "project";

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
impl ProjectStore for Elasticsearch {
    async fn ping(&self) -> Result<(), Error> {
        let res = self.ping().send().await?;
        res.error_for_status_code()?;
        Ok(())
    }

    async fn get(&self, project_id: &Uuid) -> Result<Option<Project>, Error> {
        let res = self
            .search(SearchParts::Index(&[INDEX]))
            .body(json!({
                "query": {
                    "ids": {
                        "values": [
                            project_id.to_string(),
                        ]
                    }
                }
            }))
            .send()
            .await?;

        match res.status_code() {
            StatusCode::OK => {
                let response_body = res.json::<Value>().await?;
                let project = response_body["hits"]["hits"]
                    .as_array()
                    .ok_or_else(|| anyhow!("res has no hits array"))?
                    .first()
                    .and_then(|p| serde_json::from_value::<Project>(p["_source"].clone()).ok());
                return Ok(project);
            }
            StatusCode::NOT_FOUND => Ok(None),
            _ => {
                res.error_for_status_code_ref()?;
                unreachable!("unexpected result from elasticsearch");
            }
        }
    }

    async fn search(&self, name: &str) -> Result<Vec<taskboard_core_lib::Project>, Error> {
        let res = self
            .search(SearchParts::Index(&[INDEX]))
            .body(json!({
                "query": {
                    "match": {
                        "name": {
                            "query": name,
                            "fuzziness": "auto",
                            "operator": "and"
                        }
                    }
                }
            }))
            .send()
            .await?;

        match res.status_code() {
            StatusCode::OK => {
                let response_body = res.json::<Value>().await?;
                let matches = response_body["hits"]["hits"]
                    .as_array()
                    .ok_or_else(|| anyhow!("res has no hits array"))?
                    .iter()
                    .map(|p| {
                        serde_json::from_value::<Project>(p["_source"].clone())
                            .expect("project not mappable")
                    })
                    .collect();
                return Ok(matches);
            }
            StatusCode::NOT_FOUND => Ok(Vec::new()),
            _ => {
                res.error_for_status_code_ref()?;
                unreachable!("unexpected result from elasticsearch");
            }
        }
    }

    async fn persist(&self, project: &Project) -> Result<(), Error> {
        let res = self
            .index(IndexParts::IndexId(INDEX, &project.id.to_string()))
            .body(project)
            .send()
            .await?;
        res.error_for_status_code()?;
        Ok(())
    }

    async fn delete(&self, project_id: &Uuid) -> Result<(), Error> {
        let res = self
            .delete(DeleteParts::IndexId(INDEX, &project_id.to_string()))
            .send()
            .await?;
        res.error_for_status_code()?;
        Ok(())
    }
}
