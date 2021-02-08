use std::env;

use taskboard_core_lib::{uuid::Uuid, Project, ProjectTasks};
use tracing::{error, info, warn};
use warp::{reject, Rejection, Reply};

use crate::{
    errors::{FetchError, ValidationError},
    store::TaskStore,
};

pub async fn handle_task_list(
    store: impl TaskStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!("Finding tasks associated to project {}", project_id);

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        reject::custom(ValidationError {
            reason: format!("Invalid project id: {}", e),
        })
    })?;

    let project_name = fetch_project_name(&project_id).await.unwrap_or_else(|e| {
        error!("Could not get project name from project service: {}", e);
        warn!("Returning blank project name");
        String::from("")
    });

    let tasks = store.fetch_tasks(&project_id).await.map_err(|e| {
        error!("Unable to fetch tasks for project {}: {:?}", project_id, e);
        reject::custom(FetchError {
            reason: format!("{}", e),
        })
    })?;

    Ok(warp::reply::json(&ProjectTasks {
        project_name,
        tasks,
    }))
}

async fn fetch_project_name(project_id: &Uuid) -> Result<String, anyhow::Error> {
    let project = reqwest::get(&format!(
        "{}/{}",
        env::var("PROJECT_SERVICE_URL")?,
        project_id,
    ))
    .await?
    .json::<Project>()
    .await?;

    Ok(project.name)
}
