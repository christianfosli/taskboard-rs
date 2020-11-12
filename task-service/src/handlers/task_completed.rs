use reqwest::StatusCode;
use taskboard_core_lib::{commands::CompleteTaskCommand, Status, Task};
use warp::{reject, Rejection, Reply};

use crate::{
    errors::{FetchError, PersistError},
    store::TaskStore,
};

pub async fn handle_task_completed(
    store: impl TaskStore,
    command: CompleteTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Task Completed {:?}", command);

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
