use taskboard_core_lib::uuid::Uuid;
use tracing::{error, info};
use warp::{hyper::StatusCode, reject, Rejection, Reply};

use crate::errors::{DeleteError, ValidationError};
use crate::store::TaskStore;

/// Deletes all tasks related to a specific project
pub async fn handle_task_delete(
    store: impl TaskStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!(project = ?project_id, "deleting tasks for project");

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!(error=?e, "failed to parse project id");
        reject::custom(ValidationError {
            reason: format!("Invalid project id: {}", e),
        })
    })?;

    store.delete(&project_id).await.map_err(|e| {
        error!(error=?e, "failed to delete tasks");
        reject::custom(DeleteError {
            reason: String::from("Unable to delete tasks from store"),
        })
    })?;

    Ok(StatusCode::OK)
}
