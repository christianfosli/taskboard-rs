use taskboard_core_lib::{uuid::Uuid, Project};
use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_increment_counter(
    store: impl ProjectStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    let project_id =
        Uuid::parse_str(&project_id).map_err(|_| reject::custom(ValidationError {}))?;

    let project = store
        .get(&project_id)
        .await
        .map_err(|e| {
            error!("Failed to get project {} from store: {}", project_id, e);
            reject::custom(IncrementCounterError {})
        })?
        .ok_or(reject::not_found())?;

    let project = Project {
        task_conter: project.task_conter + 1,
        ..project
    };

    store.persist(&project).await.map_err(|e| {
        error!("Failed to persist project: {}", e);
        reject::custom(IncrementCounterError {})
    })?;

    Ok(reply::json(&project))
}

#[derive(Clone, Debug)]
struct ValidationError {}
impl Reject for ValidationError {}

#[derive(Clone, Debug)]
struct IncrementCounterError {}
impl Reject for IncrementCounterError {}
