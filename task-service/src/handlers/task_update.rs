use taskboard_core_lib::commands::UpdateTaskCommand;
use tracing::info;
use warp::{hyper::StatusCode, reject, Rejection, Reply};

use crate::{errors::PersistError, store::TaskStore};

pub async fn handle_task_update(
    store: impl TaskStore,
    command: UpdateTaskCommand,
) -> Result<impl Reply, Rejection> {
    info!("Update task: {:?}", command);

    store
        .persist(&command.project_id, &command.updated_task)
        .await
        .map_err(|e| {
            reject::custom(PersistError {
                reason: format!("{}", e),
            })
        })?;

    Ok(StatusCode::OK)
}
