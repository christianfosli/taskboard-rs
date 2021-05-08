use taskboard_core_lib::{uuid::Uuid, ProjectTasks};
use tracing::{error, info, warn};
use warp::{reject, Rejection, Reply};

use crate::{
    errors::{FetchError, ValidationError},
    services::project_service::IProjectService,
    store::TaskStore,
};

pub async fn handle_task_list(
    store: impl TaskStore,
    project_service: impl IProjectService,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!(project = ?project_id, "finding tasks for project");

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        reject::custom(ValidationError {
            reason: format!("Invalid project id: {}", e),
        })
    })?;

    let project_name = project_service
        .get_project(&project_id)
        .await
        .map(|proj| proj.name)
        .unwrap_or_else(|e| {
            warn!(error = ?e, "unable to get project name");
            "Unknown (try again later)".to_owned()
        });

    let tasks = store.fetch_tasks(&project_id).await.map_err(|e| {
        error!(error = ?e, "failed to fetch tasks");
        reject::custom(FetchError {
            reason: format!("{}", e),
        })
    })?;

    Ok(warp::reply::json(&ProjectTasks {
        project_name,
        tasks,
    }))
}
