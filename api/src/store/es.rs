use std::env;

use anyhow::Error;
use async_trait::async_trait;
use elasticsearch::{http::transport::Transport, Elasticsearch};
use taskboard_core_lib::{uuid::Uuid, Task};

use crate::store::TaskStore;

pub fn create_client() -> Result<Elasticsearch, Error> {
    let es_url = env::var("ELASTICSEARCH_URL")?;
    let transport = Transport::single_node(&es_url)?;
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
