use taskboard_core_lib::uuid::Uuid;
use warp::{
    reject::{self, Reject},
    reply, Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_get_project(
    store: impl ProjectStore,
    project_id: String,
) -> Result<impl Reply, Rejection> {
    let project_id =
        Uuid::parse_str(&project_id).map_err(|e| reject::custom(ValidationError {}))?;

    match store
        .get(&project_id)
        .await
        .map_err(|_| reject::custom(GetProjectError {}))?
    {
        Some(v) => Ok(reply::json(&v)),
        None => Err(reject::not_found()),
    }
}

#[derive(Clone, Debug)]
struct ValidationError {}
impl Reject for ValidationError {}

#[derive(Clone, Debug)]
struct GetProjectError {}
impl Reject for GetProjectError {}
