use reqwest::StatusCode;
use taskboard_core_lib::commands::StartTaskCommand;
use warp::{Rejection, Reply};

use crate::store::TaskStore;

pub async fn handle_task_started(
    store: impl TaskStore,
    command: StartTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Task Started {:?}", command);

    // TODO: Get task and persist with status set to Doing

    Ok(StatusCode::OK)
}
