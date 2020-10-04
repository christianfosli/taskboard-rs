use std::env;

use anyhow::Error;
use async_trait::async_trait;
use elasticsearch::{
    auth::Credentials, cert::CertificateValidation, http::transport::SingleNodeConnectionPool,
    http::transport::TransportBuilder, http::Url, Elasticsearch,
};
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
    async fn fetch_task(&self, _number: usize) -> Result<Task, Error> {
        todo!()
    }
    async fn fetch_tasks(&self, _project_id: &Uuid) -> Result<Vec<Task>, Error> {
        todo!()
    }
    async fn persist(&self, _task: &Task) -> Result<(), Error> {
        todo!()
    }
}
