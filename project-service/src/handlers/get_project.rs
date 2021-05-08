use taskboard_core_lib::uuid::Uuid;
use tracing::{error, info};
use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::{errors::ValidationError, store::ProjectStore};

pub async fn handle_get_project(
    store: impl ProjectStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    info!(project = ?project_id, "getting project");

    let project_id = Uuid::parse_str(&project_id).map_err(|e| {
        error!(error = ?e, "failed to parse project id");
        reject::custom(GetProjectError::Validation(ValidationError {
            reason: format!("Project id {} is not a valid uuid", &project_id),
        }))
    })?;

    store
        .get(&project_id)
        .await
        .map_err(|e| {
            error!(error = ?e, "failed to get project from store");
            reject::custom(GetProjectError::FetchProject)
        })?
        .map(|project| reply::json(&project))
        .ok_or_else(reject::not_found)
}

#[derive(Clone, Debug)]
enum GetProjectError {
    Validation(ValidationError),
    FetchProject,
}

impl Reject for GetProjectError {}
