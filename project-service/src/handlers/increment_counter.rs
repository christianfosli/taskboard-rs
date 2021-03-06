use taskboard_core_lib::{uuid::Uuid, Project};
use tracing::{error, info};
use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::{errors::ValidationError, store::ProjectStore};

pub async fn handle_increment_counter(
    store: impl ProjectStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!(project = ?project_id, "incrementing counter");

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!(error = ?e, "failed to parse project id");
        reject::custom(IncrementCounterError::Validation(ValidationError {
            reason: format!("Project id {} is not a valid uuid", &project_id),
        }))
    })?;

    let project = store
        .get(&project_id)
        .await
        .map_err(|e| {
            error!(error = ?e, "failed to get project from store");
            reject::custom(IncrementCounterError::GetProject)
        })?
        .ok_or_else(reject::not_found)?;

    let project = Project {
        task_conter: project.task_conter + 1,
        ..project
    };

    store.persist(&project).await.map_err(|e| {
        error!(error = ?e, "failed to persist");
        reject::custom(IncrementCounterError::SaveChanges)
    })?;

    Ok(reply::json(&project))
}

#[derive(Clone, Debug)]
enum IncrementCounterError {
    Validation(ValidationError),
    GetProject,
    SaveChanges,
}

impl Reject for IncrementCounterError {}
