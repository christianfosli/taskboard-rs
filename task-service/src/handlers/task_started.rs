use reqwest::StatusCode;
use taskboard_core_lib::{commands::StartTaskCommand, Status, Task};
use tracing::info;
use warp::{reject, Rejection, Reply};

use crate::{
    errors::{FetchError, PersistError},
    store::TaskStore,
};

pub async fn handle_task_started(
    store: impl TaskStore,
    command: StartTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Task Started {:?}", command);

    let task = store
        .get(&command.project_id, command.task_number)
        .await
        .map_err(|e| {
            reject::custom(FetchError {
                reason: format!("{}", e),
            })
        })?
        .ok_or(reject::not_found())?;

    store
        .persist(
            &command.project_id,
            &Task {
                status: Status::Doing,
                ..task
            },
        )
        .await
        .map_err(|e| {
            reject::custom(PersistError {
                reason: format!("{}", e),
            })
        })?;

    Ok(StatusCode::OK)
}
