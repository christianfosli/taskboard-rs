use reqwest::StatusCode;
use taskboard_core_lib::{commands::CompleteTaskCommand, Status, Task};
use tracing::info;
use warp::{reject, Rejection, Reply};

use crate::{
    errors::{FetchError, PersistError},
    store::TaskStore,
};

pub async fn handle_task_completed(
    store: impl TaskStore,
    command: CompleteTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!(
        project = ?command.project_id,
        task_number = ?command.task_number,
        "task completed"
    );

    let task = store
        .get(&command.project_id, command.task_number)
        .await
        .map_err(|e| {
            reject::custom(FetchError {
                reason: format!("{}", e),
            })
        })?
        .ok_or_else(reject::not_found)?;

    store
        .persist(
            &command.project_id,
            &Task {
                status: Status::Done,
                remaining_work: Some(0),
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
