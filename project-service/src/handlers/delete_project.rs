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
    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!("Failed to parse project id: {:?}", e);
        reject::custom(DeleteProjectError::ParseError)
    })?;

    task_service.delete_tasks(&project_id).await.map_err(|e| {
        error!("Failed to delete tasks for project: {:?}", e);
        reject::custom(DeleteProjectError::RemoveRelatedTasksError)
    })?;
    info!("Related tasks deleted successfully");

    store.delete(&project_id).await.map_err(|e| {
        error!("Failed to delete project id from store: {:?}", e);
        reject::custom(DeleteProjectError::RemoveFromStoreError)
    })?;
    info!("Project deleted successfully");

    Ok(StatusCode::OK)
}

#[derive(Clone, Debug)]
enum DeleteProjectError {
    ParseError,
    RemoveRelatedTasksError,
    RemoveFromStoreError,
}

impl Reject for DeleteProjectError {}
