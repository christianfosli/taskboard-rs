use taskboard_core_lib::{commands::CreateProjectCommand, Project};
use tracing::error;
use warp::{
    hyper::StatusCode,
    reject::{self, Reject},
    reply::{self, with_status},
    Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_create_project(
    store: impl ProjectStore,
    command: CreateProjectCommand,
) -> Result<impl Reply, Rejection> {
    let project = Project::new(&command.name);

    store.persist(&project).await.map_err(|e| {
        error!("Failed to persist project: {}", e);
        reject::custom(PersistProjectError {})
    })?;

    Ok(with_status(reply::json(&project), StatusCode::CREATED))
}

#[derive(Clone, Debug)]
struct PersistProjectError {}
impl Reject for PersistProjectError {}
