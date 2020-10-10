use taskboard_core_lib::{commands::CreateProjectCommand, Project};
use warp::{
    hyper::StatusCode,
    reject::{self, Reject},
    reply::{self, with_status},
    Rejection, Reply,
};

use crate::store::ProjectStore;

pub async fn handle_create_project(
    store: impl ProjectStore,
    command: CreateProjectCommand,
) -> Result<impl Reply, Rejection> {
    let project = Project::new(&command.name);
    match store.persist(&project).await {
        Ok(_) => Ok(with_status(reply::json(&project), StatusCode::CREATED)),
        Err(_) => Err(reject::custom(PersistProjectError {})),
    }
}

#[derive(Clone, Debug)]
struct PersistProjectError {}
impl Reject for PersistProjectError {}
