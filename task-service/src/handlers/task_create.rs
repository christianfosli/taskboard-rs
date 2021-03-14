use taskboard_core_lib::{commands::CreateTaskCommand, Status, Task};
use tracing::{error, info};
use warp::{
    hyper::StatusCode,
    reject,
    reply::{self, with_status},
    Rejection, Reply,
};

use crate::store::TaskStore;
use crate::{errors::PersistError, services::project_service::IProjectService};

pub async fn handle_task_create(
    store: impl TaskStore,
    project_service: impl IProjectService,
    command: CreateTaskCommand,
) -> Result<impl Reply, Rejection> {
    let number = project_service
        .claim_task_number(&command.project_id)
        .await
        .map_err(|e| {
            error!("Unable to claim task number: {:?}", e);
            reject::custom(PersistError {
                reason: String::from("Unable to claim task number"),
            })
        })?;

    let task = Task {
        number,
        title: command.title,
        status: Status::Todo,
        remaining_work: command.estimate,
    };

    store
        .persist(&command.project_id, &task)
        .await
        .map_err(|e| {
            error!("Unable to persist task {:?}: {:?}", task, e);
            reject::custom(PersistError {
                reason: String::from("Unable to persist to store"),
            })
        })?;

    info!("Task {} created successfully", number);
    Ok(with_status(reply::json(&task), StatusCode::CREATED))
}
