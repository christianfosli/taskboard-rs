use taskboard_core_lib::{uuid::Uuid, ProjectTasks};
use warp::{
    reject::{self, Reject},
    Rejection, Reply,
};

use crate::store::TaskStore;

pub async fn handle_task_list(
    store: impl TaskStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!("Finding tasks associated to project {}", project_id);

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!("Unable to parse {} to Uuid: {:?}", project_id, e);
        reject::custom(ValidationError {})
    })?;

    let project_name = String::from("Dummy Project"); // TODO: Get name of project with this id

    let tasks = store.fetch_tasks(&project_id).await.map_err(|e| {
        error!("Unable to fetch tasks for project {}: {:?}", project_id, e);
        reject::custom(FetchError {})
    })?;

    Ok(warp::reply::json(&ProjectTasks {
        project_name,
        tasks,
    }))
}

#[derive(Clone, Debug)]
struct ValidationError {}
impl Reject for ValidationError {}

#[derive(Clone, Debug)]
struct FetchError {}
impl Reject for FetchError {}
