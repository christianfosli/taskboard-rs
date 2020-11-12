use taskboard_core_lib::{uuid::Uuid, ProjectTasks};
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
        error!("Unable to parse {} to Uuid: {:?}", project_id, e);
        reject::custom(ValidationError {
            reason: String::from("Invalid project id"),
        })
    })?;

    let project_name = String::from("Dummy Project"); // TODO: Get name of project with this id

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
