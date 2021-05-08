use crate::errors::ValidationError;
use taskboard_core_lib::uuid::Uuid;
use tracing::{error, info};
use warp::{
    hyper::StatusCode,
    reject::{self, Reject},
    Rejection, Reply,
};

use crate::services::task_service::ITaskService;
use crate::store::ProjectStore;

pub async fn handle_delete_project(
    store: impl ProjectStore,
    task_service: impl ITaskService,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!(project = ?project_id, "deleting project");

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!(error = ?e, "failed to parse project id");
        reject::custom(DeleteProjectError::Validation(ValidationError {
            reason: format!("Project id {} is not a valid uuid", &project_id),
        }))
    })?;

    task_service.delete_tasks(&project_id).await.map_err(|e| {
        error!(error = ?e, "failed to delete project's tasks");
        reject::custom(DeleteProjectError::RemoveRelatedTasks)
    })?;

    info!("related tasks deleted");

    store.delete(&project_id).await.map_err(|e| {
        error!(error = ?e, "failed to delete project itself");
        reject::custom(DeleteProjectError::RemoveFromStore)
    })?;
    info!("project itself deleted");

    Ok(StatusCode::OK)
}

#[derive(Clone, Debug)]
enum DeleteProjectError {
    Validation(ValidationError),
    RemoveRelatedTasks,
    RemoveFromStore,
}

impl Reject for DeleteProjectError {}
