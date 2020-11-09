use reqwest::StatusCode;
use taskboard_core_lib::commands::CompleteTaskCommand;
use warp::{Rejection, Reply};

use crate::store::TaskStore;

pub async fn handle_task_completed(
    store: impl TaskStore,
    command: CompleteTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Task Completed {:?}", command);

    // TODO: Get task and persist with Completed set to true

    Ok(StatusCode::OK)
}
